use camino::Utf8Path;
use line_index::{LineCol, TextRange, TextSize};
use macros::format_to;
use type_sitter::Node;

use crate::{
    config::Config,
    db::{
        ParsedDocument,
        def::{Component, DefDatabase},
    },
};

#[derive(Debug)]
pub struct SignatureHelp {
    pub signature: String,
    pub active_parameter: Option<usize>,
    parameters: Vec<TextRange>,
}

impl SignatureHelp {
    pub fn parameter_labels(&self) -> impl Iterator<Item = &str> + '_ {
        self.parameters.iter().map(move |&it| &self.signature[it])
    }

    pub fn parameter_ranges(&self) -> &[TextRange] {
        &self.parameters
    }

    fn push_attr(&mut self, name: &str, default: Option<&str>) {
        self.signature.push(' ');
        let start = TextSize::of(&self.signature);
        self.signature.push_str(name);
        format_to!(self.signature, "=\"{}\"", default.unwrap_or_default());
        let end = TextSize::of(&self.signature);
        self.parameters.push(TextRange::new(start, end))
    }
}

pub fn signature_help(
    db: &dyn DefDatabase,
    config: &Config,
    doc_path: &Utf8Path,
    line_col: LineCol,
) -> Option<SignatureHelp> {
    let document = &db.parsed_document(doc_path)?;
    let line_index = db.line_index(doc_path)?;
    let offset = line_index.offset(line_col)?;
    let node = document.get_node_at(offset)?;

    let ancestors = std::iter::successors(Some(node), Node::parent);
    for ancestor in ancestors {
        ast::match_node!(ancestor, {
            ast::blade::Attribute(attr) => return signature_help_for_attr(db, attr, document, offset, config),
            _ => ()
        })
    }

    None
}

fn signature_help_for_attr(
    db: &dyn DefDatabase,
    attr: ast::blade::Attribute,
    document: &ParsedDocument,
    offset: TextSize,
    config: &Config,
) -> Option<SignatureHelp> {
    let (component, active_parameter) = Component::for_attr(db, attr, document, offset, config)?;
    let mut res = SignatureHelp {
        signature: String::new(),
        active_parameter,
        parameters: Vec::new(),
    };
    format_to!(res.signature, "<{}", component.name(db));
    if let Some(attrs) = component.attrs(db) {
        for attr in attrs {
            res.push_attr(attr.name.as_str(), attr.default_value.as_deref());
        }
    }
    res.signature.push('>');
    Some(res)
}

fn get_tag_attrs<'tree, Tag: Node<'tree>>(tag: Tag) -> Option<Vec<ast::blade::Attribute<'tree>>> {
    ast::match_node!(tag, {
        ast::blade::SelfClosingTag(self_tag) => {
            let mut cursor = self_tag.walk();
            Some(self_tag.attributes(&mut cursor).filter_map(Result::ok).collect())
        },
        ast::blade::StartTag(start_tag) => {
            let mut cursor = start_tag.walk();
            Some(start_tag.attributes(&mut cursor).filter_map(Result::ok).collect())
        },
        _ => None,
    })
}
