use std::sync::Arc;

use async_lsp::lsp_types::{Location, SignatureInformation};
use camino::{Utf8Path, Utf8PathBuf};
use line_index::LineCol;
use smol_str::SmolStr;
use tree_sitter::{Node, Query, QueryCursor, StreamingIterator};

use crate::{analysis::Analysis, db::ParsedDocument, lsp, server::ServerSnapshot, util::FileType};

#[derive(Debug, PartialEq)]
pub struct Component {
    name: SmolStr,
}

impl Component {
    pub fn new(name: &str) -> Self {
        let name = name.strip_prefix("x-").unwrap_or(name);
        Self {
            name: SmolStr::new(name),
        }
    }

    pub fn resources_path(&self, work: &Utf8Path) -> Utf8PathBuf {
        work.join("resources/views/components")
            .join(self.component_path() + ".blade.php")
    }

    pub fn class_path(&self, work: &Utf8Path) -> Utf8PathBuf {
        let class_path = self
            .component_path()
            .split('/')
            .map(|p| convert_case::ccase!(pascal, p))
            .collect::<Vec<_>>()
            .join("/");
        work.join("app/View/Components/").join(class_path + ".php")
    }

    fn component_path(&self) -> String {
        self.name.replace(".", "/")
    }
}

fn signature_help_for_component(
    component: &Component,
    analysis: &Analysis,
    work: &Utf8Path,
) -> Option<(SignatureInformation, Option<usize>)> {
    let min_label_capacity = "<x-".len() + component.name.len() + ">".len();
    let mut info = SignatureInformation {
        label: String::with_capacity(min_label_capacity),
        documentation: None,
        parameters: None,
        active_parameter: None,
    };
    let active_param = None;
    info.label.push_str("<x-");
    info.label.push_str(&component.name);
    let attributes = if let Some(class_doc) =
        analysis.parsed_document(&component.class_path(work)).ok()?
        && class_doc.filetype == FileType::PHP
    {
        class_component_attrs(class_doc)
    } else if let Some(res_doc) = analysis
        .parsed_document(&component.resources_path(work))
        .ok()?
        && res_doc.filetype == FileType::Blade
    {
        anon_component_attrs(res_doc)
    } else {
        Vec::new()
    };
    if attributes.is_empty() {
        info.label.push_str(" ...$attributes>");
    }
    Some((info, active_param))
}

fn class_component_attrs(class_doc: ParsedDocument) -> Vec<(Arc<str>, Option<Arc<str>>)> {
    Vec::new()
}

fn anon_component_attrs(res_doc: ParsedDocument) -> Vec<(Arc<str>, Option<Arc<str>>)> {
    Vec::new()
}

#[derive(Debug, PartialEq)]
pub struct Layout {
    name: SmolStr,
}

impl Layout {
    pub fn new(name: &str) -> Self {
        let name = name.strip_prefix("x-").unwrap_or(name);
        Self {
            name: SmolStr::new(name),
        }
    }

    pub fn resources_path(&self, work: &Utf8Path) -> Utf8PathBuf {
        self.name
            .strip_suffix("layout")
            .and_then(|s| {
                let path = if s.is_empty() {
                    work.join("resources/views/components/layout.blade.php")
                } else {
                    let layout_name = s.strip_suffix('-')?;
                    work.join(
                        &("resources/views/layouts/".to_string() + layout_name + ".blade.php"),
                    )
                };
                Some(path)
            })
            .unwrap()
    }

    pub fn class_path(&self, work: &Utf8Path) -> Utf8PathBuf {
        let layout_class_name = convert_case::ccase!(pascal, self.name.clone());
        work.join("app/View/Components/")
            .join(layout_class_name + ".php")
    }
}

