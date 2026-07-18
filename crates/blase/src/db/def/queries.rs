/// This module contains all the tree-sitter queries that are used in the language server in one place.

pub(crate) const ALL_COMPONENTS_AND_LAYOUTS: &'static str = r#"
((element [(start_tag (tag_name) @tag) (self_closing_tag (tag_name) @tag)]) (#match? @tag "^x-([a-zA-Z\-]+::)?([a-zA-Z\-]\.?)+[a-zA-Z\-]+")) @component
"#;

pub(crate) fn component_and_layout_named(name: &str) -> String {
    format!(
        r#"
((element [(start_tag (tag_name) @tag) (self_closing_tag (tag_name) @tag)]) (#match? @tag "^{}")) @component
"#,
        name
    )
}

pub(crate) const SLOT_CANDIDATE_NAMES: &'static str = r#"
(php_statement
  (variable_name
    (name) @slot))
"#;

pub(crate) const COMMENTS_FOR_CLASS_COMPONENTS: &'static str = r#"
((class_declaration
   (base_clause (name) @class)
   body: (declaration_list
     (comment) @doc
     ((method_declaration name: (name) @method
      (#eq? @method "__construct")))))
 (#eq? @class "Component"))
"#;

pub(crate) const VIEW_INCLUDE_STRING: &'static str = r#"
(inline_directive
   directive:
   ["@include"
    "@includeIf"
    "@includeWhen"
    "@includeUnless"
    "@includeIsolated"]
    (string) @view)

(inline_directive
  directive: "@includeFirst"
  (array_creation_expression
      (array_element_initializer
        (array_element_value_initializer (string) @view)+)))
"#;
