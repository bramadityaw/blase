use ast::blade::TagName;
use camino::Utf8Path;
use line_index::{LineCol, TextRange, TextSize};
use macros::format_to;
use type_sitter::{Node, UntypedNode};

use crate::{
    config::Config,
    db::{
        ParsedDocument,
        def::{Component, DefDatabase},
    },
};

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct Markup(String);

impl From<Markup> for String {
    fn from(value: Markup) -> Self {
        value.0
    }
}

fn markup(rel_path: String, source_code: String, doc: Option<String>) -> Markup {
    let mut buf = String::new();
    format_to!(buf, "*Location*: {}\n\n", rel_path);
    format_to!(buf, "```blade\n{}\n```", source_code);
    if let Some(doc) = doc {
        format_to!(buf, "\n___\n{}", doc);
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
                    // TODO: Cover the rest of the cases
                    //.or_else(f)
            },
            _ => None,
        })
    }
}

pub fn hover(
    db: &dyn DefDatabase,
    config: &Config,
    doc_path: &Utf8Path,
    line_col: LineCol,
) -> Option<HoverResult> {
    let doc = &db.parsed_document(doc_path)?;
    let line_index = db.line_index(doc_path)?;
    let offset = line_index.offset(line_col)?;
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
            let markup = markup(rel_path.to_string(), label, None);
            Some(HoverResult { markup, range })
        }
    }
}