#[salsa::tracked]
impl super::Analysis {
    pub fn signature_help(
        &self,
        snap: &ServerSnapshot,
        path: &Utf8Path,
        line_col: LineCol,
    ) -> Option<(SignatureInformation, Option<usize>)> {
        let source_file = self.source_file(path)?;
        let document = self.parsed_document(path).ok()??;
        let contents = &self
            .with_db(|db| Arc::clone(source_file.contents(db)))
            .ok()?;
        let line_index = self.with_db(|db| source_file.line_index(db)).ok()?;
        let offset = line_index.offset(line_col)?;
        let node = document.get_node_at(offset)?;
        match SyntaxKind::from_node(node, contents)? {
            SyntaxKind::Component(component) => {
                signature_help_for_component(&component, self, &snap.workspace_folder())
            }
            SyntaxKind::Directive(_) => todo!(),
            SyntaxKind::Layout(_) => None,
        }
    }

    pub fn goto_def(
        &self,
        snap: &ServerSnapshot,
        path: &Utf8Path,
        line_col: LineCol,
    ) -> Vec<Location> {
        let Some(source_file) = self.source_file(path) else {
            return Vec::new();
        };
        let Ok(Some(document)) = self.parsed_document(path) else {
            return Vec::new();
        };
        let Ok(contents) = &self.with_db(|db| Arc::clone(source_file.contents(db))) else {
            return Vec::new();
        };
        let Ok(line_index) = self.with_db(|db| source_file.line_index(db)) else {
            return Vec::new();
        };
        let Some(offset) = line_index.offset(line_col) else {
            return Vec::new();
        };
        let Some(node) = document.get_node_at(offset) else {
            return Vec::new();
        };
        let work = &snap.workspace_folder();
        SyntaxNode::new(node, contents)
            .map(|node| match node.kind() {
                SyntaxKind::Component(component) => {
                    vec![component.resources_path(work), component.class_path(work)]
                        .into_iter()
                        .filter_map(|path| {
                            if snap.analysis.file_exists(&path) {
                                let uri = lsp::into::url(&path);
                                Some(Location {
                                    uri,
                                    range: Default::default(),
                                })
                            } else {
                                None
                            }
                        })
                        .collect()
                }
                SyntaxKind::Layout(layout) => {
                    vec![layout.resources_path(work), layout.class_path(work)]
                        .into_iter()
                        .filter_map(|path| {
                            if snap.analysis.file_exists(&path) {
                                let uri = lsp::into::url(&path);
                                Some(Location {
                                    uri,
                                    range: Default::default(),
                                })
                            } else {
                                None
                            }
                        })
                        .collect()
                }
                // As we only support builtin directives,
                // we treat directives like keywords
                SyntaxKind::Directive(_) => Vec::new(),
            })
            .unwrap_or_default()
    }
}

#[derive(Debug, PartialEq)]
pub struct SyntaxNode<'doc> {
    node: tree_sitter::Node<'doc>,
    kind: SyntaxKind,
}

impl<'doc> SyntaxNode<'doc> {
    pub fn new(node: Node<'doc>, contents: &str) -> Option<Self> {
        Some(Self {
            node,
            kind: SyntaxKind::from_node(node, contents)?,
        })
    }

    pub fn kind(&self) -> &SyntaxKind {
        &self.kind
    }
}

#[derive(Debug, PartialEq)]
/// Kinds of nodes we are interested in
pub enum SyntaxKind {
    /// <x-component foo="bar" baz>
    Component(Component),
    /// <x-xxx-layout>
    Layout(Layout),
    /// @directive
    Directive(SmolStr),
}

