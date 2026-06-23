use std::collections::HashMap;

use camino::Utf8PathBuf;
use line_index::{TextRange, TextSize};
use tree_sitter::{Query, QueryCursor, StreamingIterator};
use type_sitter::{Node, UntypedNode};

use crate::{
    config::Config,
    db::{
        DocumentDatabase, FilePosition, ParsedDocument, RootDatabase,
        def::{ComponentName, LayoutName, queries},
    },
    resolve_path,
    util::FileType,
};

pub struct ReferenceSearchResult {
    /// If Some, it stores the file paths of the document that defines the component/layout
    /// If None, it is a builtin directive
    pub defined_files: Option<Box<[Utf8PathBuf]>>,

    /// All references found, grouped by file
    pub references: HashMap<Utf8PathBuf, Vec<TextRange>>,
}

pub fn references(
    db: &RootDatabase,
    config: &Config,
    position: FilePosition,
) -> Option<Vec<ReferenceSearchResult>> {
    let path = &position.path;
    let offset = position.offset;
    let document = db.parsed_document(path)?;
    let current_node = document.get_node_at(offset)?;
    ast::match_node!(current_node, {
        ast::blade::TagName(tag_name) => handle_component_or_layout_references(db, config, tag_name, &document),
        _ => None,
    })
}

pub fn handle_component_or_layout_references(
    db: &RootDatabase,
    config: &Config,
    tag_name: ast::blade::TagName<'_>,
    document: &ParsedDocument,
) -> Option<Vec<ReferenceSearchResult>> {
    let name = document.text_for_node(db, tag_name)?;
    let (name, (class_path, resources_path)) = match ComponentName::new(name) {
        Some(name) => (
            &name.tag_name(),
            resolve_path::component_paths(&name, config),
        ),
        None => {
            let name = LayoutName::new(name)?;
            (&name.tag_name(), resolve_path::layout_paths(&name, config))
        }
    };
    let doc_paths = [class_path, resources_path]
        .into_iter()
        .filter(|path| db.parsed_document(path).is_some())
        .collect::<Box<_>>();
    let tag_name = tag_name.raw();
    let query = Query::new(
        &tag_name.language(),
        &queries::component_and_layout_named(name),
    )
    .unwrap();
    let references = db
        .all_documents()
        .into_iter()
        .filter(|doc| doc.filetype == FileType::Blade)
        .flat_map(|doc| {
            let node = doc.root_node();
            let contents = doc.contents(db);
            let mut cursor = QueryCursor::new();
            let mut matches = cursor.matches(&query, *node.raw(), contents.as_bytes());

            let mut captures = Vec::new();

            while let Some(m) = matches.next() {
                captures.push(m.captures.to_owned());
            }

            captures
                .into_iter()
                .map(|captures| (captures, doc.source.path(db)))
        })
        .map(|(captures, path)| {
            let ranges = captures
                .iter()
                .filter_map(|capture| {
                    let node = UntypedNode::new(capture.node)
                        .downcast::<ast::blade::Element>()
                        .ok()?;
                    let range = TextRange::new(
                        TextSize::new(node.byte_range().start as u32),
                        TextSize::new(node.byte_range().end as u32),
                    );
                    Some(range)
                })
                .collect::<Vec<_>>();
            (path.to_owned(), ranges)
        })
        .collect::<HashMap<_, _>>();
    Some(vec![ReferenceSearchResult {
        defined_files: Some(doc_paths),
        references,
    }])
}

#[cfg(test)]
mod tests;
