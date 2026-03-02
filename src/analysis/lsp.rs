use async_lsp::lsp_types::{Location, Position, Url};
use camino::Utf8Path;
use smol_str::SmolStr;
use tree_sitter::{Node, Query, QueryCursor, StreamingIterator};

use crate::{lsp, server::ServerSnapshot};

impl super::Analysis {
    pub fn goto_def(
        &self,
        snap: &ServerSnapshot,
        path: &Utf8Path,
        position: Position,
    ) -> Vec<Location> {
        let document = self.parsed_document(path);
        let contents = &self.file_contents(path);
        let line_index = self.line_index(path);
        let line_col = lsp::from::line_col(position);
        let Some(offset) = line_index.offset(line_col) else {
            return Vec::new();
        };
        let Some(node) = document.get_node_at(offset) else {
            return Vec::new();
        };
        NodeKind::from_node(&node, contents)
            .map(|kind| match kind {
                NodeKind::Component(name) => name
                    .strip_prefix("x-")
                    .and_then(|name| {
                        let component_path = name.replace(".", "/");
                        let mut locations = Vec::new();
                        let work = snap.workspace_folder();
                        let view_url = work
                            .join("resources/views/components")
                            .join(component_path.clone() + ".blade.php");
                        tracing::debug!(?view_url);
                        if let Ok(true) = std::fs::exists(&view_url) {
                            locations.push(Location {
                                uri: Url::from_file_path(&view_url).ok()?,
                                range: Default::default(),
                            });
                        }
                        let class_path = component_path
                            .split('/')
                            .map(|p| convert_case::ccase!(pascal, p))
                            .collect::<Vec<_>>()
                            .join("/");
                        let class_url = work.join("app/View/Components/").join(class_path + ".php");
                        tracing::debug!(?class_url);
                        if let Ok(true) = std::fs::exists(&class_url) {
                            locations.push(Location {
                                uri: Url::from_file_path(&class_url).ok()?,
                                range: Default::default(),
                            });
                        }

                        Some(locations)
                    })
                    .unwrap_or_default(),
                NodeKind::Layout(name) => name
                    .strip_prefix("x-")
                    .and_then(|s| s.strip_suffix("layout"))
                    .and_then(|s| {
                        let work = snap.workspace_folder();
                        tracing::debug!(?work);
                        let mut locations = Vec::new();
                        let layout_path = if s.is_empty() {
                            work.join("resources/views/components/layout.blade.php")
                        } else {
                            let layout_name = s.strip_suffix('-')?;
                            work.join(
                                &("resources/views/layouts/".to_string()
                                    + layout_name
                                    + ".blade.php"),
                            )
                        };
                        tracing::debug!(url = ?layout_path);
                        if let Ok(true) = std::fs::exists(&layout_path) {
                            locations.push(Location {
                                uri: Url::from_file_path(layout_path).ok()?,
                                range: Default::default(),
                            });
                        };

                        if !s.is_empty() {
                            let layout_class_name = convert_case::ccase!(
                                pascal,
                                s.strip_suffix('-')?.to_string() + "-layout"
                            );
                            let layout_class_path = work
                                .join("app/View/Components/")
                                .join(layout_class_name + ".php");
                            if let Ok(true) = std::fs::exists(&layout_class_path) {
                                locations.push(Location {
                                    uri: Url::from_file_path(layout_class_path).ok()?,
                                    range: Default::default(),
                                });
                            }
                        }
                        Some(locations)
                    })
                    .unwrap_or_default(),
                // As we only support builtin directives,
                // we treat directives like keywords
                NodeKind::Directive(_) => Vec::new(),
            })
            .unwrap_or_default()
    }
}

#[derive(Debug, PartialEq)]
/// Kinds of nodes we are interested in
pub enum NodeKind {
    /// <x-component foo="bar" baz>
    Component(SmolStr),
    /// <x-xxx-layout>
    Layout(SmolStr),
    /// @directive
    Directive(SmolStr),
}

impl NodeKind {
    pub fn from_node(node: &Node, contents: &str) -> Option<Self> {
        match node.kind() {
            "tag_name" => {
                let tag_name = &contents[node.byte_range()];
                if !tag_name.starts_with("x-") {
                    return None;
                }

                let kind = if tag_name.ends_with("-layout") {
                    Self::Layout(SmolStr::new(tag_name))
                } else {
                    Self::Component(SmolStr::new(tag_name))
                };

                Some(kind)
            }
            "element" => Query::new(&node.language(), "(_ (tag_name) @tag_name)")
                .ok()
                .and_then(|query| {
                    let mut cursor = QueryCursor::new();
                    let mut matches = cursor.matches(&query, *node, contents.as_bytes());
                    while let Some(m) = matches.next() {
                        for capture in m.captures {
                            assert_eq!(query.capture_names()[capture.index as usize], "tag_name");
                            return Self::from_node(&capture.node, contents);
                        }
                    }
                    None
                }),
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

    use crate::{analysis::lsp::NodeKind, db::ParsedDocument, util::FileType};
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
            Some(NodeKind::Directive(SmolStr::new("directive"))),
            NodeKind::from_node(&node, contents)
        );

        let node = document.get_node_at(line_index::TextSize::new(20)).unwrap();
        assert_eq!(node.kind(), "directive_end");
        assert_eq!(
            Some(NodeKind::Directive(SmolStr::new("directive"))),
            NodeKind::from_node(&node, contents)
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
            Some(NodeKind::Component(SmolStr::new("x-foo"))),
            NodeKind::from_node(&node, contents)
        );
        let node = document.get_node_at(line_index::TextSize::new(10)).unwrap();
        assert_eq!(node.kind(), "element");
        assert_eq!(
            Some(NodeKind::Component(SmolStr::new("x-foo"))),
            NodeKind::from_node(&node, contents)
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
            Some(NodeKind::Layout(SmolStr::new("x-foo-layout"))),
            NodeKind::from_node(&node, contents)
        );
        let node = document.get_node_at(line_index::TextSize::new(15)).unwrap();
        assert_eq!(node.kind(), "element");
        assert_eq!(
            Some(NodeKind::Layout(SmolStr::new("x-foo-layout"))),
            NodeKind::from_node(&node, contents)
        );
    }
}
