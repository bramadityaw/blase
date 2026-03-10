use async_lsp::lsp_types;
use camino::{Utf8Path, Utf8PathBuf};
use line_index::LineCol;
use type_sitter::{HasChild, HasChildren, Node, UntypedNode};

use crate::{db::DocumentDatabase, lsp};

#[tracing::instrument(skip(db))]
pub fn goto_definition(
    db: &dyn DocumentDatabase,
    doc_path: &Utf8Path,
    work_path: &Utf8Path,
    line_col: LineCol,
) -> Option<Vec<lsp_types::Location>> {
    let document = db.parsed_document(doc_path)?;
    let contents = &db.contents(doc_path)?;
    let line_index = db.line_index(doc_path)?;
    let node = document.get_node_at(line_index.offset(line_col)?)?;
    tracing::debug!(node = node.kind(), path = doc_path.as_str());
    goto_def(db, contents, work_path, node)
}

fn goto_def<'tree, N: Node<'tree>>(
    db: &dyn DocumentDatabase,
    contents: &str,
    work_path: &Utf8Path,
    node: N,
) -> Option<Vec<lsp_types::Location>> {
    fn inner(
        db: &dyn DocumentDatabase,
        contents: &str,
        work_path: &Utf8Path,
        node: UntypedNode,
    ) -> Option<Vec<lsp_types::Location>> {
        ast::match_node!(node, {
            ast::TagName(tag_name) => goto_def_for_tagname(db, tag_name, contents, work_path),
            ast::StartTag(start_tag) => {
                use ast::anon_unions::Attribute_TagName;
                let mut cursor = start_tag.walk();
                for child in start_tag.children(&mut cursor) {
                    match child.ok()? {
                        Attribute_TagName::TagName(tag_name) => {
                            return goto_def_for_tagname(db, tag_name, contents, work_path);
                        }
                        _ => continue,
                    }
                }
                None
            },
            ast::EndTag(end_tag) => {
                let tag_name = end_tag.child().ok()?;
                return goto_def_for_tagname(db, tag_name, contents, work_path);
            },
            ast::SelfClosingTag(self_tag) => {
                use ast::anon_unions::Attribute_TagName;
                let mut cursor = self_tag.walk();
                for child in self_tag.children(&mut cursor) {
                    match child.ok()? {
                        Attribute_TagName::TagName(tag_name) => {
                            return goto_def_for_tagname(db, tag_name, contents, work_path);
                        }
                        _ => continue,
                    }
                }
                None
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

#[tracing::instrument(skip(db))]
fn goto_def_for_tagname(
    db: &dyn DocumentDatabase,
    tag_name: ast::TagName,
    contents: &str,
    work_path: &Utf8Path,
) -> Option<Vec<lsp_types::Location>> {
    let name = contents.get(tag_name.byte_range())?.strip_prefix("x-")?;
    let (class_path, resources_path) = if name.ends_with("layout") {
        layout_paths(name.strip_suffix("layout").unwrap(), work_path)
    } else {
        component_paths(name, work_path)
    };
    tracing::debug!(
        class_path = class_path.as_str(),
        resources_path = resources_path.as_str()
    );
    let paths = vec![class_path, resources_path]
        .into_iter()
        .filter_map(|path| {
            if db.parsed_document(&path).is_some() {
                let uri = lsp::into::url(&path);
                Some(lsp_types::Location {
                    uri,
                    range: Default::default(),
                })
            } else {
                None
            }
        })
        .collect();
    Some(paths)
}

fn layout_paths(name: &str, work_path: &Utf8Path) -> (Utf8PathBuf, Utf8PathBuf) {
    let class_path = layout_class_path(name, work_path);
    let resources_path = layout_resources_path(name, work_path);
    (class_path, resources_path)
}

fn layout_class_path(name: &str, work_path: &Utf8Path) -> Utf8PathBuf {
    let name = format!("{}-layout", name);
    let layout_class_name = convert_case::ccase!(pascal, name);
    work_path
        .join(component_class_dir())
        .join(layout_class_name + ".php")
}

fn layout_resources_path(name: &str, work_path: &Utf8Path) -> Utf8PathBuf {
    let path = if name.is_empty() {
        work_path
            .join(component_views_dir())
            .join("layout.blade.php")
    } else {
        let layout_name = name.strip_suffix('-').unwrap();
        let path = Utf8PathBuf::from(views_dir())
            .join("layouts")
            .join(layout_name.to_string() + ".blade.php");
        work_path.join(path)
    };
    path
}

fn component_paths(name: &str, work_path: &Utf8Path) -> (Utf8PathBuf, Utf8PathBuf) {
    let name = name.replace('.', std::path::MAIN_SEPARATOR_STR);
    let class_path = component_class_path(name.clone(), work_path);
    let resources_path = component_resources_path(name.clone(), work_path);
    (class_path, resources_path)
}

fn component_resources_path(path: String, work_path: &Utf8Path) -> Utf8PathBuf {
    work_path
        .join(component_views_dir())
        .join(path + ".blade.php")
}

fn component_views_dir() -> String {
    views_dir() + std::path::MAIN_SEPARATOR_STR + "components"
}
fn views_dir() -> String {
    "resources/views".to_string()
}
fn component_class_dir() -> String {
    "app/View/Components".to_string()
}

fn component_class_path(path: String, work_path: &Utf8Path) -> Utf8PathBuf {
    let class_path = path
        .split(std::path::MAIN_SEPARATOR_STR)
        .map(|p| convert_case::ccase!(pascal, p))
        .collect::<Vec<_>>()
        .join(std::path::MAIN_SEPARATOR_STR);
    work_path
        .join(component_class_dir())
        .join(class_path + ".php")
}