impl SyntaxKind {
    pub fn from_node(node: Node, contents: &str) -> Option<Self> {
        match node.kind() {
            "tag_name" => {
                let tag_name = &contents[node.byte_range()];
                if !tag_name.starts_with("x-") {
                    return None;
                }

                let kind = if tag_name.ends_with("-layout") {
                    Self::Layout(Layout::new(tag_name))
                } else {
                    Self::Component(Component::new(tag_name))
                };

                Some(kind)
            }
            "self_closing_tag" | "start_tag" => {
                Query::new(&node.language(), "(_ (tag_name) @tag_name)")
                    .ok()
                    .and_then(|query| {
                        let mut cursor = QueryCursor::new();
                        let mut matches = cursor.matches(&query, node, contents.as_bytes());
                        while let Some(m) = matches.next() {
                            for capture in m.captures {
                                assert_eq!(
                                    query.capture_names()[capture.index as usize],
                                    "tag_name"
                                );
                                return Self::from_node(capture.node, contents);
                            }
                        }
                        None
                    })
            }
            _ if node.kind().starts_with("directive") => {
                let range = node.byte_range();
                // +1 to get rid of the '@'
                let directive_name = &contents[range.start + 1..range.end];
                Some(Self::Directive(SmolStr::new(
                    directive_name
                        .strip_prefix("end")
                        .unwrap_or(&directive_name),
                )))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use smol_str::SmolStr;

    use crate::{
        analysis::lsp::{Component, Layout, SyntaxKind},
        db::ParsedDocument,
        util::FileType,
    };
    use std::cell::RefCell;

    thread_local! {
        static PARSER: RefCell<tree_sitter::Parser> = RefCell::new(tree_sitter::Parser::new());
    }

    fn parse_document(contents: &str, filetype: FileType) -> ParsedDocument {
        let tree = PARSER.with(|parser| {
            let language = match filetype {
                FileType::Blade => tree_sitter_blade::LANGUAGE.into(),
                FileType::PHP => tree_sitter_php::LANGUAGE_PHP.into(),
            };
            let mut parser = parser.borrow_mut();
            parser.set_language(&language).expect("mismatched version");
            parser
                .parse(contents.as_bytes(), None)
                .expect("Language has been set at Server construction")
        });
        ParsedDocument { tree, filetype }
    }

    #[test]
    fn test_node_kind_directive() {
        let contents = r#"
@directive($foo)
@enddirective
"#;
        let document = parse_document(contents, FileType::Blade);
        let node = document.get_node_at(line_index::TextSize::new(3)).unwrap();
        assert_eq!(node.kind(), "directive_start");
        assert_eq!(
            Some(SyntaxKind::Directive(SmolStr::new("directive"))),
            SyntaxKind::from_node(node, contents)
        );

        let node = document.get_node_at(line_index::TextSize::new(20)).unwrap();
        assert_eq!(node.kind(), "directive_end");
        assert_eq!(
            Some(SyntaxKind::Directive(SmolStr::new("directive"))),
            SyntaxKind::from_node(node, contents)
        );
    }
    #[test]
    fn test_node_kind_component() {
        let contents = r#"
<x-foo>
    Foo </x-foo>
"#;
        let document = parse_document(contents, FileType::Blade);
        let node = document.get_node_at(line_index::TextSize::new(3)).unwrap();
        assert_eq!(node.kind(), "tag_name");
        assert_eq!(
            Some(SyntaxKind::Component(Component::new("x-foo"))),
            SyntaxKind::from_node(node, contents)
        );
        let node = document.get_node_at(line_index::TextSize::new(10)).unwrap();
        assert_eq!(node.kind(), "element");
        assert_eq!(
            Some(SyntaxKind::Component(Component::new("x-foo"))),
            SyntaxKind::from_node(node, contents)
        );
    }
    #[test]
    fn test_node_kind_layout() {
        let contents = r#"
<x-foo-layout>
    Foo </x-foo-layout>
"#;
        let document = parse_document(contents, FileType::Blade);
        let node = document.get_node_at(line_index::TextSize::new(3)).unwrap();
        assert_eq!(node.kind(), "tag_name");
        assert_eq!(
            Some(SyntaxKind::Layout(Layout::new("x-foo-layout"))),
            SyntaxKind::from_node(node, contents)
        );
        let node = document.get_node_at(line_index::TextSize::new(15)).unwrap();
        assert_eq!(node.kind(), "element");
        assert_eq!(
            Some(SyntaxKind::Layout(Layout::new("x-foo-layout"))),
            SyntaxKind::from_node(node, contents)
        );
    }
}
