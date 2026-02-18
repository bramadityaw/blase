use std::{
    cell::RefCell,
    fmt::Debug,
    sync::{Arc, LazyLock},
};

use async_lsp::lsp_types;
use dashmap::DashMap;
use salsa::{Database as Db, Setter};
use smol_str::SmolStr;

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
    pub contents: Arc<str>,
}

#[derive(Debug, Default)]
pub struct Files {
    files: Arc<DashMap<lsp_types::Url, SourceFile>>,
}

impl Files {
    pub fn source_file(&self, url: &lsp_types::Url) -> SourceFile {
        match self.files.get(url) {
            Some(file) => *file,
            None => {
                panic!("Unable to fetch source file for `Url`: {url:?}; this is a bug")
            }
        }
    }

    pub fn set_source_file(&self, db: &mut dyn Db, url: lsp_types::Url, contents: &str) {
        match self.files.entry(url) {
            dashmap::Entry::Occupied(mut occupied) => {
                occupied.get_mut().set_contents(db).to(Arc::from(contents));
            }
            dashmap::Entry::Vacant(vacant) => {
                let contents = SourceFile::new(db, Arc::from(contents));
                vacant.insert(contents);
            }
        }
    }
}

impl RootDatabase {
    pub fn source_file(&self, url: &lsp_types::Url) -> SourceFile {
        let files = Arc::clone(&self.files);
        files.source_file(url)
    }

    pub fn set_source_file(&mut self, url: lsp_types::Url, contents: &str) {
        let files = Arc::clone(&self.files);
        files.set_source_file(self, url, contents);
    }
}

#[salsa::interned(debug)]
pub struct ComponentId<'db> {
    #[returns(ref)]
    name: SmolStr,
}

#[derive(Clone)]
pub struct BladeDocument {
    pub tree: tree_sitter::Tree,
}

impl Debug for BladeDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BladeDocument")
            .field("tree", &self.tree.root_node().to_sexp())
            .finish()
    }
}

impl PartialEq for BladeDocument {
    fn eq(&self, other: &Self) -> bool {
        self.tree.root_node().to_sexp() == other.tree.root_node().to_sexp()
    }
}

#[test]
fn test_document_eq() {
    let contents = r#"
@if($foo)
  {{ $bar }}
@endif
"#;

    let (doc1, doc2) = RootDatabase::default().attach(|db| {
        let source1 = SourceFile::new(db, Arc::from(contents));
        let doc1 = parse_document(db, source1);

        let source2 = SourceFile::new(db, Arc::from(contents));
        let doc2 = parse_document(db, source2);
        (doc1, doc2)
    });
    assert_eq!(doc1, doc2);
}

#[test]
fn test_document_eq_diff_ws_contents() {
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
        let source1 = SourceFile::new(db, Arc::from(contents1));
        let doc1 = parse_document(db, source1);

        let source2 = SourceFile::new(db, Arc::from(contents2));
        let doc2 = parse_document(db, source2);
        (doc1, doc2)
    });
    assert_ne!(contents1, contents2);
    assert_eq!(doc1, doc2);
}

impl Eq for BladeDocument {}

thread_local! {
    static PARSER: RefCell<LazyLock<tree_sitter::Parser>> = RefCell::new(LazyLock::new(|| {
        use tree_sitter::Parser;

        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_blade::LANGUAGE.into())
            .expect("msg");
        parser
    }));
}

#[salsa::tracked]
pub fn parse_document(db: &dyn Db, source: SourceFile) -> BladeDocument {
    let contents = source.contents(db);

    let tree = PARSER.with(|parser| {
        parser
            .borrow_mut()
            .parse(contents.as_bytes(), None)
            .expect("Language has been set at Server construction")
    });

    BladeDocument { tree }
}
