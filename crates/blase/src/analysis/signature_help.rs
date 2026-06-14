use line_index::{TextRange, TextSize};
use macros::format_to;
use type_sitter::{HasChildren, Node};

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
        let attr_name = match attr {
            ast::blade::Attribute::ExpressionAttribute(expression_attribute) => expression_attribute.children(&mut expression_attribute.walk()).filter_map(|ch| {
                let ch = ch.ok()?;
                match ch {
                    ast::blade::anon_unions::ExpressionAttributeName_QuotedExpression::ExpressionAttributeName(attribute_name) => Some(attribute_name.upcast()),
                    ast::blade::anon_unions::ExpressionAttributeName_QuotedExpression::QuotedExpression(_) => None,
                }
            }).last(),
            ast::blade::Attribute::HtmlAttribute(html_attribute) => html_attribute.children(&mut html_attribute.walk()).filter_map(|ch| {
                let ch = ch.ok()?;
                match ch {
                    ast::blade::anon_unions::AttributeName_AttributeValue_QuotedAttributeValue::AttributeName(attribute_name) => Some(attribute_name.upcast()),
                    ast::blade::anon_unions::AttributeName_AttributeValue_QuotedAttributeValue::AttributeValue(_) |
                    ast::blade::anon_unions::AttributeName_AttributeValue_QuotedAttributeValue::QuotedAttributeValue(_) => None,
                }
            }).last(),
            ast::blade::Attribute::ShortAttribute(short_attribute) => short_attribute.variable_name().ok().map(Node::upcast),
            ast::blade::Attribute::BladeAttribute(_) |
            ast::blade::Attribute::PhpStatement(_) |
            ast::blade::Attribute::Comment(_) |
            ast::blade::Attribute::Conditional(_) |
            ast::blade::Attribute::Envoy(_) |
            ast::blade::Attribute::InlineDirective(_) |
            ast::blade::Attribute::Keyword(_) |
            ast::blade::Attribute::Livewire(_) |
            ast::blade::Attribute::Loops(_) |
            ast::blade::Attribute::Props(_) |
            ast::blade::Attribute::Switch(_) |
            ast::blade::Attribute::WireUi(_) => None,
        }?;
        let attr_name = doc.text_for_node(db, attr_name)?;
        let attr_name = attr_name.strip_prefix(':').unwrap_or(attr_name);
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
        for attr in attrs.as_ref() {
            res.push_attr(attr.name.as_str(), attr.default_value.as_deref());
        }
    }
    res.signature.push('>');
    Some(res)
}

#[cfg(test)]
mod test {
    use std::sync::LazyLock;

    use expect_test::{Expect, expect};

    use crate::analysis::fixture;

    use super::*;

    const TEST_CONFIG: LazyLock<Config> = LazyLock::new(|| Config {
        capabilities: async_lsp::lsp_types::ClientCapabilities::default(),
        workspace_folder: camino::Utf8PathBuf::from_path_buf(std::path::PathBuf::from("/"))
            .unwrap(),
        client_info: None,
    });

    fn check(blase_fixture: &str, expect: Expect) {
        let config = &TEST_CONFIG;
        let (analysis, position) = fixture::position(blase_fixture);
        let help = analysis.signature_help(config, dbg!(position)).unwrap();
        let actual = match dbg!(help) {
            Some(help) => match help.active_parameter {
                Some(active_param) => {
                    let mut rendered = String::new();
                    macros::format_to!(rendered, "{}\n", help.signature);
                    let active_range = help.parameter_ranges()[active_param];
                    for i in 0..help.signature.len() {
                        let marker = if active_range.contains_inclusive((i as u32).into()) {
                            '^'
                        } else {
                            '-'
                        };
                        rendered.push(marker);
                    }
                    rendered
                }
                None => String::new(),
            },
            None => String::new(),
        };
        expect.assert_eq(&actual);
    }

    #[test]
    fn named_attribute_signature_help() {
        check(
            r#"
//- /resources/views/components/index-table.blade.php
@props(['subject', 'rows' => [], 'columns' => []])

//- /resources/views/index.blade.php
<x-index-table :ro$0ws="$units" subject="unit" :columns="[
    'name',
]">
</x-index-table>
            "#,
            expect![[r#"
                <x-index-table subject="" rows="[]" columns="[]">
                --------------------------^^^^^^^^^^-------------"#]],
        );
    }
}
