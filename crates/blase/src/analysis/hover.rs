use crate::db::{
    def::DocumentId,
    documentation::{Documentation, HasDocs},
};
use line_index::{TextRange, TextSize};
use macros::format_to;
use type_sitter::{Node, UntypedNode};

use crate::{
    config::Config,
    db::{
        FilePosition, ParsedDocument,
        def::{Component, DefDatabase, Layout},
    },
};

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct Markup(String);

impl Markup {
    pub const HORIZONTAL_RULE: &'static str = "\n---\n";
}

impl From<Markup> for String {
    fn from(value: Markup) -> Self {
        value.0
    }
}

impl std::fmt::Display for Markup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

fn markup(rel_path: String, source_code: String, doc: Option<Documentation>) -> Markup {
    let mut buf = String::new();

    #[cfg(not(windows))]
    let path = rel_path.replace('\\', "/");
    #[cfg(windows)]
    let path = rel_path.replace('/', "\\");

    format_to!(buf, "```blade\n{}\n```", source_code);
    buf.push_str(Markup::HORIZONTAL_RULE);
    format_to!(buf, "*Project Path*: {}", path);
    if let Some(doc) = doc {
        let doc = String::from(doc);
        buf.push_str(Markup::HORIZONTAL_RULE);
        format_to!(buf, "\n{}", doc.trim());
    }
    Markup(buf)
}

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct HoverResult {
    pub(crate) markup: Markup,
    pub(crate) range: TextRange,
}

#[non_exhaustive]
enum Hoverable {
    Component(Component),
    Layout(Layout),
}

impl Hoverable {
    fn from_node(
        db: &dyn DefDatabase,
        node: UntypedNode,
        doc: &ParsedDocument,
        config: &Config,
    ) -> Option<Self> {
        ast::match_node!(node,  {
            ast::blade::TagName(tag) => {
                Component::for_tagname(db, tag, doc, config)
                    .map(Hoverable::Component)
                    .or_else(|| Layout::for_tagname(db, tag, doc, config).map(Hoverable::Layout))
            },
            _ => None,
        })
    }
}

pub fn hover(
    db: &dyn DefDatabase,
    config: &Config,
    FilePosition { path, offset }: FilePosition,
) -> Option<HoverResult> {
    let doc = &db.parsed_document(&path)?;
    let node = doc.get_node_at(offset)?;
    let hoverable = Hoverable::from_node(db, node, doc, config)?;

    match hoverable {
        Hoverable::Component(component) => {
            let rel_path = component
                .id
                .path(db)
                .strip_prefix(config.workspace_folder())
                .expect("Component is not in the workspace folder. This is a bug");
            let mut label = format!("<x-{}", component.name(db));
            if let Some(attrs) = component.attrs(db) {
                for attr in attrs {
                    label.push(' ');
                    label.push_str(attr.name.as_str());
                    format_to!(label, "=\"{}\"", attr.default_value.unwrap_or_default());
                }
            }
            label.push('>');
            let range = TextRange::new(
                TextSize::new(node.byte_range().start as u32),
                TextSize::new(node.byte_range().end as u32),
            );
            let docs = component.docs(db);
            let markup = markup(rel_path.to_string(), label, docs);
            Some(HoverResult { markup, range })
        }
        Hoverable::Layout(layout) => {
            let rel_path = layout
                .id
                .path(db)
                .strip_prefix(config.workspace_folder())
                .expect("Layout is not in the workspace folder. This is a bug");
            let mut label = format!("<x-{}>\n", layout.name(db));
            label.push_str("  {{ $slot }}\n");
            format_to!(label, "</x-{}>", layout.name(db));
            let range = TextRange::new(
                TextSize::new(node.byte_range().start as u32),
                TextSize::new(node.byte_range().end as u32),
            );
            let markup = markup(rel_path.to_string(), label, None);
            Some(HoverResult { markup, range })
        }
    }
}
