use type_sitter::{HasChild, Node, UntypedNode};

use crate::{
    config::Config,
    db::{
        DocumentDatabase, FilePosition, FileRange,
        def::{ComponentName, LayoutName},
    },
    resolve_path,
};

#[cfg(test)]
mod tests;

pub fn goto_definition(
    db: &dyn DocumentDatabase,
    config: &Config,
    FilePosition { path, offset }: FilePosition,
) -> Option<Vec<FileRange>> {
    let document = db.parsed_document(&path)?;
    let contents = &db.contents(&path)?;
    let node = document.get_node_at(offset)?;
    tracing::debug!(node = node.kind(), path = path.as_str());
    goto_def(db, contents, config, node)
}

fn goto_def<'tree, N: Node<'tree>>(
    db: &dyn DocumentDatabase,
    contents: &str,
    work_path: &Config,
    node: N,
) -> Option<Vec<FileRange>> {
    fn inner(
        db: &dyn DocumentDatabase,
        contents: &str,
        config: &Config,
        node: UntypedNode,
    ) -> Option<Vec<FileRange>> {
        ast::match_node!(node, {
            ast::blade::TagName(tag_name) => goto_def_for_component(db, config, tag_name, contents),
            ast::blade::StartTag(start_tag) => {
                goto_def_for_component(db, config, start_tag.tag_name().ok()?, contents)
            },
            ast::blade::EndTag(end_tag) => {
                let tag_name = end_tag.child().ok()?;
                goto_def_for_component(db, config, tag_name, contents)
            },
            ast::blade::SelfClosingTag(self_tag) => {
                goto_def_for_component(db, config, self_tag.tag_name().ok()?, contents)
            },
            _ => {
                tracing::error!(node=node.kind(), "No component found");
                None
            },
        })
    }
    let node = node.upcast();
    inner(db, contents, work_path, node)
}

#[tracing::instrument(skip(db, config))]
fn goto_def_for_component(
    db: &dyn DocumentDatabase,
    config: &Config,
    tag_name: ast::blade::TagName,
    contents: &str,
) -> Option<Vec<FileRange>> {
    let name = contents.get(tag_name.byte_range())?;
    let (class_path, resources_path) = if let Some(layout) = LayoutName::new(name) {
        resolve_path::layout_paths(layout, config)
    } else {
        resolve_path::component_paths(&ComponentName::new(name)?, config)
    };
    tracing::debug!(
        class_path = class_path.as_str(),
        resources_path = resources_path.as_str()
    );
    let ranges = vec![class_path, resources_path]
        .into_iter()
        .filter_map(|path| {
            if db.parsed_document(&path).is_some() {
                let range = FileRange {
                    path,
                    range: Default::default(),
                };
                Some(range)
            } else {
                None
            }
        })
        .collect();
    Some(ranges)
}
