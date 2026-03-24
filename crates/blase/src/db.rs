use std::{
    fmt::Debug,
    sync::{Arc, LazyLock, RwLock},
};

use async_lsp::lsp_types;
use camino::{Utf8Path, Utf8PathBuf};
use dashmap::DashMap;
use line_index::LineIndex;
use salsa::{Accumulator, Database, Setter};
use type_sitter::UntypedNode;

use crate::{lsp, util::FileType};

pub mod def;

#[salsa::db]
#[derive(Clone, Default)]
pub struct RootDatabase {
    storage: salsa::Storage<Self>,
    files: Arc<Files>,
}

#[salsa::db]
impl salsa::Database for RootDatabase {}

#[salsa::input(debug)]
pub struct SourceFile {
    #[returns(ref)]
    pub path: Utf8PathBuf,
    #[returns(ref)]
    pub contents: Arc<str>,
    pub file_type: FileType,
    pub line_index: LineIndex,
}

#[derive(Debug, Default)]
pub struct Files {
    files: Arc<DashMap<Utf8PathBuf, SourceFile>>,
}

impl Files {
    pub fn source_file(&self, path: &Utf8Path) -> Option<SourceFile> {
        self.files.get(path).map(|x| *x)
    }

    pub fn len(&self) -> usize {
        self.files.len()
    }

    pub fn set_source_file(&self, db: &mut dyn Database, path: Utf8PathBuf, contents: &str) {
        match self.files.entry(path.clone()) {
            dashmap::Entry::Occupied(mut occupied) => {
                let source_file = occupied.get_mut();
                source_file.set_contents(db).to(Arc::from(contents));
                source_file.set_line_index(db).to(LineIndex::new(contents));
            }
            dashmap::Entry::Vacant(vacant) => {
                if let Some(ty) = FileType::from_path(&path) {
                    let contents = SourceFile::new(
                        db,
                        path,
                        Arc::from(contents),
                        ty,
                        LineIndex::new(contents),
                    );
                    vacant.insert(contents);
                } else {
                    tracing::error!(url = path.as_str(), "Unknown filetype");
                }
            }
        }
    }
}

impl RootDatabase {
    pub fn source_file(&self, path: &Utf8Path) -> Option<SourceFile> {
        let files = Arc::clone(&self.files);
        files.source_file(path)
    }

    pub fn files_count(&self) -> usize {
        self.files.len()
    }

    pub fn set_source_file(&mut self, path: Utf8PathBuf, contents: &str) {
        let files = Arc::clone(&self.files);
        files.set_source_file(self, path, contents);
    }
}

#[salsa::db]
pub trait SourceDatabase: salsa::Database + 'static {
    fn source_file(&self, path: &Utf8Path) -> Option<SourceFile>;

    fn contents(&self, path: &Utf8Path) -> Option<Arc<str>> {
        self.source_file(path)
            .map(|file| Arc::clone(file.contents(self)))
    }

    fn file_type(&self, path: &Utf8Path) -> Option<FileType> {
        self.source_file(path).map(|file| file.file_type(self))
    }

    fn line_index(&self, path: &Utf8Path) -> Option<LineIndex> {
        self.source_file(path).map(|file| file.line_index(self))
    }
}

#[salsa::db]
impl SourceDatabase for RootDatabase {
    fn source_file(&self, path: &Utf8Path) -> Option<SourceFile> {
        self.source_file(path)
    }
}

#[salsa::db]
pub trait DocumentDatabase: SourceDatabase + salsa::Database {
    fn parsed_document(&self, path: &Utf8Path) -> Option<ParsedDocument>;
    fn parse_errors(&self, path: &Utf8Path) -> Vec<ParseError>;
}

#[salsa::db]
#[salsa::tracked]
impl DocumentDatabase for RootDatabase {
    fn parsed_document(&self, path: &Utf8Path) -> Option<ParsedDocument> {
        self.source_file(path)
            .map(|file| parse_document(self, file))
    }
    fn parse_errors(&self, path: &Utf8Path) -> Vec<ParseError> {
        match self.source_file(path) {
            Some(file) => parse_document::accumulated::<ParseErrorAccumulator>(self, file)
                .into_iter()
                .map(|x| x.0.clone())
                .collect(),
            None => Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct ParsedDocument {
    pub source: SourceFile,
    pub tree: tree_sitter::Tree,
    pub filetype: FileType,
}

impl ParsedDocument {
    pub fn root_node<'doc>(&'doc self) -> UntypedNode<'doc> {
        let root_node = self.tree.root_node();
        UntypedNode::new(root_node)
    }

    pub fn get_node_at<'doc>(
        &'doc self,
        text_size: line_index::TextSize,
    ) -> Option<UntypedNode<'doc>> {
        let offset: usize = text_size.into();
        self.tree
            .root_node()
            .descendant_for_byte_range(offset, offset)
            .map(|node| UntypedNode::new(node))
    }

    pub fn text_for_node<'db>(
        &'db self,
        db: &'db dyn DocumentDatabase,
        node: impl type_sitter::Node<'db>,
    ) -> Option<&'db str> {
        let contents = self.source.contents(db);
        contents.get(node.byte_range())
    }
}

impl Debug for ParsedDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BladeDocument")
            .field("tree", &self.tree.root_node().to_sexp())
            .finish()
    }
}

impl PartialEq for ParsedDocument {
    fn eq(&self, other: &Self) -> bool {
        self.filetype == other.filetype
            && self.tree.root_node().to_sexp() == other.tree.root_node().to_sexp()
    }
}

