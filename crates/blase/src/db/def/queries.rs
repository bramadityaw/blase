/// This module contains all the tree-sitter queries that are used in the language server in one place.

pub(crate) const ALL_COMPONENTS_AND_LAYOUTS: &'static str = r#"
((element [(start_tag (tag_name) @tag) (self_closing_tag (tag_name) @tag)]) (#match? @tag "^x-([a-zA-Z\-]+::)?([a-zA-Z\-]\.?)+[a-zA-Z\-]+")) @component
"#;

pub fn component_and_layout_named(name: &str) -> String {
    format!(
        r#"
((element [(start_tag (tag_name) @tag) (self_closing_tag (tag_name) @tag)]) (#match? @tag "^{}")) @component
"#,
        name
    )
}
