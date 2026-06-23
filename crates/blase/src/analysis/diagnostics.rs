use camino::Utf8Path;
use line_index::{TextRange, TextSize};
use tree_sitter::{Query, QueryCursor, StreamingIterator};
use type_sitter::{HasChildren, Node, UntypedNode};

use crate::{
    config::Config,
    db::{
        DocumentDatabase, FileRange, ParsedDocument, RootDatabase, Severity,
        def::{ComponentName, LayoutName, queries},
    },
    resolve_path,
    util::FileType,
};

#[cfg(test)]
mod tests;

pub struct Diagnostic {
    pub message: String,
    pub range: FileRange,
    pub severity: Severity,
}

pub fn syntax_errors(db: &RootDatabase, path: &Utf8Path) -> Vec<Diagnostic> {
    let errors = db.parse_errors(path);
    errors
        .into_iter()
        .map(|e| {
            let (range, message) = match e {
                crate::db::ParseError::Missing {
                    range,
                    error,
                    grammar_name: _,
                } => (range, error),
                crate::db::ParseError::Syntax {
                    range,
                    error,
                    affected: _,
                } => (range, error),
            };
            let text_range = TextRange::new(
                TextSize::new(range.start_byte as u32),
                TextSize::new(range.end_byte as u32),
            );
            Diagnostic {
                message,
                range: FileRange {
                    path: path.to_owned(),
                    range: text_range,
                },
                severity: Severity::Error,
            }
        })
        .collect()
}

/// Request both syntax and semantic diagnostics for the given [`Utf8Path`].
pub fn full_diagnostics(db: &RootDatabase, config: &Config, path: &Utf8Path) -> Vec<Diagnostic> {
    let mut syntax_errors = syntax_errors(db, path);
    let semantic_errors = semantic_diagnostics(db, config, path);
    syntax_errors.extend(semantic_errors);
    syntax_errors
}

pub fn semantic_diagnostics(
    db: &RootDatabase,
    config: &Config,
    path: &Utf8Path,
) -> Vec<Diagnostic> {
    let mut acc = Vec::new();
    let Some(document) = db.parsed_document(path) else {
        return acc;
    };
    if document.filetype == FileType::Blade {
        no_such_component_or_layout(db, &document, config, &mut acc);
    }
    acc
}

pub struct Element<'tree>(pub ast::blade::Element<'tree>);

impl<'tree> Element<'tree> {
    pub fn tag_name(&self) -> Option<ast::blade::TagName<'tree>> {
        let mut cursor = self.0.walk();
        let children = self.0.children(&mut cursor);
        children
            .filter_map(|child| {
                let child = child.ok()?;
                match child {
                    ast::blade::anon_unions::Anon261678758207218650704219939059354909483::SelfClosingTag(
                        self_closing,
                    ) => self_closing.tag_name().ok(),
                    ast::blade::anon_unions::Anon261678758207218650704219939059354909483::StartTag(
                        start,
                    ) => start.tag_name().ok(),
                    _ => None,
                }
            })
            .next()
    }
}

fn no_such_component_or_layout<'tree>(
    db: &RootDatabase,
    document: &ParsedDocument,
    config: &Config,
    acc: &mut Vec<Diagnostic>,
) {
    let contents = document.contents(db);
    let root = document.root_node();
    let root = root.raw();
    let mut cursor = QueryCursor::new();
    let query = Query::new(&root.language(), queries::ALL_COMPONENTS_AND_LAYOUTS).unwrap();
    let mut matches = cursor.matches(&query, *root, contents.as_bytes());
    while let Some(m) = matches.next() {
        for m in m.captures {
            let node = UntypedNode::new(m.node);
            let Ok(element) = node.downcast::<ast::blade::Element>() else {
                continue;
            };
            let Some(tag_name) = Element(element).tag_name() else {
                continue;
            };
            let name = contents.get(tag_name.byte_range());

            let range = FileRange {
                path: document.source.path(db).to_owned(),
                range: TextRange::new(
                    TextSize::new(tag_name.byte_range().start as u32),
                    TextSize::new(tag_name.byte_range().end as u32),
                ),
            };
            let severity = Severity::Error;
            if let Some(ref component_name) = name.and_then(ComponentName::new) {
                let message = format!(
                    "cannot find component `{}` in the current workspace",
                    component_name.tag_name()
                );

                let (class_path, resources_path) =
                    resolve_path::component_paths(component_name, config);

                if ![class_path, resources_path]
                    .iter()
                    .any(|path| db.parsed_document(path).is_some())
                {
                    acc.push(Diagnostic {
                        message,
                        range,
                        severity,
                    });
                }
                continue;
            }

            if let Some(ref layout_name) = name.and_then(LayoutName::new) {
                let message = format!(
                    "cannot find layout `{}` in the current workspace",
                    layout_name.tag_name()
                );

                let (class_path, resources_path) = resolve_path::layout_paths(layout_name, config);
                if ![class_path, resources_path]
                    .iter()
                    .any(|path| db.parsed_document(path).is_some())
                {
                    acc.push(Diagnostic {
                        message,
                        range,
                        severity,
                    });
                }
                continue;
            }
        }
    }
}