#[test]
fn test_document_eq() {
    use std::path::PathBuf;
    let path = PathBuf::from("C:\\foo\\bar\\baz.blade.php");
    let path = Utf8PathBuf::from_path_buf(path).unwrap();
    let contents = r#"
@if($foo)
  {{ $bar }}
@endif
"#;

    let (doc1, doc2) = RootDatabase::default().attach(|db| {
        let source1 = SourceFile::new(
            db,
            path.clone(),
            Arc::from(contents),
            FileType::Blade,
            LineIndex::new(contents),
        );
        let doc1 = parse_document(db, source1);

        let source2 = SourceFile::new(
            db,
            path,
            Arc::from(contents),
            FileType::Blade,
            LineIndex::new(contents),
        );
        let doc2 = parse_document(db, source2);
        (doc1, doc2)
    });
    assert_eq!(doc1, doc2);
}

#[test]
fn test_document_eq_diff_ws_contents() {
    use std::path::PathBuf;
    let path = PathBuf::from("C:\\foo\\bar\\baz.blade.php");
    let path = Utf8PathBuf::from_path_buf(path).unwrap();
    let contents1 = r#"
@if($foo)
  {{ $bar }}
@endif
"#;

    let contents2 = r#"

@if($foo)
  {{ $bar }}
@endif
"#;

    let (doc1, doc2) = RootDatabase::default().attach(|db| {
        let source1 = SourceFile::new(
            db,
            path.clone(),
            Arc::from(contents1),
            FileType::Blade,
            LineIndex::new(contents1),
        );
        let doc1 = parse_document(db, source1);

        let source2 = SourceFile::new(
            db,
            path.clone(),
            Arc::from(contents2),
            FileType::Blade,
            LineIndex::new(contents2),
        );
        let doc2 = parse_document(db, source2);
        (doc1, doc2)
    });
    assert_ne!(contents1, contents2);
    assert_eq!(doc1, doc2);
}

impl Eq for ParsedDocument {}

#[derive(Clone)]
pub enum ParseError {
    Missing {
        range: tree_sitter::Range,
        error: String,
        // Missing node's symbol name as it appears in the grammar ignoring aliases as a string
        grammar_name: &'static str,
    },
    Syntax {
        range: tree_sitter::Range,
        error: String,
        affected: String,
    },
}

impl From<ParseError> for lsp_types::Diagnostic {
    fn from(error: ParseError) -> Self {
        (&error).into()
    }
}

impl From<&ParseError> for lsp_types::Diagnostic {
    fn from(error: &ParseError) -> Self {
        let (range, message) = match error {
            ParseError::Missing { range, error, .. } | ParseError::Syntax { range, error, .. } => {
                (lsp::from::range(*range), error.clone())
            }
        };
        lsp_types::Diagnostic {
            range: range.into(),
            severity: Some(lsp_types::DiagnosticSeverity::ERROR),
            message,
            code: Some(lsp_types::NumberOrString::String("BLASE".into())),
            ..Default::default()
        }
    }
}

#[salsa::accumulator]
pub struct ParseErrorAccumulator(pub ParseError);

static PARSER: LazyLock<RwLock<tree_sitter::Parser>> =
    LazyLock::new(|| RwLock::new(tree_sitter::Parser::new()));

#[salsa::tracked]
pub fn parse_document(db: &dyn DocumentDatabase, source: SourceFile) -> ParsedDocument {
    let contents = source.contents(db);
    let filetype = source.file_type(db);

    let tree = {
        let language = match filetype {
            FileType::Blade => tree_sitter_blade::LANGUAGE.into(),
            FileType::PHP => tree_sitter_php::LANGUAGE_PHP.into(),
        };
        let mut parser = PARSER.write().expect("poison");
        parser.set_language(&language).expect("mismatched version");
        parser
            .parse(contents.as_bytes(), None)
            .expect("Language has been set")
    };

    get_tree_sitter_errors(db, tree.root_node(), contents);

    ParsedDocument {
        source,
        tree,
        filetype,
    }
}

/// Vendored from https://github.com/adclz/auto-lsp/blob/d133723bfbd9150c0ec944b4e9f9cf96844dc167/crates/default/src/db/lexer.rs#L30
///
/// Traverse a tree-sitter syntax tree to collect error nodes.
///
/// This function traverses the syntax tree in a depth-first manner to find error nodes:
/// - If a node `has_error()` but none of its children have errors, it is collected
/// - If a node `has_error()` and some children have errors, traverse those children
fn get_tree_sitter_errors(db: &dyn DocumentDatabase, node: tree_sitter::Node, source_code: &str) {
    let mut cursor = node.walk();

    if node.has_error() {
        if node.children(&mut cursor).any(|f| f.has_error()) {
            for child in node.children(&mut cursor) {
                get_tree_sitter_errors(db, child, source_code);
            }
        } else {
            ParseErrorAccumulator(format_error(node, source_code)).accumulate(db);
        }
    }
}

/// Vendored from https://github.com/adclz/auto-lsp/blob/d133723bfbd9150c0ec944b4e9f9cf96844dc167/crates/default/src/db/lexer.rs#L44
fn format_error(node: tree_sitter::Node, source_code: &str) -> ParseError {
    if node.is_missing() {
        ParseError::Missing {
            range: node.range(),
            error: format!("Syntax error: Missing '{}'", node.grammar_name()),
            grammar_name: node.grammar_name(),
        }
    } else {
        let children_text: Vec<String> = (0..node.child_count())
            .filter_map(|i| {
                let child = node.child(i)?;
                Some(source_code[child.byte_range()].to_string())
            })
            .collect();
        ParseError::Syntax {
            range: node.range(),
            error: format!("Unexpected token(s): '{}'", children_text.join(" ")),
            affected: children_text.join(" "),
        }
    }
}
