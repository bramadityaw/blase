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
    pub fn horizontal_rule(is_neovim: bool) -> &'static str {
        if is_neovim { "\n---\n" } else { "\n___\n" }
    }
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

fn markup(
    rel_path: String,
    source_code: String,
    doc: Option<Documentation>,
    is_neovim: bool,
) -> Markup {
    let mut buf = String::new();
    let horizontal_rule = Markup::horizontal_rule(is_neovim);

    let path = if cfg!(windows) {
        rel_path.replace('/', "\\")
    } else {
        rel_path.replace('\\', "/")
    };

    format_to!(buf, "*Project Path*: {}", path);
    buf.push_str(horizontal_rule);
    format_to!(buf, "```blade\n{}\n```", source_code);
    if let Some(doc) = doc {
        let doc = String::from(doc);
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

    let is_neovim = config.client_is_neovim();

    match hoverable {
        Hoverable::Component(component) => {
            let rel_path = component
                .id
                .path(db)
                .strip_prefix(config.workspace_folder())
                .expect("Component is not in the workspace folder. This is a bug");
            let mut label = format!("<{}", component.name(db).tag_name());
            if let Some(attrs) = component.attrs(db) {
                for attr in attrs.as_ref() {
                    label.push(' ');
                    label.push_str(attr.name.as_str());
                    format_to!(
                        label,
                        "=\"{}\"",
                        attr.default_value.clone().unwrap_or_default()
                    );
                }
            }
            label.push('>');
            let range = TextRange::new(
                TextSize::new(node.byte_range().start as u32),
                TextSize::new(node.byte_range().end as u32),
            );
            let docs = component.docs(db);
            let markup = markup(rel_path.to_string(), label, docs, is_neovim);
            Some(HoverResult { markup, range })
        }
        Hoverable::Layout(layout) => {
            let rel_path = layout
                .id
                .path(db)
                .strip_prefix(config.workspace_folder())
                .expect("Layout is not in the workspace folder. This is a bug");
            let tag_name = &layout.name(db).tag_name();
            let mut label = format!("<{}>\n", tag_name);
            label.push_str("  {{ $slot }}\n");
            format_to!(label, "</{}>", tag_name);
            let range = TextRange::new(
                TextSize::new(node.byte_range().start as u32),
                TextSize::new(node.byte_range().end as u32),
            );
            let markup = markup(rel_path.to_string(), label, None, is_neovim);
            Some(HoverResult { markup, range })
        }
    }
}
