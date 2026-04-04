use line_index::{TextRange, TextSize};
use macros::format_to;
use type_sitter::Node;

use crate::{
    config::Config,
    db::{
        FilePosition, ParsedDocument,
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
    FilePosition { path, offset }: FilePosition,
) -> Option<SignatureHelp> {
    let document = &db.parsed_document(&path)?;
    let node = document.get_node_at(offset)?;

    let ancestors = std::iter::successors(Some(node), Node::parent);
    for ancestor in ancestors {
        ast::match_node!(ancestor, {
            ast::blade::Attribute(attr) => return signature_help_for_attr(db, attr, document, config),
            _ => ()
        })
    }

    None
}

impl Component {
    pub fn active_attr(
        &self,
        db: &dyn DefDatabase,
        attr: &ast::blade::Attribute,
        doc: &ParsedDocument,
    ) -> Option<usize> {
        use type_sitter::HasChildren;
        let attr_name = attr.children(&mut attr.walk()).filter_map(|child| {
            use ast::blade::anon_unions::Anon240808076208140814358720572997350938194::AttributeName;
            let child = child.ok()?;
            match child {
                AttributeName(attribute_name) => Some(attribute_name),
                _ => None
            }
        }).last()?;
        let attr_name = doc.text_for_node(db, attr_name)?;
        self.attrs(db)?
            .iter()
            .position(|attr| attr.name.as_str() == attr_name)
    }
}

fn signature_help_for_attr(
    db: &dyn DefDatabase,
    attr: ast::blade::Attribute,
    document: &ParsedDocument,
    config: &Config,
) -> Option<SignatureHelp> {
    let component = Component::for_attr(db, attr, document, config)?;
    let active_attr = component.active_attr(db, &attr, document);
    let mut res = SignatureHelp {
        signature: String::new(),
        active_parameter: active_attr,
        parameters: Vec::new(),
    };
    format_to!(res.signature, "<x-{}", component.name(db));
    if let Some(attrs) = component.attrs(db) {
        for attr in attrs {
            res.push_attr(attr.name.as_str(), attr.default_value.as_deref());
        }
    }
    res.signature.push('>');
    Some(res)
}
