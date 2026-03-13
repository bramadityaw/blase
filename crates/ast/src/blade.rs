/**Typed node `attribute`

This node has named children of type `{attribute_name | attribute_value | directive | parameter | php_statement | quoted_attribute_value}+`:

- [`AttributeName`]
- [`AttributeValue`]
- [`Directive`]
- [`Parameter`]
- [`PhpStatement`]
- [`QuotedAttributeValue`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Attribute<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Attribute<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for Attribute<'tree> {
    type Child = anon_unions::AttributeName_AttributeValue_Directive_Parameter_PhpStatement_QuotedAttributeValue<
        'tree,
    >;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Attribute<'tree> {
    type WithLifetime<'a> = Attribute<'a>;
    const KIND: &'static str = "attribute";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "attribute" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "attribute");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `attribute_name`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct AttributeName<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> AttributeName<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for AttributeName<'tree> {
    type WithLifetime<'a> = AttributeName<'a>;
    const KIND: &'static str = "attribute_name";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "attribute_name" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "attribute_name");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `attribute_value`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct AttributeValue<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> AttributeValue<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for AttributeValue<'tree> {
    type WithLifetime<'a> = AttributeValue<'a>;
    const KIND: &'static str = "attribute_value";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "attribute_value" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "attribute_value");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `comment`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Comment<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Comment<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Comment<'tree> {
    type WithLifetime<'a> = Comment<'a>;
    const KIND: &'static str = "comment";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "comment" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "comment");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `conditional`

This node has named children of type `{comment | conditional | conditional_keyword | directive | directive_end | directive_start | doctype | element | entity | envoy | erroneous_end_tag | fragment | keyword | livewire | loop | once | parameter | php_statement | script_element | section | stack | style_element | switch | text | verbatim}*`:

- [`Comment`]
- [`Conditional`]
- [`ConditionalKeyword`]
- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Doctype`]
- [`Element`]
- [`Entity`]
- [`Envoy`]
- [`ErroneousEndTag`]
- [`Fragment`]
- [`Keyword`]
- [`Livewire`]
- [`Loop`]
- [`Once`]
- [`Parameter`]
- [`PhpStatement`]
- [`ScriptElement`]
- [`Section`]
- [`Stack`]
- [`StyleElement`]
- [`Switch`]
- [`Text`]
- [`Verbatim`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Conditional<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Conditional<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for Conditional<'tree> {
    type Child = anon_unions::Anon40248131041664470318391226482553460865<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Conditional<'tree> {
    type WithLifetime<'a> = Conditional<'a>;
    const KIND: &'static str = "conditional";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "conditional" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "conditional");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `conditional_keyword`

This node has named children of type `{directive | parameter}+`:

- [`Directive`]
- [`Parameter`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ConditionalKeyword<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ConditionalKeyword<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for ConditionalKeyword<'tree> {
    type Child = anon_unions::Directive_Parameter<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for ConditionalKeyword<'tree> {
    type WithLifetime<'a> = ConditionalKeyword<'a>;
    const KIND: &'static str = "conditional_keyword";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "conditional_keyword" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "conditional_keyword");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `directive`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Directive<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Directive<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Directive<'tree> {
    type WithLifetime<'a> = Directive<'a>;
    const KIND: &'static str = "directive";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "directive" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "directive");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `directive_end`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct DirectiveEnd<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> DirectiveEnd<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for DirectiveEnd<'tree> {
    type WithLifetime<'a> = DirectiveEnd<'a>;
    const KIND: &'static str = "directive_end";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "directive_end" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "directive_end");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `directive_start`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct DirectiveStart<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> DirectiveStart<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for DirectiveStart<'tree> {
    type WithLifetime<'a> = DirectiveStart<'a>;
    const KIND: &'static str = "directive_start";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "directive_start" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "directive_start");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `doctype`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Doctype<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Doctype<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Doctype<'tree> {
    type WithLifetime<'a> = Doctype<'a>;
    const KIND: &'static str = "doctype";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "doctype" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "doctype");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `document`

This node has named children of type `{comment | conditional | directive | doctype | element | entity | envoy | erroneous_end_tag | fragment | keyword | livewire | loop | once | parameter | php_statement | script_element | section | stack | style_element | switch | text | verbatim}*`:

- [`Comment`]
- [`Conditional`]
- [`Directive`]
- [`Doctype`]
- [`Element`]
- [`Entity`]
- [`Envoy`]
- [`ErroneousEndTag`]
- [`Fragment`]
- [`Keyword`]
- [`Livewire`]
- [`Loop`]
- [`Once`]
- [`Parameter`]
- [`PhpStatement`]
- [`ScriptElement`]
- [`Section`]
- [`Stack`]
- [`StyleElement`]
- [`Switch`]
- [`Text`]
- [`Verbatim`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Document<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Document<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for Document<'tree> {
    type Child = anon_unions::Anon125155986692203312381277354493354211763<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Document<'tree> {
    type WithLifetime<'a> = Document<'a>;
    const KIND: &'static str = "document";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "document" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "document");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `element`

This node has named children of type `{comment | conditional | directive | doctype | element | end_tag | entity | envoy | erroneous_end_tag | fragment | keyword | livewire | loop | once | parameter | php_statement | script_element | section | self_closing_tag | stack | start_tag | style_element | switch | text | verbatim}+`:

- [`Comment`]
- [`Conditional`]
- [`Directive`]
- [`Doctype`]
- [`Element`]
- [`EndTag`]
- [`Entity`]
- [`Envoy`]
- [`ErroneousEndTag`]
- [`Fragment`]
- [`Keyword`]
- [`Livewire`]
- [`Loop`]
- [`Once`]
- [`Parameter`]
- [`PhpStatement`]
- [`ScriptElement`]
- [`Section`]
- [`SelfClosingTag`]
- [`Stack`]
- [`StartTag`]
- [`StyleElement`]
- [`Switch`]
- [`Text`]
- [`Verbatim`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Element<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Element<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for Element<'tree> {
    type Child = anon_unions::Anon87989396667188848098758835341468324800<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Element<'tree> {
    type WithLifetime<'a> = Element<'a>;
    const KIND: &'static str = "element";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "element" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "element");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `end_tag`

This node has a named child of type `tag_name` ([`TagName`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct EndTag<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> EndTag<'tree> {
    /**Get the node's only not-extra named child.

This child has type `tag_name` ([`TagName`])*/
    #[inline]
    pub fn tag_name(&self) -> ::type_sitter::NodeResult<'tree, TagName<'tree>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .map(|i| ::type_sitter::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(<TagName<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChild<'tree> for EndTag<'tree> {
    type Child = TagName<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for EndTag<'tree> {
    type WithLifetime<'a> = EndTag<'a>;
    const KIND: &'static str = "end_tag";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "end_tag" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "end_tag");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `entity`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Entity<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Entity<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Entity<'tree> {
    type WithLifetime<'a> = Entity<'a>;
    const KIND: &'static str = "entity";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "entity" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "entity");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `envoy`

This node has named children of type `{conditional_keyword | directive | directive_end | directive_start | parameter | php_only | text}*`:

- [`ConditionalKeyword`]
- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Parameter`]
- [`PhpOnly`]
- [`Text`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Envoy<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Envoy<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for Envoy<'tree> {
    type Child = anon_unions::ConditionalKeyword_Directive_DirectiveEnd_DirectiveStart_Parameter_PhpOnly_Text<
        'tree,
    >;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Envoy<'tree> {
    type WithLifetime<'a> = Envoy<'a>;
    const KIND: &'static str = "envoy";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "envoy" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "envoy");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `erroneous_end_tag`

This node has a named child of type `erroneous_end_tag_name` ([`ErroneousEndTagName`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ErroneousEndTag<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ErroneousEndTag<'tree> {
    /**Get the node's only not-extra named child.

This child has type `erroneous_end_tag_name` ([`ErroneousEndTagName`])*/
    #[inline]
    pub fn erroneous_end_tag_name(
        &self,
    ) -> ::type_sitter::NodeResult<'tree, ErroneousEndTagName<'tree>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .map(|i| ::type_sitter::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(
                <ErroneousEndTagName<'tree> as ::type_sitter::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChild<'tree> for ErroneousEndTag<'tree> {
    type Child = ErroneousEndTagName<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for ErroneousEndTag<'tree> {
    type WithLifetime<'a> = ErroneousEndTag<'a>;
    const KIND: &'static str = "erroneous_end_tag";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "erroneous_end_tag" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "erroneous_end_tag");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `erroneous_end_tag_name`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ErroneousEndTagName<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ErroneousEndTagName<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for ErroneousEndTagName<'tree> {
    type WithLifetime<'a> = ErroneousEndTagName<'a>;
    const KIND: &'static str = "erroneous_end_tag_name";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "erroneous_end_tag_name" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "erroneous_end_tag_name");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `fragment`

This node has named children of type `{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | keyword | loop | parameter | php_statement | script_element | style_element | switch | text | verbatim}+`:

- [`Comment`]
- [`Conditional`]
- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Element`]
- [`Entity`]
- [`ErroneousEndTag`]
- [`Keyword`]
- [`Loop`]
- [`Parameter`]
- [`PhpStatement`]
- [`ScriptElement`]
- [`StyleElement`]
- [`Switch`]
- [`Text`]
- [`Verbatim`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Fragment<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Fragment<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for Fragment<'tree> {
    type Child = anon_unions::Anon220860789005696801182824190281800612819<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Fragment<'tree> {
    type WithLifetime<'a> = Fragment<'a>;
    const KIND: &'static str = "fragment";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "fragment" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "fragment");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `keyword`

This node has a named child of type `directive` ([`Directive`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Keyword<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Keyword<'tree> {
    /**Get the node's only not-extra named child.

This child has type `directive` ([`Directive`])*/
    #[inline]
    pub fn directive(&self) -> ::type_sitter::NodeResult<'tree, Directive<'tree>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .map(|i| ::type_sitter::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(<Directive<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChild<'tree> for Keyword<'tree> {
    type Child = Directive<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Keyword<'tree> {
    type WithLifetime<'a> = Keyword<'a>;
    const KIND: &'static str = "keyword";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "keyword" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "keyword");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `livewire`

This node has named children of type `{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | keyword | livewire | loop | parameter | php_statement | script_element | style_element | switch | text}+`:

- [`Comment`]
- [`Conditional`]
- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Element`]
- [`Entity`]
- [`ErroneousEndTag`]
- [`Keyword`]
- [`Livewire`]
- [`Loop`]
- [`Parameter`]
- [`PhpStatement`]
- [`ScriptElement`]
- [`StyleElement`]
- [`Switch`]
- [`Text`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Livewire<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Livewire<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for Livewire<'tree> {
    type Child = anon_unions::Anon318641722947100924141238363717069648819<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Livewire<'tree> {
    type WithLifetime<'a> = Livewire<'a>;
    const KIND: &'static str = "livewire";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "livewire" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "livewire");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `loop`

This node has named children of type `{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | keyword | loop | parameter | php_statement | script_element | style_element | switch | text}+`:

- [`Comment`]
- [`Conditional`]
- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Element`]
- [`Entity`]
- [`ErroneousEndTag`]
- [`Keyword`]
- [`Loop`]
- [`Parameter`]
- [`PhpStatement`]
- [`ScriptElement`]
- [`StyleElement`]
- [`Switch`]
- [`Text`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Loop<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Loop<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for Loop<'tree> {
    type Child = anon_unions::Anon7748293708392891243390944054945158570<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Loop<'tree> {
    type WithLifetime<'a> = Loop<'a>;
    const KIND: &'static str = "loop";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "loop" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "loop");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `once`

This node has named children of type `{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | fragment | keyword | livewire | loop | once | parameter | php_statement | script_element | stack | style_element | switch | text | verbatim}+`:

- [`Comment`]
- [`Conditional`]
- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Element`]
- [`Entity`]
- [`ErroneousEndTag`]
- [`Fragment`]
- [`Keyword`]
- [`Livewire`]
- [`Loop`]
- [`Once`]
- [`Parameter`]
- [`PhpStatement`]
- [`ScriptElement`]
- [`Stack`]
- [`StyleElement`]
- [`Switch`]
- [`Text`]
- [`Verbatim`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Once<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Once<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for Once<'tree> {
    type Child = anon_unions::Anon167980544356910785377397835572504828111<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Once<'tree> {
    type WithLifetime<'a> = Once<'a>;
    const KIND: &'static str = "once";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "once" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "once");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `parameter`

This node has named children of type `parameter*` ([`Parameter`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Parameter<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Parameter<'tree> {
    /**Get the node's not-extra named children.

These children have type `parameter*` ([`Parameter`])*/
    #[inline]
    pub fn parameters<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<'tree, Parameter<'tree>>,
    > + 'a {
        ::type_sitter::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<Parameter<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for Parameter<'tree> {
    type Child = Parameter<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Parameter<'tree> {
    type WithLifetime<'a> = Parameter<'a>;
    const KIND: &'static str = "parameter";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "parameter" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "parameter");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `php_end_tag`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct PhpEndTag<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> PhpEndTag<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for PhpEndTag<'tree> {
    type WithLifetime<'a> = PhpEndTag<'a>;
    const KIND: &'static str = "php_end_tag";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "php_end_tag" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "php_end_tag");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `php_only`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct PhpOnly<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> PhpOnly<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for PhpOnly<'tree> {
    type WithLifetime<'a> = PhpOnly<'a>;
    const KIND: &'static str = "php_only";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "php_only" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "php_only");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `php_statement`

This node has named children of type `{directive | directive_end | directive_start | parameter | php_end_tag | php_only | php_tag}*`:

- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Parameter`]
- [`PhpEndTag`]
- [`PhpOnly`]
- [`PhpTag`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct PhpStatement<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> PhpStatement<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for PhpStatement<'tree> {
    type Child = anon_unions::Directive_DirectiveEnd_DirectiveStart_Parameter_PhpEndTag_PhpOnly_PhpTag<
        'tree,
    >;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for PhpStatement<'tree> {
    type WithLifetime<'a> = PhpStatement<'a>;
    const KIND: &'static str = "php_statement";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "php_statement" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "php_statement");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `php_tag`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct PhpTag<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> PhpTag<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for PhpTag<'tree> {
    type WithLifetime<'a> = PhpTag<'a>;
    const KIND: &'static str = "php_tag";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "php_tag" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "php_tag");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `quoted_attribute_value`

This node has named children of type `{attribute_value | comment | conditional | directive | parameter | php_statement}*`:

- [`AttributeValue`]
- [`Comment`]
- [`Conditional`]
- [`Directive`]
- [`Parameter`]
- [`PhpStatement`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct QuotedAttributeValue<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> QuotedAttributeValue<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for QuotedAttributeValue<'tree> {
    type Child = anon_unions::AttributeValue_Comment_Conditional_Directive_Parameter_PhpStatement<
        'tree,
    >;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for QuotedAttributeValue<'tree> {
    type WithLifetime<'a> = QuotedAttributeValue<'a>;
    const KIND: &'static str = "quoted_attribute_value";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "quoted_attribute_value" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "quoted_attribute_value");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `raw_text`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct RawText<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> RawText<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for RawText<'tree> {
    type WithLifetime<'a> = RawText<'a>;
    const KIND: &'static str = "raw_text";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "raw_text" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "raw_text");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `script_element`

This node has named children of type `{end_tag | raw_text | start_tag}+`:

- [`EndTag`]
- [`RawText`]
- [`StartTag`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ScriptElement<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ScriptElement<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for ScriptElement<'tree> {
    type Child = anon_unions::EndTag_RawText_StartTag<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for ScriptElement<'tree> {
    type WithLifetime<'a> = ScriptElement<'a>;
    const KIND: &'static str = "script_element";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "script_element" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "script_element");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `section`

This node has named children of type `{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | keyword | livewire | loop | parameter | php_statement | script_element | stack | style_element | switch | text | verbatim}+`:

- [`Comment`]
- [`Conditional`]
- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Element`]
- [`Entity`]
- [`ErroneousEndTag`]
- [`Keyword`]
- [`Livewire`]
- [`Loop`]
- [`Parameter`]
- [`PhpStatement`]
- [`ScriptElement`]
- [`Stack`]
- [`StyleElement`]
- [`Switch`]
- [`Text`]
- [`Verbatim`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Section<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Section<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for Section<'tree> {
    type Child = anon_unions::Anon243545065307733425234527997183447807845<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Section<'tree> {
    type WithLifetime<'a> = Section<'a>;
    const KIND: &'static str = "section";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "section" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "section");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `self_closing_tag`

This node has named children of type `{attribute | tag_name}+`:

- [`Attribute`]
- [`TagName`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct SelfClosingTag<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> SelfClosingTag<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for SelfClosingTag<'tree> {
    type Child = anon_unions::Attribute_TagName<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for SelfClosingTag<'tree> {
    type WithLifetime<'a> = SelfClosingTag<'a>;
    const KIND: &'static str = "self_closing_tag";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "self_closing_tag" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "self_closing_tag");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `stack`

This node has named children of type `{comment | directive | directive_end | directive_start | element | entity | erroneous_end_tag | keyword | parameter | php_statement | script_element | style_element | switch | text}+`:

- [`Comment`]
- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Element`]
- [`Entity`]
- [`ErroneousEndTag`]
- [`Keyword`]
- [`Parameter`]
- [`PhpStatement`]
- [`ScriptElement`]
- [`StyleElement`]
- [`Switch`]
- [`Text`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Stack<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Stack<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for Stack<'tree> {
    type Child = anon_unions::Anon34450808297487433971579096753333213753<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Stack<'tree> {
    type WithLifetime<'a> = Stack<'a>;
    const KIND: &'static str = "stack";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "stack" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "stack");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `start_tag`

This node has named children of type `{attribute | tag_name}+`:

- [`Attribute`]
- [`TagName`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct StartTag<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> StartTag<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for StartTag<'tree> {
    type Child = anon_unions::Attribute_TagName<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for StartTag<'tree> {
    type WithLifetime<'a> = StartTag<'a>;
    const KIND: &'static str = "start_tag";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "start_tag" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "start_tag");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `style_element`

This node has named children of type `{end_tag | raw_text | start_tag}+`:

- [`EndTag`]
- [`RawText`]
- [`StartTag`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct StyleElement<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> StyleElement<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for StyleElement<'tree> {
    type Child = anon_unions::EndTag_RawText_StartTag<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for StyleElement<'tree> {
    type WithLifetime<'a> = StyleElement<'a>;
    const KIND: &'static str = "style_element";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "style_element" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "style_element");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `switch`

This node has named children of type `{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | keyword | livewire | loop | parameter | php_statement | script_element | style_element | text}+`:

- [`Comment`]
- [`Conditional`]
- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Element`]
- [`Entity`]
- [`ErroneousEndTag`]
- [`Keyword`]
- [`Livewire`]
- [`Loop`]
- [`Parameter`]
- [`PhpStatement`]
- [`ScriptElement`]
- [`StyleElement`]
- [`Text`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Switch<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Switch<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for Switch<'tree> {
    type Child = anon_unions::Anon217009455819341117748124715030970988988<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Switch<'tree> {
    type WithLifetime<'a> = Switch<'a>;
    const KIND: &'static str = "switch";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "switch" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "switch");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `tag_name`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct TagName<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> TagName<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for TagName<'tree> {
    type WithLifetime<'a> = TagName<'a>;
    const KIND: &'static str = "tag_name";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "tag_name" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "tag_name");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `text`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Text<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Text<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Text<'tree> {
    type WithLifetime<'a> = Text<'a>;
    const KIND: &'static str = "text";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "text" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "text");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
/**Typed node `verbatim`

This node has named children of type `{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | fragment | keyword | loop | once | parameter | php_statement | script_element | section | stack | style_element | switch | text | verbatim}+`:

- [`Comment`]
- [`Conditional`]
- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Element`]
- [`Entity`]
- [`ErroneousEndTag`]
- [`Fragment`]
- [`Keyword`]
- [`Loop`]
- [`Once`]
- [`Parameter`]
- [`PhpStatement`]
- [`ScriptElement`]
- [`Section`]
- [`Stack`]
- [`StyleElement`]
- [`Switch`]
- [`Text`]
- [`Verbatim`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Verbatim<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Verbatim<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for Verbatim<'tree> {
    type Child = anon_unions::Anon296218948099219678605060842541820530786<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Verbatim<'tree> {
    type WithLifetime<'a> = Verbatim<'a>;
    const KIND: &'static str = "verbatim";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "verbatim" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "verbatim");
        Self(node)
    }
    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }
    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
pub mod unnamed {
    #[allow(unused_imports)]
    use super::*;
    /**Typed node `doctype`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Doctype<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Doctype<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Doctype<'tree> {
        type WithLifetime<'a> = Doctype<'a>;
        const KIND: &'static str = "doctype";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "doctype" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "doctype");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
}
pub mod symbols {
    #[allow(unused_imports)]
    use super::*;
    /**Typed node `!!}`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct NotNotRBrace<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> NotNotRBrace<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for NotNotRBrace<'tree> {
        type WithLifetime<'a> = NotNotRBrace<'a>;
        const KIND: &'static str = "!!}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "!!}" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "!!}");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `"`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct DoubleQuote<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> DoubleQuote<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for DoubleQuote<'tree> {
        type WithLifetime<'a> = DoubleQuote<'a>;
        const KIND: &'static str = "\"";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "\"" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "\"");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `'`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Quote<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Quote<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Quote<'tree> {
        type WithLifetime<'a> = Quote<'a>;
        const KIND: &'static str = "'";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "'" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "'");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `(`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct LParen<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> LParen<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for LParen<'tree> {
        type WithLifetime<'a> = LParen<'a>;
        const KIND: &'static str = "(";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "(" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "(");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `)`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct RParen<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> RParen<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for RParen<'tree> {
        type WithLifetime<'a> = RParen<'a>;
        const KIND: &'static str = ")";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == ")" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), ")");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `,`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Comma<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Comma<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Comma<'tree> {
        type WithLifetime<'a> = Comma<'a>;
        const KIND: &'static str = ",";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "," {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), ",");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `/>`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct DivGt<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> DivGt<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for DivGt<'tree> {
        type WithLifetime<'a> = DivGt<'a>;
        const KIND: &'static str = "/>";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "/>" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "/>");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `<`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Lt<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Lt<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Lt<'tree> {
        type WithLifetime<'a> = Lt<'a>;
        const KIND: &'static str = "<";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "<" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "<");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `<!`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct LtNot<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> LtNot<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for LtNot<'tree> {
        type WithLifetime<'a> = LtNot<'a>;
        const KIND: &'static str = "<!";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "<!" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "<!");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `</`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct LtDiv<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> LtDiv<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for LtDiv<'tree> {
        type WithLifetime<'a> = LtDiv<'a>;
        const KIND: &'static str = "</";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "</" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "</");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `=`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Eq<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Eq<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Eq<'tree> {
        type WithLifetime<'a> = Eq<'a>;
        const KIND: &'static str = "=";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "=");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `>`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Gt<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Gt<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Gt<'tree> {
        type WithLifetime<'a> = Gt<'a>;
        const KIND: &'static str = ">";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == ">" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), ">");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `{!!`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct LBraceNotNot<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> LBraceNotNot<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for LBraceNotNot<'tree> {
        type WithLifetime<'a> = LBraceNotNot<'a>;
        const KIND: &'static str = "{!!";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "{!!" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "{!!");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `{{`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct LBraceLBrace<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> LBraceLBrace<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for LBraceLBrace<'tree> {
        type WithLifetime<'a> = LBraceLBrace<'a>;
        const KIND: &'static str = "{{";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "{{" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "{{");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    /**Typed node `}}`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct RBraceRBrace<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> RBraceRBrace<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for RBraceRBrace<'tree> {
        type WithLifetime<'a> = RBraceRBrace<'a>;
        const KIND: &'static str = "}}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "}}" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "}}");
            Self(node)
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
}
pub mod anon_unions {
    #[allow(unused_imports)]
    use super::*;
    /**One of `{comment | conditional | directive | doctype | element | entity | envoy | erroneous_end_tag | fragment | keyword | livewire | loop | once | parameter | php_statement | script_element | section | stack | style_element | switch | text | verbatim}`:
- [`Comment`]
- [`Conditional`]
- [`Directive`]
- [`Doctype`]
- [`Element`]
- [`Entity`]
- [`Envoy`]
- [`ErroneousEndTag`]
- [`Fragment`]
- [`Keyword`]
- [`Livewire`]
- [`Loop`]
- [`Once`]
- [`Parameter`]
- [`PhpStatement`]
- [`ScriptElement`]
- [`Section`]
- [`Stack`]
- [`StyleElement`]
- [`Switch`]
- [`Text`]
- [`Verbatim`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Anon125155986692203312381277354493354211763<'tree> {
        Comment(Comment<'tree>),
        Conditional(Conditional<'tree>),
        Directive(Directive<'tree>),
        Doctype(Doctype<'tree>),
        Element(Element<'tree>),
        Entity(Entity<'tree>),
        Envoy(Envoy<'tree>),
        ErroneousEndTag(ErroneousEndTag<'tree>),
        Fragment(Fragment<'tree>),
        Keyword(Keyword<'tree>),
        Livewire(Livewire<'tree>),
        Loop(Loop<'tree>),
        Once(Once<'tree>),
        Parameter(Parameter<'tree>),
        PhpStatement(PhpStatement<'tree>),
        ScriptElement(ScriptElement<'tree>),
        Section(Section<'tree>),
        Stack(Stack<'tree>),
        StyleElement(StyleElement<'tree>),
        Switch(Switch<'tree>),
        Text(Text<'tree>),
        Verbatim(Verbatim<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Anon125155986692203312381277354493354211763<'tree> {
        ///Returns the node if it is of type `comment` ([`Comment`]), otherwise returns `None`
        #[inline]
        pub fn as_comment(self) -> ::std::option::Option<Comment<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Comment(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `conditional` ([`Conditional`]), otherwise returns `None`
        #[inline]
        pub fn as_conditional(self) -> ::std::option::Option<Conditional<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Conditional(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `directive` ([`Directive`]), otherwise returns `None`
        #[inline]
        pub fn as_directive(self) -> ::std::option::Option<Directive<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Directive(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `doctype` ([`Doctype`]), otherwise returns `None`
        #[inline]
        pub fn as_doctype(self) -> ::std::option::Option<Doctype<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Doctype(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `element` ([`Element`]), otherwise returns `None`
        #[inline]
        pub fn as_element(self) -> ::std::option::Option<Element<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Element(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `entity` ([`Entity`]), otherwise returns `None`
        #[inline]
        pub fn as_entity(self) -> ::std::option::Option<Entity<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Entity(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `envoy` ([`Envoy`]), otherwise returns `None`
        #[inline]
        pub fn as_envoy(self) -> ::std::option::Option<Envoy<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Envoy(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `erroneous_end_tag` ([`ErroneousEndTag`]), otherwise returns `None`
        #[inline]
        pub fn as_erroneous_end_tag(
            self,
        ) -> ::std::option::Option<ErroneousEndTag<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ErroneousEndTag(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `fragment` ([`Fragment`]), otherwise returns `None`
        #[inline]
        pub fn as_fragment(self) -> ::std::option::Option<Fragment<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Fragment(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `keyword` ([`Keyword`]), otherwise returns `None`
        #[inline]
        pub fn as_keyword(self) -> ::std::option::Option<Keyword<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Keyword(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `livewire` ([`Livewire`]), otherwise returns `None`
        #[inline]
        pub fn as_livewire(self) -> ::std::option::Option<Livewire<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Livewire(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `loop` ([`Loop`]), otherwise returns `None`
        #[inline]
        pub fn as_loop(self) -> ::std::option::Option<Loop<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Loop(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `once` ([`Once`]), otherwise returns `None`
        #[inline]
        pub fn as_once(self) -> ::std::option::Option<Once<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Once(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `parameter` ([`Parameter`]), otherwise returns `None`
        #[inline]
        pub fn as_parameter(self) -> ::std::option::Option<Parameter<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Parameter(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `php_statement` ([`PhpStatement`]), otherwise returns `None`
        #[inline]
        pub fn as_php_statement(self) -> ::std::option::Option<PhpStatement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::PhpStatement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `script_element` ([`ScriptElement`]), otherwise returns `None`
        #[inline]
        pub fn as_script_element(self) -> ::std::option::Option<ScriptElement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ScriptElement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `section` ([`Section`]), otherwise returns `None`
        #[inline]
        pub fn as_section(self) -> ::std::option::Option<Section<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Section(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `stack` ([`Stack`]), otherwise returns `None`
        #[inline]
        pub fn as_stack(self) -> ::std::option::Option<Stack<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Stack(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `style_element` ([`StyleElement`]), otherwise returns `None`
        #[inline]
        pub fn as_style_element(self) -> ::std::option::Option<StyleElement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::StyleElement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `switch` ([`Switch`]), otherwise returns `None`
        #[inline]
        pub fn as_switch(self) -> ::std::option::Option<Switch<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Switch(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `text` ([`Text`]), otherwise returns `None`
        #[inline]
        pub fn as_text(self) -> ::std::option::Option<Text<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Text(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `verbatim` ([`Verbatim`]), otherwise returns `None`
        #[inline]
        pub fn as_verbatim(self) -> ::std::option::Option<Verbatim<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Verbatim(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
    for Anon125155986692203312381277354493354211763<'tree> {
        type WithLifetime<'a> = Anon125155986692203312381277354493354211763<'a>;
        const KIND: &'static str = "{comment | conditional | directive | doctype | element | entity | envoy | erroneous_end_tag | fragment | keyword | livewire | loop | once | parameter | php_statement | script_element | section | stack | style_element | switch | text | verbatim}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "comment" => {
                    Ok(unsafe {
                        Self::Comment(
                            <Comment<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "conditional" => {
                    Ok(unsafe {
                        Self::Conditional(
                            <Conditional<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "directive" => {
                    Ok(unsafe {
                        Self::Directive(
                            <Directive<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "doctype" => {
                    Ok(unsafe {
                        Self::Doctype(
                            <Doctype<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "element" => {
                    Ok(unsafe {
                        Self::Element(
                            <Element<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "entity" => {
                    Ok(unsafe {
                        Self::Entity(
                            <Entity<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "envoy" => {
                    Ok(unsafe {
                        Self::Envoy(
                            <Envoy<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "erroneous_end_tag" => {
                    Ok(unsafe {
                        Self::ErroneousEndTag(
                            <ErroneousEndTag<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "fragment" => {
                    Ok(unsafe {
                        Self::Fragment(
                            <Fragment<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "keyword" => {
                    Ok(unsafe {
                        Self::Keyword(
                            <Keyword<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "livewire" => {
                    Ok(unsafe {
                        Self::Livewire(
                            <Livewire<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "loop" => {
                    Ok(unsafe {
                        Self::Loop(
                            <Loop<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "once" => {
                    Ok(unsafe {
                        Self::Once(
                            <Once<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "parameter" => {
                    Ok(unsafe {
                        Self::Parameter(
                            <Parameter<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "php_statement" => {
                    Ok(unsafe {
                        Self::PhpStatement(
                            <PhpStatement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "script_element" => {
                    Ok(unsafe {
                        Self::ScriptElement(
                            <ScriptElement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "section" => {
                    Ok(unsafe {
                        Self::Section(
                            <Section<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "stack" => {
                    Ok(unsafe {
                        Self::Stack(
                            <Stack<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "style_element" => {
                    Ok(unsafe {
                        Self::StyleElement(
                            <StyleElement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "switch" => {
                    Ok(unsafe {
                        Self::Switch(
                            <Switch<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "text" => {
                    Ok(unsafe {
                        Self::Text(
                            <Text<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "verbatim" => {
                    Ok(unsafe {
                        Self::Verbatim(
                            <Verbatim<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Comment(x) => ::type_sitter::Node::raw(x),
                Self::Conditional(x) => ::type_sitter::Node::raw(x),
                Self::Directive(x) => ::type_sitter::Node::raw(x),
                Self::Doctype(x) => ::type_sitter::Node::raw(x),
                Self::Element(x) => ::type_sitter::Node::raw(x),
                Self::Entity(x) => ::type_sitter::Node::raw(x),
                Self::Envoy(x) => ::type_sitter::Node::raw(x),
                Self::ErroneousEndTag(x) => ::type_sitter::Node::raw(x),
                Self::Fragment(x) => ::type_sitter::Node::raw(x),
                Self::Keyword(x) => ::type_sitter::Node::raw(x),
                Self::Livewire(x) => ::type_sitter::Node::raw(x),
                Self::Loop(x) => ::type_sitter::Node::raw(x),
                Self::Once(x) => ::type_sitter::Node::raw(x),
                Self::Parameter(x) => ::type_sitter::Node::raw(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw(x),
                Self::ScriptElement(x) => ::type_sitter::Node::raw(x),
                Self::Section(x) => ::type_sitter::Node::raw(x),
                Self::Stack(x) => ::type_sitter::Node::raw(x),
                Self::StyleElement(x) => ::type_sitter::Node::raw(x),
                Self::Switch(x) => ::type_sitter::Node::raw(x),
                Self::Text(x) => ::type_sitter::Node::raw(x),
                Self::Verbatim(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Comment(x) => ::type_sitter::Node::raw_mut(x),
                Self::Conditional(x) => ::type_sitter::Node::raw_mut(x),
                Self::Directive(x) => ::type_sitter::Node::raw_mut(x),
                Self::Doctype(x) => ::type_sitter::Node::raw_mut(x),
                Self::Element(x) => ::type_sitter::Node::raw_mut(x),
                Self::Entity(x) => ::type_sitter::Node::raw_mut(x),
                Self::Envoy(x) => ::type_sitter::Node::raw_mut(x),
                Self::ErroneousEndTag(x) => ::type_sitter::Node::raw_mut(x),
                Self::Fragment(x) => ::type_sitter::Node::raw_mut(x),
                Self::Keyword(x) => ::type_sitter::Node::raw_mut(x),
                Self::Livewire(x) => ::type_sitter::Node::raw_mut(x),
                Self::Loop(x) => ::type_sitter::Node::raw_mut(x),
                Self::Once(x) => ::type_sitter::Node::raw_mut(x),
                Self::Parameter(x) => ::type_sitter::Node::raw_mut(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw_mut(x),
                Self::ScriptElement(x) => ::type_sitter::Node::raw_mut(x),
                Self::Section(x) => ::type_sitter::Node::raw_mut(x),
                Self::Stack(x) => ::type_sitter::Node::raw_mut(x),
                Self::StyleElement(x) => ::type_sitter::Node::raw_mut(x),
                Self::Switch(x) => ::type_sitter::Node::raw_mut(x),
                Self::Text(x) => ::type_sitter::Node::raw_mut(x),
                Self::Verbatim(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Comment(x) => x.into_raw(),
                Self::Conditional(x) => x.into_raw(),
                Self::Directive(x) => x.into_raw(),
                Self::Doctype(x) => x.into_raw(),
                Self::Element(x) => x.into_raw(),
                Self::Entity(x) => x.into_raw(),
                Self::Envoy(x) => x.into_raw(),
                Self::ErroneousEndTag(x) => x.into_raw(),
                Self::Fragment(x) => x.into_raw(),
                Self::Keyword(x) => x.into_raw(),
                Self::Livewire(x) => x.into_raw(),
                Self::Loop(x) => x.into_raw(),
                Self::Once(x) => x.into_raw(),
                Self::Parameter(x) => x.into_raw(),
                Self::PhpStatement(x) => x.into_raw(),
                Self::ScriptElement(x) => x.into_raw(),
                Self::Section(x) => x.into_raw(),
                Self::Stack(x) => x.into_raw(),
                Self::StyleElement(x) => x.into_raw(),
                Self::Switch(x) => x.into_raw(),
                Self::Text(x) => x.into_raw(),
                Self::Verbatim(x) => x.into_raw(),
            }
        }
    }
    /**One of `{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | fragment | keyword | livewire | loop | once | parameter | php_statement | script_element | stack | style_element | switch | text | verbatim}`:
- [`Comment`]
- [`Conditional`]
- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Element`]
- [`Entity`]
- [`ErroneousEndTag`]
- [`Fragment`]
- [`Keyword`]
- [`Livewire`]
- [`Loop`]
- [`Once`]
- [`Parameter`]
- [`PhpStatement`]
- [`ScriptElement`]
- [`Stack`]
- [`StyleElement`]
- [`Switch`]
- [`Text`]
- [`Verbatim`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Anon167980544356910785377397835572504828111<'tree> {
        Comment(Comment<'tree>),
        Conditional(Conditional<'tree>),
        Directive(Directive<'tree>),
        DirectiveEnd(DirectiveEnd<'tree>),
        DirectiveStart(DirectiveStart<'tree>),
        Element(Element<'tree>),
        Entity(Entity<'tree>),
        ErroneousEndTag(ErroneousEndTag<'tree>),
        Fragment(Fragment<'tree>),
        Keyword(Keyword<'tree>),
        Livewire(Livewire<'tree>),
        Loop(Loop<'tree>),
        Once(Once<'tree>),
        Parameter(Parameter<'tree>),
        PhpStatement(PhpStatement<'tree>),
        ScriptElement(ScriptElement<'tree>),
        Stack(Stack<'tree>),
        StyleElement(StyleElement<'tree>),
        Switch(Switch<'tree>),
        Text(Text<'tree>),
        Verbatim(Verbatim<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Anon167980544356910785377397835572504828111<'tree> {
        ///Returns the node if it is of type `comment` ([`Comment`]), otherwise returns `None`
        #[inline]
        pub fn as_comment(self) -> ::std::option::Option<Comment<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Comment(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `conditional` ([`Conditional`]), otherwise returns `None`
        #[inline]
        pub fn as_conditional(self) -> ::std::option::Option<Conditional<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Conditional(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `directive` ([`Directive`]), otherwise returns `None`
        #[inline]
        pub fn as_directive(self) -> ::std::option::Option<Directive<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Directive(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `directive_end` ([`DirectiveEnd`]), otherwise returns `None`
        #[inline]
        pub fn as_directive_end(self) -> ::std::option::Option<DirectiveEnd<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DirectiveEnd(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `directive_start` ([`DirectiveStart`]), otherwise returns `None`
        #[inline]
        pub fn as_directive_start(self) -> ::std::option::Option<DirectiveStart<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DirectiveStart(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `element` ([`Element`]), otherwise returns `None`
        #[inline]
        pub fn as_element(self) -> ::std::option::Option<Element<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Element(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `entity` ([`Entity`]), otherwise returns `None`
        #[inline]
        pub fn as_entity(self) -> ::std::option::Option<Entity<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Entity(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `erroneous_end_tag` ([`ErroneousEndTag`]), otherwise returns `None`
        #[inline]
        pub fn as_erroneous_end_tag(
            self,
        ) -> ::std::option::Option<ErroneousEndTag<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ErroneousEndTag(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `fragment` ([`Fragment`]), otherwise returns `None`
        #[inline]
        pub fn as_fragment(self) -> ::std::option::Option<Fragment<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Fragment(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `keyword` ([`Keyword`]), otherwise returns `None`
        #[inline]
        pub fn as_keyword(self) -> ::std::option::Option<Keyword<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Keyword(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `livewire` ([`Livewire`]), otherwise returns `None`
        #[inline]
        pub fn as_livewire(self) -> ::std::option::Option<Livewire<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Livewire(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `loop` ([`Loop`]), otherwise returns `None`
        #[inline]
        pub fn as_loop(self) -> ::std::option::Option<Loop<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Loop(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `once` ([`Once`]), otherwise returns `None`
        #[inline]
        pub fn as_once(self) -> ::std::option::Option<Once<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Once(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `parameter` ([`Parameter`]), otherwise returns `None`
        #[inline]
        pub fn as_parameter(self) -> ::std::option::Option<Parameter<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Parameter(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `php_statement` ([`PhpStatement`]), otherwise returns `None`
        #[inline]
        pub fn as_php_statement(self) -> ::std::option::Option<PhpStatement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::PhpStatement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `script_element` ([`ScriptElement`]), otherwise returns `None`
        #[inline]
        pub fn as_script_element(self) -> ::std::option::Option<ScriptElement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ScriptElement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `stack` ([`Stack`]), otherwise returns `None`
        #[inline]
        pub fn as_stack(self) -> ::std::option::Option<Stack<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Stack(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `style_element` ([`StyleElement`]), otherwise returns `None`
        #[inline]
        pub fn as_style_element(self) -> ::std::option::Option<StyleElement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::StyleElement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `switch` ([`Switch`]), otherwise returns `None`
        #[inline]
        pub fn as_switch(self) -> ::std::option::Option<Switch<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Switch(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `text` ([`Text`]), otherwise returns `None`
        #[inline]
        pub fn as_text(self) -> ::std::option::Option<Text<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Text(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `verbatim` ([`Verbatim`]), otherwise returns `None`
        #[inline]
        pub fn as_verbatim(self) -> ::std::option::Option<Verbatim<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Verbatim(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
    for Anon167980544356910785377397835572504828111<'tree> {
        type WithLifetime<'a> = Anon167980544356910785377397835572504828111<'a>;
        const KIND: &'static str = "{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | fragment | keyword | livewire | loop | once | parameter | php_statement | script_element | stack | style_element | switch | text | verbatim}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "comment" => {
                    Ok(unsafe {
                        Self::Comment(
                            <Comment<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "conditional" => {
                    Ok(unsafe {
                        Self::Conditional(
                            <Conditional<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "directive" => {
                    Ok(unsafe {
                        Self::Directive(
                            <Directive<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "directive_end" => {
                    Ok(unsafe {
                        Self::DirectiveEnd(
                            <DirectiveEnd<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "directive_start" => {
                    Ok(unsafe {
                        Self::DirectiveStart(
                            <DirectiveStart<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "element" => {
                    Ok(unsafe {
                        Self::Element(
                            <Element<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "entity" => {
                    Ok(unsafe {
                        Self::Entity(
                            <Entity<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "erroneous_end_tag" => {
                    Ok(unsafe {
                        Self::ErroneousEndTag(
                            <ErroneousEndTag<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "fragment" => {
                    Ok(unsafe {
                        Self::Fragment(
                            <Fragment<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "keyword" => {
                    Ok(unsafe {
                        Self::Keyword(
                            <Keyword<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "livewire" => {
                    Ok(unsafe {
                        Self::Livewire(
                            <Livewire<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "loop" => {
                    Ok(unsafe {
                        Self::Loop(
                            <Loop<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "once" => {
                    Ok(unsafe {
                        Self::Once(
                            <Once<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "parameter" => {
                    Ok(unsafe {
                        Self::Parameter(
                            <Parameter<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "php_statement" => {
                    Ok(unsafe {
                        Self::PhpStatement(
                            <PhpStatement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "script_element" => {
                    Ok(unsafe {
                        Self::ScriptElement(
                            <ScriptElement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "stack" => {
                    Ok(unsafe {
                        Self::Stack(
                            <Stack<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "style_element" => {
                    Ok(unsafe {
                        Self::StyleElement(
                            <StyleElement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "switch" => {
                    Ok(unsafe {
                        Self::Switch(
                            <Switch<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "text" => {
                    Ok(unsafe {
                        Self::Text(
                            <Text<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "verbatim" => {
                    Ok(unsafe {
                        Self::Verbatim(
                            <Verbatim<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Comment(x) => ::type_sitter::Node::raw(x),
                Self::Conditional(x) => ::type_sitter::Node::raw(x),
                Self::Directive(x) => ::type_sitter::Node::raw(x),
                Self::DirectiveEnd(x) => ::type_sitter::Node::raw(x),
                Self::DirectiveStart(x) => ::type_sitter::Node::raw(x),
                Self::Element(x) => ::type_sitter::Node::raw(x),
                Self::Entity(x) => ::type_sitter::Node::raw(x),
                Self::ErroneousEndTag(x) => ::type_sitter::Node::raw(x),
                Self::Fragment(x) => ::type_sitter::Node::raw(x),
                Self::Keyword(x) => ::type_sitter::Node::raw(x),
                Self::Livewire(x) => ::type_sitter::Node::raw(x),
                Self::Loop(x) => ::type_sitter::Node::raw(x),
                Self::Once(x) => ::type_sitter::Node::raw(x),
                Self::Parameter(x) => ::type_sitter::Node::raw(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw(x),
                Self::ScriptElement(x) => ::type_sitter::Node::raw(x),
                Self::Stack(x) => ::type_sitter::Node::raw(x),
                Self::StyleElement(x) => ::type_sitter::Node::raw(x),
                Self::Switch(x) => ::type_sitter::Node::raw(x),
                Self::Text(x) => ::type_sitter::Node::raw(x),
                Self::Verbatim(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Comment(x) => ::type_sitter::Node::raw_mut(x),
                Self::Conditional(x) => ::type_sitter::Node::raw_mut(x),
                Self::Directive(x) => ::type_sitter::Node::raw_mut(x),
                Self::DirectiveEnd(x) => ::type_sitter::Node::raw_mut(x),
                Self::DirectiveStart(x) => ::type_sitter::Node::raw_mut(x),
                Self::Element(x) => ::type_sitter::Node::raw_mut(x),
                Self::Entity(x) => ::type_sitter::Node::raw_mut(x),
                Self::ErroneousEndTag(x) => ::type_sitter::Node::raw_mut(x),
                Self::Fragment(x) => ::type_sitter::Node::raw_mut(x),
                Self::Keyword(x) => ::type_sitter::Node::raw_mut(x),
                Self::Livewire(x) => ::type_sitter::Node::raw_mut(x),
                Self::Loop(x) => ::type_sitter::Node::raw_mut(x),
                Self::Once(x) => ::type_sitter::Node::raw_mut(x),
                Self::Parameter(x) => ::type_sitter::Node::raw_mut(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw_mut(x),
                Self::ScriptElement(x) => ::type_sitter::Node::raw_mut(x),
                Self::Stack(x) => ::type_sitter::Node::raw_mut(x),
                Self::StyleElement(x) => ::type_sitter::Node::raw_mut(x),
                Self::Switch(x) => ::type_sitter::Node::raw_mut(x),
                Self::Text(x) => ::type_sitter::Node::raw_mut(x),
                Self::Verbatim(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Comment(x) => x.into_raw(),
                Self::Conditional(x) => x.into_raw(),
                Self::Directive(x) => x.into_raw(),
                Self::DirectiveEnd(x) => x.into_raw(),
                Self::DirectiveStart(x) => x.into_raw(),
                Self::Element(x) => x.into_raw(),
                Self::Entity(x) => x.into_raw(),
                Self::ErroneousEndTag(x) => x.into_raw(),
                Self::Fragment(x) => x.into_raw(),
                Self::Keyword(x) => x.into_raw(),
                Self::Livewire(x) => x.into_raw(),
                Self::Loop(x) => x.into_raw(),
                Self::Once(x) => x.into_raw(),
                Self::Parameter(x) => x.into_raw(),
                Self::PhpStatement(x) => x.into_raw(),
                Self::ScriptElement(x) => x.into_raw(),
                Self::Stack(x) => x.into_raw(),
                Self::StyleElement(x) => x.into_raw(),
                Self::Switch(x) => x.into_raw(),
                Self::Text(x) => x.into_raw(),
                Self::Verbatim(x) => x.into_raw(),
            }
        }
    }
    /**One of `{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | keyword | livewire | loop | parameter | php_statement | script_element | style_element | text}`:
- [`Comment`]
- [`Conditional`]
- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Element`]
- [`Entity`]
- [`ErroneousEndTag`]
- [`Keyword`]
- [`Livewire`]
- [`Loop`]
- [`Parameter`]
- [`PhpStatement`]
- [`ScriptElement`]
- [`StyleElement`]
- [`Text`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Anon217009455819341117748124715030970988988<'tree> {
        Comment(Comment<'tree>),
        Conditional(Conditional<'tree>),
        Directive(Directive<'tree>),
        DirectiveEnd(DirectiveEnd<'tree>),
        DirectiveStart(DirectiveStart<'tree>),
        Element(Element<'tree>),
        Entity(Entity<'tree>),
        ErroneousEndTag(ErroneousEndTag<'tree>),
        Keyword(Keyword<'tree>),
        Livewire(Livewire<'tree>),
        Loop(Loop<'tree>),
        Parameter(Parameter<'tree>),
        PhpStatement(PhpStatement<'tree>),
        ScriptElement(ScriptElement<'tree>),
        StyleElement(StyleElement<'tree>),
        Text(Text<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Anon217009455819341117748124715030970988988<'tree> {
        ///Returns the node if it is of type `comment` ([`Comment`]), otherwise returns `None`
        #[inline]
        pub fn as_comment(self) -> ::std::option::Option<Comment<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Comment(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `conditional` ([`Conditional`]), otherwise returns `None`
        #[inline]
        pub fn as_conditional(self) -> ::std::option::Option<Conditional<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Conditional(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `directive` ([`Directive`]), otherwise returns `None`
        #[inline]
        pub fn as_directive(self) -> ::std::option::Option<Directive<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Directive(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `directive_end` ([`DirectiveEnd`]), otherwise returns `None`
        #[inline]
        pub fn as_directive_end(self) -> ::std::option::Option<DirectiveEnd<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DirectiveEnd(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `directive_start` ([`DirectiveStart`]), otherwise returns `None`
        #[inline]
        pub fn as_directive_start(self) -> ::std::option::Option<DirectiveStart<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DirectiveStart(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `element` ([`Element`]), otherwise returns `None`
        #[inline]
        pub fn as_element(self) -> ::std::option::Option<Element<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Element(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `entity` ([`Entity`]), otherwise returns `None`
        #[inline]
        pub fn as_entity(self) -> ::std::option::Option<Entity<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Entity(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `erroneous_end_tag` ([`ErroneousEndTag`]), otherwise returns `None`
        #[inline]
        pub fn as_erroneous_end_tag(
            self,
        ) -> ::std::option::Option<ErroneousEndTag<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ErroneousEndTag(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `keyword` ([`Keyword`]), otherwise returns `None`
        #[inline]
        pub fn as_keyword(self) -> ::std::option::Option<Keyword<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Keyword(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `livewire` ([`Livewire`]), otherwise returns `None`
        #[inline]
        pub fn as_livewire(self) -> ::std::option::Option<Livewire<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Livewire(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `loop` ([`Loop`]), otherwise returns `None`
        #[inline]
        pub fn as_loop(self) -> ::std::option::Option<Loop<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Loop(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `parameter` ([`Parameter`]), otherwise returns `None`
        #[inline]
        pub fn as_parameter(self) -> ::std::option::Option<Parameter<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Parameter(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `php_statement` ([`PhpStatement`]), otherwise returns `None`
        #[inline]
        pub fn as_php_statement(self) -> ::std::option::Option<PhpStatement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::PhpStatement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `script_element` ([`ScriptElement`]), otherwise returns `None`
        #[inline]
        pub fn as_script_element(self) -> ::std::option::Option<ScriptElement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ScriptElement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `style_element` ([`StyleElement`]), otherwise returns `None`
        #[inline]
        pub fn as_style_element(self) -> ::std::option::Option<StyleElement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::StyleElement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `text` ([`Text`]), otherwise returns `None`
        #[inline]
        pub fn as_text(self) -> ::std::option::Option<Text<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Text(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
    for Anon217009455819341117748124715030970988988<'tree> {
        type WithLifetime<'a> = Anon217009455819341117748124715030970988988<'a>;
        const KIND: &'static str = "{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | keyword | livewire | loop | parameter | php_statement | script_element | style_element | text}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "comment" => {
                    Ok(unsafe {
                        Self::Comment(
                            <Comment<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "conditional" => {
                    Ok(unsafe {
                        Self::Conditional(
                            <Conditional<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "directive" => {
                    Ok(unsafe {
                        Self::Directive(
                            <Directive<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "directive_end" => {
                    Ok(unsafe {
                        Self::DirectiveEnd(
                            <DirectiveEnd<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "directive_start" => {
                    Ok(unsafe {
                        Self::DirectiveStart(
                            <DirectiveStart<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "element" => {
                    Ok(unsafe {
                        Self::Element(
                            <Element<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "entity" => {
                    Ok(unsafe {
                        Self::Entity(
                            <Entity<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "erroneous_end_tag" => {
                    Ok(unsafe {
                        Self::ErroneousEndTag(
                            <ErroneousEndTag<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "keyword" => {
                    Ok(unsafe {
                        Self::Keyword(
                            <Keyword<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "livewire" => {
                    Ok(unsafe {
                        Self::Livewire(
                            <Livewire<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "loop" => {
                    Ok(unsafe {
                        Self::Loop(
                            <Loop<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "parameter" => {
                    Ok(unsafe {
                        Self::Parameter(
                            <Parameter<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "php_statement" => {
                    Ok(unsafe {
                        Self::PhpStatement(
                            <PhpStatement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "script_element" => {
                    Ok(unsafe {
                        Self::ScriptElement(
                            <ScriptElement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "style_element" => {
                    Ok(unsafe {
                        Self::StyleElement(
                            <StyleElement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "text" => {
                    Ok(unsafe {
                        Self::Text(
                            <Text<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Comment(x) => ::type_sitter::Node::raw(x),
                Self::Conditional(x) => ::type_sitter::Node::raw(x),
                Self::Directive(x) => ::type_sitter::Node::raw(x),
                Self::DirectiveEnd(x) => ::type_sitter::Node::raw(x),
                Self::DirectiveStart(x) => ::type_sitter::Node::raw(x),
                Self::Element(x) => ::type_sitter::Node::raw(x),
                Self::Entity(x) => ::type_sitter::Node::raw(x),
                Self::ErroneousEndTag(x) => ::type_sitter::Node::raw(x),
                Self::Keyword(x) => ::type_sitter::Node::raw(x),
                Self::Livewire(x) => ::type_sitter::Node::raw(x),
                Self::Loop(x) => ::type_sitter::Node::raw(x),
                Self::Parameter(x) => ::type_sitter::Node::raw(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw(x),
                Self::ScriptElement(x) => ::type_sitter::Node::raw(x),
                Self::StyleElement(x) => ::type_sitter::Node::raw(x),
                Self::Text(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Comment(x) => ::type_sitter::Node::raw_mut(x),
                Self::Conditional(x) => ::type_sitter::Node::raw_mut(x),
                Self::Directive(x) => ::type_sitter::Node::raw_mut(x),
                Self::DirectiveEnd(x) => ::type_sitter::Node::raw_mut(x),
                Self::DirectiveStart(x) => ::type_sitter::Node::raw_mut(x),
                Self::Element(x) => ::type_sitter::Node::raw_mut(x),
                Self::Entity(x) => ::type_sitter::Node::raw_mut(x),
                Self::ErroneousEndTag(x) => ::type_sitter::Node::raw_mut(x),
                Self::Keyword(x) => ::type_sitter::Node::raw_mut(x),
                Self::Livewire(x) => ::type_sitter::Node::raw_mut(x),
                Self::Loop(x) => ::type_sitter::Node::raw_mut(x),
                Self::Parameter(x) => ::type_sitter::Node::raw_mut(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw_mut(x),
                Self::ScriptElement(x) => ::type_sitter::Node::raw_mut(x),
                Self::StyleElement(x) => ::type_sitter::Node::raw_mut(x),
                Self::Text(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Comment(x) => x.into_raw(),
                Self::Conditional(x) => x.into_raw(),
                Self::Directive(x) => x.into_raw(),
                Self::DirectiveEnd(x) => x.into_raw(),
                Self::DirectiveStart(x) => x.into_raw(),
                Self::Element(x) => x.into_raw(),
                Self::Entity(x) => x.into_raw(),
                Self::ErroneousEndTag(x) => x.into_raw(),
                Self::Keyword(x) => x.into_raw(),
                Self::Livewire(x) => x.into_raw(),
                Self::Loop(x) => x.into_raw(),
                Self::Parameter(x) => x.into_raw(),
                Self::PhpStatement(x) => x.into_raw(),
                Self::ScriptElement(x) => x.into_raw(),
                Self::StyleElement(x) => x.into_raw(),
                Self::Text(x) => x.into_raw(),
            }
        }
    }
    /**One of `{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | keyword | loop | parameter | php_statement | script_element | style_element | switch | text | verbatim}`:
- [`Comment`]
- [`Conditional`]
- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Element`]
- [`Entity`]
- [`ErroneousEndTag`]
- [`Keyword`]
- [`Loop`]
- [`Parameter`]
- [`PhpStatement`]
- [`ScriptElement`]
- [`StyleElement`]
- [`Switch`]
- [`Text`]
- [`Verbatim`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Anon220860789005696801182824190281800612819<'tree> {
        Comment(Comment<'tree>),
        Conditional(Conditional<'tree>),
        Directive(Directive<'tree>),
        DirectiveEnd(DirectiveEnd<'tree>),
        DirectiveStart(DirectiveStart<'tree>),
        Element(Element<'tree>),
        Entity(Entity<'tree>),
        ErroneousEndTag(ErroneousEndTag<'tree>),
        Keyword(Keyword<'tree>),
        Loop(Loop<'tree>),
        Parameter(Parameter<'tree>),
        PhpStatement(PhpStatement<'tree>),
        ScriptElement(ScriptElement<'tree>),
        StyleElement(StyleElement<'tree>),
        Switch(Switch<'tree>),
        Text(Text<'tree>),
        Verbatim(Verbatim<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Anon220860789005696801182824190281800612819<'tree> {
        ///Returns the node if it is of type `comment` ([`Comment`]), otherwise returns `None`
        #[inline]
        pub fn as_comment(self) -> ::std::option::Option<Comment<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Comment(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `conditional` ([`Conditional`]), otherwise returns `None`
        #[inline]
        pub fn as_conditional(self) -> ::std::option::Option<Conditional<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Conditional(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `directive` ([`Directive`]), otherwise returns `None`
        #[inline]
        pub fn as_directive(self) -> ::std::option::Option<Directive<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Directive(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `directive_end` ([`DirectiveEnd`]), otherwise returns `None`
        #[inline]
        pub fn as_directive_end(self) -> ::std::option::Option<DirectiveEnd<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DirectiveEnd(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `directive_start` ([`DirectiveStart`]), otherwise returns `None`
        #[inline]
        pub fn as_directive_start(self) -> ::std::option::Option<DirectiveStart<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DirectiveStart(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `element` ([`Element`]), otherwise returns `None`
        #[inline]
        pub fn as_element(self) -> ::std::option::Option<Element<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Element(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `entity` ([`Entity`]), otherwise returns `None`
        #[inline]
        pub fn as_entity(self) -> ::std::option::Option<Entity<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Entity(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `erroneous_end_tag` ([`ErroneousEndTag`]), otherwise returns `None`
        #[inline]
        pub fn as_erroneous_end_tag(
            self,
        ) -> ::std::option::Option<ErroneousEndTag<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ErroneousEndTag(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `keyword` ([`Keyword`]), otherwise returns `None`
        #[inline]
        pub fn as_keyword(self) -> ::std::option::Option<Keyword<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Keyword(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `loop` ([`Loop`]), otherwise returns `None`
        #[inline]
        pub fn as_loop(self) -> ::std::option::Option<Loop<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Loop(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `parameter` ([`Parameter`]), otherwise returns `None`
        #[inline]
        pub fn as_parameter(self) -> ::std::option::Option<Parameter<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Parameter(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `php_statement` ([`PhpStatement`]), otherwise returns `None`
        #[inline]
        pub fn as_php_statement(self) -> ::std::option::Option<PhpStatement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::PhpStatement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `script_element` ([`ScriptElement`]), otherwise returns `None`
        #[inline]
        pub fn as_script_element(self) -> ::std::option::Option<ScriptElement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ScriptElement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `style_element` ([`StyleElement`]), otherwise returns `None`
        #[inline]
        pub fn as_style_element(self) -> ::std::option::Option<StyleElement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::StyleElement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `switch` ([`Switch`]), otherwise returns `None`
        #[inline]
        pub fn as_switch(self) -> ::std::option::Option<Switch<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Switch(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `text` ([`Text`]), otherwise returns `None`
        #[inline]
        pub fn as_text(self) -> ::std::option::Option<Text<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Text(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `verbatim` ([`Verbatim`]), otherwise returns `None`
        #[inline]
        pub fn as_verbatim(self) -> ::std::option::Option<Verbatim<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Verbatim(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
    for Anon220860789005696801182824190281800612819<'tree> {
        type WithLifetime<'a> = Anon220860789005696801182824190281800612819<'a>;
        const KIND: &'static str = "{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | keyword | loop | parameter | php_statement | script_element | style_element | switch | text | verbatim}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "comment" => {
                    Ok(unsafe {
                        Self::Comment(
                            <Comment<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "conditional" => {
                    Ok(unsafe {
                        Self::Conditional(
                            <Conditional<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "directive" => {
                    Ok(unsafe {
                        Self::Directive(
                            <Directive<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "directive_end" => {
                    Ok(unsafe {
                        Self::DirectiveEnd(
                            <DirectiveEnd<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "directive_start" => {
                    Ok(unsafe {
                        Self::DirectiveStart(
                            <DirectiveStart<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "element" => {
                    Ok(unsafe {
                        Self::Element(
                            <Element<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "entity" => {
                    Ok(unsafe {
                        Self::Entity(
                            <Entity<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "erroneous_end_tag" => {
                    Ok(unsafe {
                        Self::ErroneousEndTag(
                            <ErroneousEndTag<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "keyword" => {
                    Ok(unsafe {
                        Self::Keyword(
                            <Keyword<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "loop" => {
                    Ok(unsafe {
                        Self::Loop(
                            <Loop<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "parameter" => {
                    Ok(unsafe {
                        Self::Parameter(
                            <Parameter<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "php_statement" => {
                    Ok(unsafe {
                        Self::PhpStatement(
                            <PhpStatement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "script_element" => {
                    Ok(unsafe {
                        Self::ScriptElement(
                            <ScriptElement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "style_element" => {
                    Ok(unsafe {
                        Self::StyleElement(
                            <StyleElement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "switch" => {
                    Ok(unsafe {
                        Self::Switch(
                            <Switch<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "text" => {
                    Ok(unsafe {
                        Self::Text(
                            <Text<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "verbatim" => {
                    Ok(unsafe {
                        Self::Verbatim(
                            <Verbatim<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Comment(x) => ::type_sitter::Node::raw(x),
                Self::Conditional(x) => ::type_sitter::Node::raw(x),
                Self::Directive(x) => ::type_sitter::Node::raw(x),
                Self::DirectiveEnd(x) => ::type_sitter::Node::raw(x),
                Self::DirectiveStart(x) => ::type_sitter::Node::raw(x),
                Self::Element(x) => ::type_sitter::Node::raw(x),
                Self::Entity(x) => ::type_sitter::Node::raw(x),
                Self::ErroneousEndTag(x) => ::type_sitter::Node::raw(x),
                Self::Keyword(x) => ::type_sitter::Node::raw(x),
                Self::Loop(x) => ::type_sitter::Node::raw(x),
                Self::Parameter(x) => ::type_sitter::Node::raw(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw(x),
                Self::ScriptElement(x) => ::type_sitter::Node::raw(x),
                Self::StyleElement(x) => ::type_sitter::Node::raw(x),
                Self::Switch(x) => ::type_sitter::Node::raw(x),
                Self::Text(x) => ::type_sitter::Node::raw(x),
                Self::Verbatim(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Comment(x) => ::type_sitter::Node::raw_mut(x),
                Self::Conditional(x) => ::type_sitter::Node::raw_mut(x),
                Self::Directive(x) => ::type_sitter::Node::raw_mut(x),
                Self::DirectiveEnd(x) => ::type_sitter::Node::raw_mut(x),
                Self::DirectiveStart(x) => ::type_sitter::Node::raw_mut(x),
                Self::Element(x) => ::type_sitter::Node::raw_mut(x),
                Self::Entity(x) => ::type_sitter::Node::raw_mut(x),
                Self::ErroneousEndTag(x) => ::type_sitter::Node::raw_mut(x),
                Self::Keyword(x) => ::type_sitter::Node::raw_mut(x),
                Self::Loop(x) => ::type_sitter::Node::raw_mut(x),
                Self::Parameter(x) => ::type_sitter::Node::raw_mut(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw_mut(x),
                Self::ScriptElement(x) => ::type_sitter::Node::raw_mut(x),
                Self::StyleElement(x) => ::type_sitter::Node::raw_mut(x),
                Self::Switch(x) => ::type_sitter::Node::raw_mut(x),
                Self::Text(x) => ::type_sitter::Node::raw_mut(x),
                Self::Verbatim(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Comment(x) => x.into_raw(),
                Self::Conditional(x) => x.into_raw(),
                Self::Directive(x) => x.into_raw(),
                Self::DirectiveEnd(x) => x.into_raw(),
                Self::DirectiveStart(x) => x.into_raw(),
                Self::Element(x) => x.into_raw(),
                Self::Entity(x) => x.into_raw(),
                Self::ErroneousEndTag(x) => x.into_raw(),
                Self::Keyword(x) => x.into_raw(),
                Self::Loop(x) => x.into_raw(),
                Self::Parameter(x) => x.into_raw(),
                Self::PhpStatement(x) => x.into_raw(),
                Self::ScriptElement(x) => x.into_raw(),
                Self::StyleElement(x) => x.into_raw(),
                Self::Switch(x) => x.into_raw(),
                Self::Text(x) => x.into_raw(),
                Self::Verbatim(x) => x.into_raw(),
            }
        }
    }
    /**One of `{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | keyword | livewire | loop | parameter | php_statement | script_element | stack | style_element | switch | text | verbatim}`:
- [`Comment`]
- [`Conditional`]
- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Element`]
- [`Entity`]
- [`ErroneousEndTag`]
- [`Keyword`]
- [`Livewire`]
- [`Loop`]
- [`Parameter`]
- [`PhpStatement`]
- [`ScriptElement`]
- [`Stack`]
- [`StyleElement`]
- [`Switch`]
- [`Text`]
- [`Verbatim`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Anon243545065307733425234527997183447807845<'tree> {
        Comment(Comment<'tree>),
        Conditional(Conditional<'tree>),
        Directive(Directive<'tree>),
        DirectiveEnd(DirectiveEnd<'tree>),
        DirectiveStart(DirectiveStart<'tree>),
        Element(Element<'tree>),
        Entity(Entity<'tree>),
        ErroneousEndTag(ErroneousEndTag<'tree>),
        Keyword(Keyword<'tree>),
        Livewire(Livewire<'tree>),
        Loop(Loop<'tree>),
        Parameter(Parameter<'tree>),
        PhpStatement(PhpStatement<'tree>),
        ScriptElement(ScriptElement<'tree>),
        Stack(Stack<'tree>),
        StyleElement(StyleElement<'tree>),
        Switch(Switch<'tree>),
        Text(Text<'tree>),
        Verbatim(Verbatim<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Anon243545065307733425234527997183447807845<'tree> {
        ///Returns the node if it is of type `comment` ([`Comment`]), otherwise returns `None`
        #[inline]
        pub fn as_comment(self) -> ::std::option::Option<Comment<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Comment(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `conditional` ([`Conditional`]), otherwise returns `None`
        #[inline]
        pub fn as_conditional(self) -> ::std::option::Option<Conditional<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Conditional(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `directive` ([`Directive`]), otherwise returns `None`
        #[inline]
        pub fn as_directive(self) -> ::std::option::Option<Directive<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Directive(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `directive_end` ([`DirectiveEnd`]), otherwise returns `None`
        #[inline]
        pub fn as_directive_end(self) -> ::std::option::Option<DirectiveEnd<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DirectiveEnd(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `directive_start` ([`DirectiveStart`]), otherwise returns `None`
        #[inline]
        pub fn as_directive_start(self) -> ::std::option::Option<DirectiveStart<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DirectiveStart(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `element` ([`Element`]), otherwise returns `None`
        #[inline]
        pub fn as_element(self) -> ::std::option::Option<Element<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Element(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `entity` ([`Entity`]), otherwise returns `None`
        #[inline]
        pub fn as_entity(self) -> ::std::option::Option<Entity<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Entity(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `erroneous_end_tag` ([`ErroneousEndTag`]), otherwise returns `None`
        #[inline]
        pub fn as_erroneous_end_tag(
            self,
        ) -> ::std::option::Option<ErroneousEndTag<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ErroneousEndTag(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `keyword` ([`Keyword`]), otherwise returns `None`
        #[inline]
        pub fn as_keyword(self) -> ::std::option::Option<Keyword<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Keyword(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `livewire` ([`Livewire`]), otherwise returns `None`
        #[inline]
        pub fn as_livewire(self) -> ::std::option::Option<Livewire<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Livewire(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `loop` ([`Loop`]), otherwise returns `None`
        #[inline]
        pub fn as_loop(self) -> ::std::option::Option<Loop<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Loop(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `parameter` ([`Parameter`]), otherwise returns `None`
        #[inline]
        pub fn as_parameter(self) -> ::std::option::Option<Parameter<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Parameter(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `php_statement` ([`PhpStatement`]), otherwise returns `None`
        #[inline]
        pub fn as_php_statement(self) -> ::std::option::Option<PhpStatement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::PhpStatement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `script_element` ([`ScriptElement`]), otherwise returns `None`
        #[inline]
        pub fn as_script_element(self) -> ::std::option::Option<ScriptElement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ScriptElement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `stack` ([`Stack`]), otherwise returns `None`
        #[inline]
        pub fn as_stack(self) -> ::std::option::Option<Stack<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Stack(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `style_element` ([`StyleElement`]), otherwise returns `None`
        #[inline]
        pub fn as_style_element(self) -> ::std::option::Option<StyleElement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::StyleElement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `switch` ([`Switch`]), otherwise returns `None`
        #[inline]
        pub fn as_switch(self) -> ::std::option::Option<Switch<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Switch(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `text` ([`Text`]), otherwise returns `None`
        #[inline]
        pub fn as_text(self) -> ::std::option::Option<Text<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Text(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `verbatim` ([`Verbatim`]), otherwise returns `None`
        #[inline]
        pub fn as_verbatim(self) -> ::std::option::Option<Verbatim<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Verbatim(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
    for Anon243545065307733425234527997183447807845<'tree> {
        type WithLifetime<'a> = Anon243545065307733425234527997183447807845<'a>;
        const KIND: &'static str = "{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | keyword | livewire | loop | parameter | php_statement | script_element | stack | style_element | switch | text | verbatim}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "comment" => {
                    Ok(unsafe {
                        Self::Comment(
                            <Comment<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "conditional" => {
                    Ok(unsafe {
                        Self::Conditional(
                            <Conditional<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "directive" => {
                    Ok(unsafe {
                        Self::Directive(
                            <Directive<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "directive_end" => {
                    Ok(unsafe {
                        Self::DirectiveEnd(
                            <DirectiveEnd<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "directive_start" => {
                    Ok(unsafe {
                        Self::DirectiveStart(
                            <DirectiveStart<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "element" => {
                    Ok(unsafe {
                        Self::Element(
                            <Element<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "entity" => {
                    Ok(unsafe {
                        Self::Entity(
                            <Entity<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "erroneous_end_tag" => {
                    Ok(unsafe {
                        Self::ErroneousEndTag(
                            <ErroneousEndTag<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "keyword" => {
                    Ok(unsafe {
                        Self::Keyword(
                            <Keyword<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "livewire" => {
                    Ok(unsafe {
                        Self::Livewire(
                            <Livewire<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "loop" => {
                    Ok(unsafe {
                        Self::Loop(
                            <Loop<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "parameter" => {
                    Ok(unsafe {
                        Self::Parameter(
                            <Parameter<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "php_statement" => {
                    Ok(unsafe {
                        Self::PhpStatement(
                            <PhpStatement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "script_element" => {
                    Ok(unsafe {
                        Self::ScriptElement(
                            <ScriptElement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "stack" => {
                    Ok(unsafe {
                        Self::Stack(
                            <Stack<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "style_element" => {
                    Ok(unsafe {
                        Self::StyleElement(
                            <StyleElement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "switch" => {
                    Ok(unsafe {
                        Self::Switch(
                            <Switch<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "text" => {
                    Ok(unsafe {
                        Self::Text(
                            <Text<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "verbatim" => {
                    Ok(unsafe {
                        Self::Verbatim(
                            <Verbatim<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Comment(x) => ::type_sitter::Node::raw(x),
                Self::Conditional(x) => ::type_sitter::Node::raw(x),
                Self::Directive(x) => ::type_sitter::Node::raw(x),
                Self::DirectiveEnd(x) => ::type_sitter::Node::raw(x),
                Self::DirectiveStart(x) => ::type_sitter::Node::raw(x),
                Self::Element(x) => ::type_sitter::Node::raw(x),
                Self::Entity(x) => ::type_sitter::Node::raw(x),
                Self::ErroneousEndTag(x) => ::type_sitter::Node::raw(x),
                Self::Keyword(x) => ::type_sitter::Node::raw(x),
                Self::Livewire(x) => ::type_sitter::Node::raw(x),
                Self::Loop(x) => ::type_sitter::Node::raw(x),
                Self::Parameter(x) => ::type_sitter::Node::raw(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw(x),
                Self::ScriptElement(x) => ::type_sitter::Node::raw(x),
                Self::Stack(x) => ::type_sitter::Node::raw(x),
                Self::StyleElement(x) => ::type_sitter::Node::raw(x),
                Self::Switch(x) => ::type_sitter::Node::raw(x),
                Self::Text(x) => ::type_sitter::Node::raw(x),
                Self::Verbatim(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Comment(x) => ::type_sitter::Node::raw_mut(x),
                Self::Conditional(x) => ::type_sitter::Node::raw_mut(x),
                Self::Directive(x) => ::type_sitter::Node::raw_mut(x),
                Self::DirectiveEnd(x) => ::type_sitter::Node::raw_mut(x),
                Self::DirectiveStart(x) => ::type_sitter::Node::raw_mut(x),
                Self::Element(x) => ::type_sitter::Node::raw_mut(x),
                Self::Entity(x) => ::type_sitter::Node::raw_mut(x),
                Self::ErroneousEndTag(x) => ::type_sitter::Node::raw_mut(x),
                Self::Keyword(x) => ::type_sitter::Node::raw_mut(x),
                Self::Livewire(x) => ::type_sitter::Node::raw_mut(x),
                Self::Loop(x) => ::type_sitter::Node::raw_mut(x),
                Self::Parameter(x) => ::type_sitter::Node::raw_mut(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw_mut(x),
                Self::ScriptElement(x) => ::type_sitter::Node::raw_mut(x),
                Self::Stack(x) => ::type_sitter::Node::raw_mut(x),
                Self::StyleElement(x) => ::type_sitter::Node::raw_mut(x),
                Self::Switch(x) => ::type_sitter::Node::raw_mut(x),
                Self::Text(x) => ::type_sitter::Node::raw_mut(x),
                Self::Verbatim(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Comment(x) => x.into_raw(),
                Self::Conditional(x) => x.into_raw(),
                Self::Directive(x) => x.into_raw(),
                Self::DirectiveEnd(x) => x.into_raw(),
                Self::DirectiveStart(x) => x.into_raw(),
                Self::Element(x) => x.into_raw(),
                Self::Entity(x) => x.into_raw(),
                Self::ErroneousEndTag(x) => x.into_raw(),
                Self::Keyword(x) => x.into_raw(),
                Self::Livewire(x) => x.into_raw(),
                Self::Loop(x) => x.into_raw(),
                Self::Parameter(x) => x.into_raw(),
                Self::PhpStatement(x) => x.into_raw(),
                Self::ScriptElement(x) => x.into_raw(),
                Self::Stack(x) => x.into_raw(),
                Self::StyleElement(x) => x.into_raw(),
                Self::Switch(x) => x.into_raw(),
                Self::Text(x) => x.into_raw(),
                Self::Verbatim(x) => x.into_raw(),
            }
        }
    }
    /**One of `{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | fragment | keyword | loop | once | parameter | php_statement | script_element | section | stack | style_element | switch | text | verbatim}`:
- [`Comment`]
- [`Conditional`]
- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Element`]
- [`Entity`]
- [`ErroneousEndTag`]
- [`Fragment`]
- [`Keyword`]
- [`Loop`]
- [`Once`]
- [`Parameter`]
- [`PhpStatement`]
- [`ScriptElement`]
- [`Section`]
- [`Stack`]
- [`StyleElement`]
- [`Switch`]
- [`Text`]
- [`Verbatim`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Anon296218948099219678605060842541820530786<'tree> {
        Comment(Comment<'tree>),
        Conditional(Conditional<'tree>),
        Directive(Directive<'tree>),
        DirectiveEnd(DirectiveEnd<'tree>),
        DirectiveStart(DirectiveStart<'tree>),
        Element(Element<'tree>),
        Entity(Entity<'tree>),
        ErroneousEndTag(ErroneousEndTag<'tree>),
        Fragment(Fragment<'tree>),
        Keyword(Keyword<'tree>),
        Loop(Loop<'tree>),
        Once(Once<'tree>),
        Parameter(Parameter<'tree>),
        PhpStatement(PhpStatement<'tree>),
        ScriptElement(ScriptElement<'tree>),
        Section(Section<'tree>),
        Stack(Stack<'tree>),
        StyleElement(StyleElement<'tree>),
        Switch(Switch<'tree>),
        Text(Text<'tree>),
        Verbatim(Verbatim<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Anon296218948099219678605060842541820530786<'tree> {
        ///Returns the node if it is of type `comment` ([`Comment`]), otherwise returns `None`
        #[inline]
        pub fn as_comment(self) -> ::std::option::Option<Comment<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Comment(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `conditional` ([`Conditional`]), otherwise returns `None`
        #[inline]
        pub fn as_conditional(self) -> ::std::option::Option<Conditional<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Conditional(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `directive` ([`Directive`]), otherwise returns `None`
        #[inline]
        pub fn as_directive(self) -> ::std::option::Option<Directive<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Directive(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `directive_end` ([`DirectiveEnd`]), otherwise returns `None`
        #[inline]
        pub fn as_directive_end(self) -> ::std::option::Option<DirectiveEnd<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DirectiveEnd(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `directive_start` ([`DirectiveStart`]), otherwise returns `None`
        #[inline]
        pub fn as_directive_start(self) -> ::std::option::Option<DirectiveStart<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DirectiveStart(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `element` ([`Element`]), otherwise returns `None`
        #[inline]
        pub fn as_element(self) -> ::std::option::Option<Element<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Element(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `entity` ([`Entity`]), otherwise returns `None`
        #[inline]
        pub fn as_entity(self) -> ::std::option::Option<Entity<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Entity(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `erroneous_end_tag` ([`ErroneousEndTag`]), otherwise returns `None`
        #[inline]
        pub fn as_erroneous_end_tag(
            self,
        ) -> ::std::option::Option<ErroneousEndTag<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ErroneousEndTag(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `fragment` ([`Fragment`]), otherwise returns `None`
        #[inline]
        pub fn as_fragment(self) -> ::std::option::Option<Fragment<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Fragment(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `keyword` ([`Keyword`]), otherwise returns `None`
        #[inline]
        pub fn as_keyword(self) -> ::std::option::Option<Keyword<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Keyword(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `loop` ([`Loop`]), otherwise returns `None`
        #[inline]
        pub fn as_loop(self) -> ::std::option::Option<Loop<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Loop(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `once` ([`Once`]), otherwise returns `None`
        #[inline]
        pub fn as_once(self) -> ::std::option::Option<Once<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Once(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `parameter` ([`Parameter`]), otherwise returns `None`
        #[inline]
        pub fn as_parameter(self) -> ::std::option::Option<Parameter<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Parameter(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `php_statement` ([`PhpStatement`]), otherwise returns `None`
        #[inline]
        pub fn as_php_statement(self) -> ::std::option::Option<PhpStatement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::PhpStatement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `script_element` ([`ScriptElement`]), otherwise returns `None`
        #[inline]
        pub fn as_script_element(self) -> ::std::option::Option<ScriptElement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ScriptElement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `section` ([`Section`]), otherwise returns `None`
        #[inline]
        pub fn as_section(self) -> ::std::option::Option<Section<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Section(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `stack` ([`Stack`]), otherwise returns `None`
        #[inline]
        pub fn as_stack(self) -> ::std::option::Option<Stack<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Stack(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `style_element` ([`StyleElement`]), otherwise returns `None`
        #[inline]
        pub fn as_style_element(self) -> ::std::option::Option<StyleElement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::StyleElement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `switch` ([`Switch`]), otherwise returns `None`
        #[inline]
        pub fn as_switch(self) -> ::std::option::Option<Switch<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Switch(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `text` ([`Text`]), otherwise returns `None`
        #[inline]
        pub fn as_text(self) -> ::std::option::Option<Text<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Text(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `verbatim` ([`Verbatim`]), otherwise returns `None`
        #[inline]
        pub fn as_verbatim(self) -> ::std::option::Option<Verbatim<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Verbatim(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
    for Anon296218948099219678605060842541820530786<'tree> {
        type WithLifetime<'a> = Anon296218948099219678605060842541820530786<'a>;
        const KIND: &'static str = "{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | fragment | keyword | loop | once | parameter | php_statement | script_element | section | stack | style_element | switch | text | verbatim}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "comment" => {
                    Ok(unsafe {
                        Self::Comment(
                            <Comment<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "conditional" => {
                    Ok(unsafe {
                        Self::Conditional(
                            <Conditional<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "directive" => {
                    Ok(unsafe {
                        Self::Directive(
                            <Directive<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "directive_end" => {
                    Ok(unsafe {
                        Self::DirectiveEnd(
                            <DirectiveEnd<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "directive_start" => {
                    Ok(unsafe {
                        Self::DirectiveStart(
                            <DirectiveStart<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "element" => {
                    Ok(unsafe {
                        Self::Element(
                            <Element<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "entity" => {
                    Ok(unsafe {
                        Self::Entity(
                            <Entity<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "erroneous_end_tag" => {
                    Ok(unsafe {
                        Self::ErroneousEndTag(
                            <ErroneousEndTag<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "fragment" => {
                    Ok(unsafe {
                        Self::Fragment(
                            <Fragment<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "keyword" => {
                    Ok(unsafe {
                        Self::Keyword(
                            <Keyword<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "loop" => {
                    Ok(unsafe {
                        Self::Loop(
                            <Loop<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "once" => {
                    Ok(unsafe {
                        Self::Once(
                            <Once<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "parameter" => {
                    Ok(unsafe {
                        Self::Parameter(
                            <Parameter<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "php_statement" => {
                    Ok(unsafe {
                        Self::PhpStatement(
                            <PhpStatement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "script_element" => {
                    Ok(unsafe {
                        Self::ScriptElement(
                            <ScriptElement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "section" => {
                    Ok(unsafe {
                        Self::Section(
                            <Section<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "stack" => {
                    Ok(unsafe {
                        Self::Stack(
                            <Stack<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "style_element" => {
                    Ok(unsafe {
                        Self::StyleElement(
                            <StyleElement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "switch" => {
                    Ok(unsafe {
                        Self::Switch(
                            <Switch<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "text" => {
                    Ok(unsafe {
                        Self::Text(
                            <Text<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "verbatim" => {
                    Ok(unsafe {
                        Self::Verbatim(
                            <Verbatim<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Comment(x) => ::type_sitter::Node::raw(x),
                Self::Conditional(x) => ::type_sitter::Node::raw(x),
                Self::Directive(x) => ::type_sitter::Node::raw(x),
                Self::DirectiveEnd(x) => ::type_sitter::Node::raw(x),
                Self::DirectiveStart(x) => ::type_sitter::Node::raw(x),
                Self::Element(x) => ::type_sitter::Node::raw(x),
                Self::Entity(x) => ::type_sitter::Node::raw(x),
                Self::ErroneousEndTag(x) => ::type_sitter::Node::raw(x),
                Self::Fragment(x) => ::type_sitter::Node::raw(x),
                Self::Keyword(x) => ::type_sitter::Node::raw(x),
                Self::Loop(x) => ::type_sitter::Node::raw(x),
                Self::Once(x) => ::type_sitter::Node::raw(x),
                Self::Parameter(x) => ::type_sitter::Node::raw(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw(x),
                Self::ScriptElement(x) => ::type_sitter::Node::raw(x),
                Self::Section(x) => ::type_sitter::Node::raw(x),
                Self::Stack(x) => ::type_sitter::Node::raw(x),
                Self::StyleElement(x) => ::type_sitter::Node::raw(x),
                Self::Switch(x) => ::type_sitter::Node::raw(x),
                Self::Text(x) => ::type_sitter::Node::raw(x),
                Self::Verbatim(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Comment(x) => ::type_sitter::Node::raw_mut(x),
                Self::Conditional(x) => ::type_sitter::Node::raw_mut(x),
                Self::Directive(x) => ::type_sitter::Node::raw_mut(x),
                Self::DirectiveEnd(x) => ::type_sitter::Node::raw_mut(x),
                Self::DirectiveStart(x) => ::type_sitter::Node::raw_mut(x),
                Self::Element(x) => ::type_sitter::Node::raw_mut(x),
                Self::Entity(x) => ::type_sitter::Node::raw_mut(x),
                Self::ErroneousEndTag(x) => ::type_sitter::Node::raw_mut(x),
                Self::Fragment(x) => ::type_sitter::Node::raw_mut(x),
                Self::Keyword(x) => ::type_sitter::Node::raw_mut(x),
                Self::Loop(x) => ::type_sitter::Node::raw_mut(x),
                Self::Once(x) => ::type_sitter::Node::raw_mut(x),
                Self::Parameter(x) => ::type_sitter::Node::raw_mut(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw_mut(x),
                Self::ScriptElement(x) => ::type_sitter::Node::raw_mut(x),
                Self::Section(x) => ::type_sitter::Node::raw_mut(x),
                Self::Stack(x) => ::type_sitter::Node::raw_mut(x),
                Self::StyleElement(x) => ::type_sitter::Node::raw_mut(x),
                Self::Switch(x) => ::type_sitter::Node::raw_mut(x),
                Self::Text(x) => ::type_sitter::Node::raw_mut(x),
                Self::Verbatim(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Comment(x) => x.into_raw(),
                Self::Conditional(x) => x.into_raw(),
                Self::Directive(x) => x.into_raw(),
                Self::DirectiveEnd(x) => x.into_raw(),
                Self::DirectiveStart(x) => x.into_raw(),
                Self::Element(x) => x.into_raw(),
                Self::Entity(x) => x.into_raw(),
                Self::ErroneousEndTag(x) => x.into_raw(),
                Self::Fragment(x) => x.into_raw(),
                Self::Keyword(x) => x.into_raw(),
                Self::Loop(x) => x.into_raw(),
                Self::Once(x) => x.into_raw(),
                Self::Parameter(x) => x.into_raw(),
                Self::PhpStatement(x) => x.into_raw(),
                Self::ScriptElement(x) => x.into_raw(),
                Self::Section(x) => x.into_raw(),
                Self::Stack(x) => x.into_raw(),
                Self::StyleElement(x) => x.into_raw(),
                Self::Switch(x) => x.into_raw(),
                Self::Text(x) => x.into_raw(),
                Self::Verbatim(x) => x.into_raw(),
            }
        }
    }
    /**One of `{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | keyword | livewire | loop | parameter | php_statement | script_element | style_element | switch | text}`:
- [`Comment`]
- [`Conditional`]
- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Element`]
- [`Entity`]
- [`ErroneousEndTag`]
- [`Keyword`]
- [`Livewire`]
- [`Loop`]
- [`Parameter`]
- [`PhpStatement`]
- [`ScriptElement`]
- [`StyleElement`]
- [`Switch`]
- [`Text`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Anon318641722947100924141238363717069648819<'tree> {
        Comment(Comment<'tree>),
        Conditional(Conditional<'tree>),
        Directive(Directive<'tree>),
        DirectiveEnd(DirectiveEnd<'tree>),
        DirectiveStart(DirectiveStart<'tree>),
        Element(Element<'tree>),
        Entity(Entity<'tree>),
        ErroneousEndTag(ErroneousEndTag<'tree>),
        Keyword(Keyword<'tree>),
        Livewire(Livewire<'tree>),
        Loop(Loop<'tree>),
        Parameter(Parameter<'tree>),
        PhpStatement(PhpStatement<'tree>),
        ScriptElement(ScriptElement<'tree>),
        StyleElement(StyleElement<'tree>),
        Switch(Switch<'tree>),
        Text(Text<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Anon318641722947100924141238363717069648819<'tree> {
        ///Returns the node if it is of type `comment` ([`Comment`]), otherwise returns `None`
        #[inline]
        pub fn as_comment(self) -> ::std::option::Option<Comment<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Comment(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `conditional` ([`Conditional`]), otherwise returns `None`
        #[inline]
        pub fn as_conditional(self) -> ::std::option::Option<Conditional<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Conditional(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `directive` ([`Directive`]), otherwise returns `None`
        #[inline]
        pub fn as_directive(self) -> ::std::option::Option<Directive<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Directive(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `directive_end` ([`DirectiveEnd`]), otherwise returns `None`
        #[inline]
        pub fn as_directive_end(self) -> ::std::option::Option<DirectiveEnd<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DirectiveEnd(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `directive_start` ([`DirectiveStart`]), otherwise returns `None`
        #[inline]
        pub fn as_directive_start(self) -> ::std::option::Option<DirectiveStart<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DirectiveStart(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `element` ([`Element`]), otherwise returns `None`
        #[inline]
        pub fn as_element(self) -> ::std::option::Option<Element<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Element(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `entity` ([`Entity`]), otherwise returns `None`
        #[inline]
        pub fn as_entity(self) -> ::std::option::Option<Entity<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Entity(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `erroneous_end_tag` ([`ErroneousEndTag`]), otherwise returns `None`
        #[inline]
        pub fn as_erroneous_end_tag(
            self,
        ) -> ::std::option::Option<ErroneousEndTag<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ErroneousEndTag(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `keyword` ([`Keyword`]), otherwise returns `None`
        #[inline]
        pub fn as_keyword(self) -> ::std::option::Option<Keyword<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Keyword(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `livewire` ([`Livewire`]), otherwise returns `None`
        #[inline]
        pub fn as_livewire(self) -> ::std::option::Option<Livewire<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Livewire(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `loop` ([`Loop`]), otherwise returns `None`
        #[inline]
        pub fn as_loop(self) -> ::std::option::Option<Loop<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Loop(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `parameter` ([`Parameter`]), otherwise returns `None`
        #[inline]
        pub fn as_parameter(self) -> ::std::option::Option<Parameter<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Parameter(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `php_statement` ([`PhpStatement`]), otherwise returns `None`
        #[inline]
        pub fn as_php_statement(self) -> ::std::option::Option<PhpStatement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::PhpStatement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `script_element` ([`ScriptElement`]), otherwise returns `None`
        #[inline]
        pub fn as_script_element(self) -> ::std::option::Option<ScriptElement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ScriptElement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `style_element` ([`StyleElement`]), otherwise returns `None`
        #[inline]
        pub fn as_style_element(self) -> ::std::option::Option<StyleElement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::StyleElement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `switch` ([`Switch`]), otherwise returns `None`
        #[inline]
        pub fn as_switch(self) -> ::std::option::Option<Switch<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Switch(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `text` ([`Text`]), otherwise returns `None`
        #[inline]
        pub fn as_text(self) -> ::std::option::Option<Text<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Text(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
    for Anon318641722947100924141238363717069648819<'tree> {
        type WithLifetime<'a> = Anon318641722947100924141238363717069648819<'a>;
        const KIND: &'static str = "{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | keyword | livewire | loop | parameter | php_statement | script_element | style_element | switch | text}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "comment" => {
                    Ok(unsafe {
                        Self::Comment(
                            <Comment<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "conditional" => {
                    Ok(unsafe {
                        Self::Conditional(
                            <Conditional<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "directive" => {
                    Ok(unsafe {
                        Self::Directive(
                            <Directive<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "directive_end" => {
                    Ok(unsafe {
                        Self::DirectiveEnd(
                            <DirectiveEnd<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "directive_start" => {
                    Ok(unsafe {
                        Self::DirectiveStart(
                            <DirectiveStart<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "element" => {
                    Ok(unsafe {
                        Self::Element(
                            <Element<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "entity" => {
                    Ok(unsafe {
                        Self::Entity(
                            <Entity<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "erroneous_end_tag" => {
                    Ok(unsafe {
                        Self::ErroneousEndTag(
                            <ErroneousEndTag<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "keyword" => {
                    Ok(unsafe {
                        Self::Keyword(
                            <Keyword<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "livewire" => {
                    Ok(unsafe {
                        Self::Livewire(
                            <Livewire<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "loop" => {
                    Ok(unsafe {
                        Self::Loop(
                            <Loop<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "parameter" => {
                    Ok(unsafe {
                        Self::Parameter(
                            <Parameter<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "php_statement" => {
                    Ok(unsafe {
                        Self::PhpStatement(
                            <PhpStatement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "script_element" => {
                    Ok(unsafe {
                        Self::ScriptElement(
                            <ScriptElement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "style_element" => {
                    Ok(unsafe {
                        Self::StyleElement(
                            <StyleElement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "switch" => {
                    Ok(unsafe {
                        Self::Switch(
                            <Switch<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "text" => {
                    Ok(unsafe {
                        Self::Text(
                            <Text<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Comment(x) => ::type_sitter::Node::raw(x),
                Self::Conditional(x) => ::type_sitter::Node::raw(x),
                Self::Directive(x) => ::type_sitter::Node::raw(x),
                Self::DirectiveEnd(x) => ::type_sitter::Node::raw(x),
                Self::DirectiveStart(x) => ::type_sitter::Node::raw(x),
                Self::Element(x) => ::type_sitter::Node::raw(x),
                Self::Entity(x) => ::type_sitter::Node::raw(x),
                Self::ErroneousEndTag(x) => ::type_sitter::Node::raw(x),
                Self::Keyword(x) => ::type_sitter::Node::raw(x),
                Self::Livewire(x) => ::type_sitter::Node::raw(x),
                Self::Loop(x) => ::type_sitter::Node::raw(x),
                Self::Parameter(x) => ::type_sitter::Node::raw(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw(x),
                Self::ScriptElement(x) => ::type_sitter::Node::raw(x),
                Self::StyleElement(x) => ::type_sitter::Node::raw(x),
                Self::Switch(x) => ::type_sitter::Node::raw(x),
                Self::Text(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Comment(x) => ::type_sitter::Node::raw_mut(x),
                Self::Conditional(x) => ::type_sitter::Node::raw_mut(x),
                Self::Directive(x) => ::type_sitter::Node::raw_mut(x),
                Self::DirectiveEnd(x) => ::type_sitter::Node::raw_mut(x),
                Self::DirectiveStart(x) => ::type_sitter::Node::raw_mut(x),
                Self::Element(x) => ::type_sitter::Node::raw_mut(x),
                Self::Entity(x) => ::type_sitter::Node::raw_mut(x),
                Self::ErroneousEndTag(x) => ::type_sitter::Node::raw_mut(x),
                Self::Keyword(x) => ::type_sitter::Node::raw_mut(x),
                Self::Livewire(x) => ::type_sitter::Node::raw_mut(x),
                Self::Loop(x) => ::type_sitter::Node::raw_mut(x),
                Self::Parameter(x) => ::type_sitter::Node::raw_mut(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw_mut(x),
                Self::ScriptElement(x) => ::type_sitter::Node::raw_mut(x),
                Self::StyleElement(x) => ::type_sitter::Node::raw_mut(x),
                Self::Switch(x) => ::type_sitter::Node::raw_mut(x),
                Self::Text(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Comment(x) => x.into_raw(),
                Self::Conditional(x) => x.into_raw(),
                Self::Directive(x) => x.into_raw(),
                Self::DirectiveEnd(x) => x.into_raw(),
                Self::DirectiveStart(x) => x.into_raw(),
                Self::Element(x) => x.into_raw(),
                Self::Entity(x) => x.into_raw(),
                Self::ErroneousEndTag(x) => x.into_raw(),
                Self::Keyword(x) => x.into_raw(),
                Self::Livewire(x) => x.into_raw(),
                Self::Loop(x) => x.into_raw(),
                Self::Parameter(x) => x.into_raw(),
                Self::PhpStatement(x) => x.into_raw(),
                Self::ScriptElement(x) => x.into_raw(),
                Self::StyleElement(x) => x.into_raw(),
                Self::Switch(x) => x.into_raw(),
                Self::Text(x) => x.into_raw(),
            }
        }
    }
    /**One of `{comment | directive | directive_end | directive_start | element | entity | erroneous_end_tag | keyword | parameter | php_statement | script_element | style_element | switch | text}`:
- [`Comment`]
- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Element`]
- [`Entity`]
- [`ErroneousEndTag`]
- [`Keyword`]
- [`Parameter`]
- [`PhpStatement`]
- [`ScriptElement`]
- [`StyleElement`]
- [`Switch`]
- [`Text`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Anon34450808297487433971579096753333213753<'tree> {
        Comment(Comment<'tree>),
        Directive(Directive<'tree>),
        DirectiveEnd(DirectiveEnd<'tree>),
        DirectiveStart(DirectiveStart<'tree>),
        Element(Element<'tree>),
        Entity(Entity<'tree>),
        ErroneousEndTag(ErroneousEndTag<'tree>),
        Keyword(Keyword<'tree>),
        Parameter(Parameter<'tree>),
        PhpStatement(PhpStatement<'tree>),
        ScriptElement(ScriptElement<'tree>),
        StyleElement(StyleElement<'tree>),
        Switch(Switch<'tree>),
        Text(Text<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Anon34450808297487433971579096753333213753<'tree> {
        ///Returns the node if it is of type `comment` ([`Comment`]), otherwise returns `None`
        #[inline]
        pub fn as_comment(self) -> ::std::option::Option<Comment<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Comment(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `directive` ([`Directive`]), otherwise returns `None`
        #[inline]
        pub fn as_directive(self) -> ::std::option::Option<Directive<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Directive(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `directive_end` ([`DirectiveEnd`]), otherwise returns `None`
        #[inline]
        pub fn as_directive_end(self) -> ::std::option::Option<DirectiveEnd<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DirectiveEnd(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `directive_start` ([`DirectiveStart`]), otherwise returns `None`
        #[inline]
        pub fn as_directive_start(self) -> ::std::option::Option<DirectiveStart<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DirectiveStart(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `element` ([`Element`]), otherwise returns `None`
        #[inline]
        pub fn as_element(self) -> ::std::option::Option<Element<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Element(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `entity` ([`Entity`]), otherwise returns `None`
        #[inline]
        pub fn as_entity(self) -> ::std::option::Option<Entity<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Entity(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `erroneous_end_tag` ([`ErroneousEndTag`]), otherwise returns `None`
        #[inline]
        pub fn as_erroneous_end_tag(
            self,
        ) -> ::std::option::Option<ErroneousEndTag<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ErroneousEndTag(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `keyword` ([`Keyword`]), otherwise returns `None`
        #[inline]
        pub fn as_keyword(self) -> ::std::option::Option<Keyword<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Keyword(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `parameter` ([`Parameter`]), otherwise returns `None`
        #[inline]
        pub fn as_parameter(self) -> ::std::option::Option<Parameter<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Parameter(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `php_statement` ([`PhpStatement`]), otherwise returns `None`
        #[inline]
        pub fn as_php_statement(self) -> ::std::option::Option<PhpStatement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::PhpStatement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `script_element` ([`ScriptElement`]), otherwise returns `None`
        #[inline]
        pub fn as_script_element(self) -> ::std::option::Option<ScriptElement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ScriptElement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `style_element` ([`StyleElement`]), otherwise returns `None`
        #[inline]
        pub fn as_style_element(self) -> ::std::option::Option<StyleElement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::StyleElement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `switch` ([`Switch`]), otherwise returns `None`
        #[inline]
        pub fn as_switch(self) -> ::std::option::Option<Switch<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Switch(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `text` ([`Text`]), otherwise returns `None`
        #[inline]
        pub fn as_text(self) -> ::std::option::Option<Text<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Text(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
    for Anon34450808297487433971579096753333213753<'tree> {
        type WithLifetime<'a> = Anon34450808297487433971579096753333213753<'a>;
        const KIND: &'static str = "{comment | directive | directive_end | directive_start | element | entity | erroneous_end_tag | keyword | parameter | php_statement | script_element | style_element | switch | text}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "comment" => {
                    Ok(unsafe {
                        Self::Comment(
                            <Comment<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "directive" => {
                    Ok(unsafe {
                        Self::Directive(
                            <Directive<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "directive_end" => {
                    Ok(unsafe {
                        Self::DirectiveEnd(
                            <DirectiveEnd<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "directive_start" => {
                    Ok(unsafe {
                        Self::DirectiveStart(
                            <DirectiveStart<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "element" => {
                    Ok(unsafe {
                        Self::Element(
                            <Element<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "entity" => {
                    Ok(unsafe {
                        Self::Entity(
                            <Entity<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "erroneous_end_tag" => {
                    Ok(unsafe {
                        Self::ErroneousEndTag(
                            <ErroneousEndTag<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "keyword" => {
                    Ok(unsafe {
                        Self::Keyword(
                            <Keyword<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "parameter" => {
                    Ok(unsafe {
                        Self::Parameter(
                            <Parameter<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "php_statement" => {
                    Ok(unsafe {
                        Self::PhpStatement(
                            <PhpStatement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "script_element" => {
                    Ok(unsafe {
                        Self::ScriptElement(
                            <ScriptElement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "style_element" => {
                    Ok(unsafe {
                        Self::StyleElement(
                            <StyleElement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "switch" => {
                    Ok(unsafe {
                        Self::Switch(
                            <Switch<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "text" => {
                    Ok(unsafe {
                        Self::Text(
                            <Text<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Comment(x) => ::type_sitter::Node::raw(x),
                Self::Directive(x) => ::type_sitter::Node::raw(x),
                Self::DirectiveEnd(x) => ::type_sitter::Node::raw(x),
                Self::DirectiveStart(x) => ::type_sitter::Node::raw(x),
                Self::Element(x) => ::type_sitter::Node::raw(x),
                Self::Entity(x) => ::type_sitter::Node::raw(x),
                Self::ErroneousEndTag(x) => ::type_sitter::Node::raw(x),
                Self::Keyword(x) => ::type_sitter::Node::raw(x),
                Self::Parameter(x) => ::type_sitter::Node::raw(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw(x),
                Self::ScriptElement(x) => ::type_sitter::Node::raw(x),
                Self::StyleElement(x) => ::type_sitter::Node::raw(x),
                Self::Switch(x) => ::type_sitter::Node::raw(x),
                Self::Text(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Comment(x) => ::type_sitter::Node::raw_mut(x),
                Self::Directive(x) => ::type_sitter::Node::raw_mut(x),
                Self::DirectiveEnd(x) => ::type_sitter::Node::raw_mut(x),
                Self::DirectiveStart(x) => ::type_sitter::Node::raw_mut(x),
                Self::Element(x) => ::type_sitter::Node::raw_mut(x),
                Self::Entity(x) => ::type_sitter::Node::raw_mut(x),
                Self::ErroneousEndTag(x) => ::type_sitter::Node::raw_mut(x),
                Self::Keyword(x) => ::type_sitter::Node::raw_mut(x),
                Self::Parameter(x) => ::type_sitter::Node::raw_mut(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw_mut(x),
                Self::ScriptElement(x) => ::type_sitter::Node::raw_mut(x),
                Self::StyleElement(x) => ::type_sitter::Node::raw_mut(x),
                Self::Switch(x) => ::type_sitter::Node::raw_mut(x),
                Self::Text(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Comment(x) => x.into_raw(),
                Self::Directive(x) => x.into_raw(),
                Self::DirectiveEnd(x) => x.into_raw(),
                Self::DirectiveStart(x) => x.into_raw(),
                Self::Element(x) => x.into_raw(),
                Self::Entity(x) => x.into_raw(),
                Self::ErroneousEndTag(x) => x.into_raw(),
                Self::Keyword(x) => x.into_raw(),
                Self::Parameter(x) => x.into_raw(),
                Self::PhpStatement(x) => x.into_raw(),
                Self::ScriptElement(x) => x.into_raw(),
                Self::StyleElement(x) => x.into_raw(),
                Self::Switch(x) => x.into_raw(),
                Self::Text(x) => x.into_raw(),
            }
        }
    }
    /**One of `{comment | conditional | conditional_keyword | directive | directive_end | directive_start | doctype | element | entity | envoy | erroneous_end_tag | fragment | keyword | livewire | loop | once | parameter | php_statement | script_element | section | stack | style_element | switch | text | verbatim}`:
- [`Comment`]
- [`Conditional`]
- [`ConditionalKeyword`]
- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Doctype`]
- [`Element`]
- [`Entity`]
- [`Envoy`]
- [`ErroneousEndTag`]
- [`Fragment`]
- [`Keyword`]
- [`Livewire`]
- [`Loop`]
- [`Once`]
- [`Parameter`]
- [`PhpStatement`]
- [`ScriptElement`]
- [`Section`]
- [`Stack`]
- [`StyleElement`]
- [`Switch`]
- [`Text`]
- [`Verbatim`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Anon40248131041664470318391226482553460865<'tree> {
        Comment(Comment<'tree>),
        Conditional(Conditional<'tree>),
        ConditionalKeyword(ConditionalKeyword<'tree>),
        Directive(Directive<'tree>),
        DirectiveEnd(DirectiveEnd<'tree>),
        DirectiveStart(DirectiveStart<'tree>),
        Doctype(Doctype<'tree>),
        Element(Element<'tree>),
        Entity(Entity<'tree>),
        Envoy(Envoy<'tree>),
        ErroneousEndTag(ErroneousEndTag<'tree>),
        Fragment(Fragment<'tree>),
        Keyword(Keyword<'tree>),
        Livewire(Livewire<'tree>),
        Loop(Loop<'tree>),
        Once(Once<'tree>),
        Parameter(Parameter<'tree>),
        PhpStatement(PhpStatement<'tree>),
        ScriptElement(ScriptElement<'tree>),
        Section(Section<'tree>),
        Stack(Stack<'tree>),
        StyleElement(StyleElement<'tree>),
        Switch(Switch<'tree>),
        Text(Text<'tree>),
        Verbatim(Verbatim<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Anon40248131041664470318391226482553460865<'tree> {
        ///Returns the node if it is of type `comment` ([`Comment`]), otherwise returns `None`
        #[inline]
        pub fn as_comment(self) -> ::std::option::Option<Comment<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Comment(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `conditional` ([`Conditional`]), otherwise returns `None`
        #[inline]
        pub fn as_conditional(self) -> ::std::option::Option<Conditional<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Conditional(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `conditional_keyword` ([`ConditionalKeyword`]), otherwise returns `None`
        #[inline]
        pub fn as_conditional_keyword(
            self,
        ) -> ::std::option::Option<ConditionalKeyword<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ConditionalKeyword(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `directive` ([`Directive`]), otherwise returns `None`
        #[inline]
        pub fn as_directive(self) -> ::std::option::Option<Directive<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Directive(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `directive_end` ([`DirectiveEnd`]), otherwise returns `None`
        #[inline]
        pub fn as_directive_end(self) -> ::std::option::Option<DirectiveEnd<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DirectiveEnd(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `directive_start` ([`DirectiveStart`]), otherwise returns `None`
        #[inline]
        pub fn as_directive_start(self) -> ::std::option::Option<DirectiveStart<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DirectiveStart(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `doctype` ([`Doctype`]), otherwise returns `None`
        #[inline]
        pub fn as_doctype(self) -> ::std::option::Option<Doctype<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Doctype(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `element` ([`Element`]), otherwise returns `None`
        #[inline]
        pub fn as_element(self) -> ::std::option::Option<Element<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Element(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `entity` ([`Entity`]), otherwise returns `None`
        #[inline]
        pub fn as_entity(self) -> ::std::option::Option<Entity<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Entity(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `envoy` ([`Envoy`]), otherwise returns `None`
        #[inline]
        pub fn as_envoy(self) -> ::std::option::Option<Envoy<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Envoy(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `erroneous_end_tag` ([`ErroneousEndTag`]), otherwise returns `None`
        #[inline]
        pub fn as_erroneous_end_tag(
            self,
        ) -> ::std::option::Option<ErroneousEndTag<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ErroneousEndTag(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `fragment` ([`Fragment`]), otherwise returns `None`
        #[inline]
        pub fn as_fragment(self) -> ::std::option::Option<Fragment<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Fragment(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `keyword` ([`Keyword`]), otherwise returns `None`
        #[inline]
        pub fn as_keyword(self) -> ::std::option::Option<Keyword<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Keyword(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `livewire` ([`Livewire`]), otherwise returns `None`
        #[inline]
        pub fn as_livewire(self) -> ::std::option::Option<Livewire<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Livewire(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `loop` ([`Loop`]), otherwise returns `None`
        #[inline]
        pub fn as_loop(self) -> ::std::option::Option<Loop<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Loop(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `once` ([`Once`]), otherwise returns `None`
        #[inline]
        pub fn as_once(self) -> ::std::option::Option<Once<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Once(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `parameter` ([`Parameter`]), otherwise returns `None`
        #[inline]
        pub fn as_parameter(self) -> ::std::option::Option<Parameter<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Parameter(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `php_statement` ([`PhpStatement`]), otherwise returns `None`
        #[inline]
        pub fn as_php_statement(self) -> ::std::option::Option<PhpStatement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::PhpStatement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `script_element` ([`ScriptElement`]), otherwise returns `None`
        #[inline]
        pub fn as_script_element(self) -> ::std::option::Option<ScriptElement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ScriptElement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `section` ([`Section`]), otherwise returns `None`
        #[inline]
        pub fn as_section(self) -> ::std::option::Option<Section<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Section(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `stack` ([`Stack`]), otherwise returns `None`
        #[inline]
        pub fn as_stack(self) -> ::std::option::Option<Stack<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Stack(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `style_element` ([`StyleElement`]), otherwise returns `None`
        #[inline]
        pub fn as_style_element(self) -> ::std::option::Option<StyleElement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::StyleElement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `switch` ([`Switch`]), otherwise returns `None`
        #[inline]
        pub fn as_switch(self) -> ::std::option::Option<Switch<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Switch(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `text` ([`Text`]), otherwise returns `None`
        #[inline]
        pub fn as_text(self) -> ::std::option::Option<Text<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Text(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `verbatim` ([`Verbatim`]), otherwise returns `None`
        #[inline]
        pub fn as_verbatim(self) -> ::std::option::Option<Verbatim<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Verbatim(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
    for Anon40248131041664470318391226482553460865<'tree> {
        type WithLifetime<'a> = Anon40248131041664470318391226482553460865<'a>;
        const KIND: &'static str = "{comment | conditional | conditional_keyword | directive | directive_end | directive_start | doctype | element | entity | envoy | erroneous_end_tag | fragment | keyword | livewire | loop | once | parameter | php_statement | script_element | section | stack | style_element | switch | text | verbatim}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "comment" => {
                    Ok(unsafe {
                        Self::Comment(
                            <Comment<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "conditional" => {
                    Ok(unsafe {
                        Self::Conditional(
                            <Conditional<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "conditional_keyword" => {
                    Ok(unsafe {
                        Self::ConditionalKeyword(
                            <ConditionalKeyword<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "directive" => {
                    Ok(unsafe {
                        Self::Directive(
                            <Directive<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "directive_end" => {
                    Ok(unsafe {
                        Self::DirectiveEnd(
                            <DirectiveEnd<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "directive_start" => {
                    Ok(unsafe {
                        Self::DirectiveStart(
                            <DirectiveStart<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "doctype" => {
                    Ok(unsafe {
                        Self::Doctype(
                            <Doctype<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "element" => {
                    Ok(unsafe {
                        Self::Element(
                            <Element<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "entity" => {
                    Ok(unsafe {
                        Self::Entity(
                            <Entity<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "envoy" => {
                    Ok(unsafe {
                        Self::Envoy(
                            <Envoy<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "erroneous_end_tag" => {
                    Ok(unsafe {
                        Self::ErroneousEndTag(
                            <ErroneousEndTag<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "fragment" => {
                    Ok(unsafe {
                        Self::Fragment(
                            <Fragment<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "keyword" => {
                    Ok(unsafe {
                        Self::Keyword(
                            <Keyword<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "livewire" => {
                    Ok(unsafe {
                        Self::Livewire(
                            <Livewire<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "loop" => {
                    Ok(unsafe {
                        Self::Loop(
                            <Loop<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "once" => {
                    Ok(unsafe {
                        Self::Once(
                            <Once<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "parameter" => {
                    Ok(unsafe {
                        Self::Parameter(
                            <Parameter<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "php_statement" => {
                    Ok(unsafe {
                        Self::PhpStatement(
                            <PhpStatement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "script_element" => {
                    Ok(unsafe {
                        Self::ScriptElement(
                            <ScriptElement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "section" => {
                    Ok(unsafe {
                        Self::Section(
                            <Section<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "stack" => {
                    Ok(unsafe {
                        Self::Stack(
                            <Stack<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "style_element" => {
                    Ok(unsafe {
                        Self::StyleElement(
                            <StyleElement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "switch" => {
                    Ok(unsafe {
                        Self::Switch(
                            <Switch<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "text" => {
                    Ok(unsafe {
                        Self::Text(
                            <Text<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "verbatim" => {
                    Ok(unsafe {
                        Self::Verbatim(
                            <Verbatim<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Comment(x) => ::type_sitter::Node::raw(x),
                Self::Conditional(x) => ::type_sitter::Node::raw(x),
                Self::ConditionalKeyword(x) => ::type_sitter::Node::raw(x),
                Self::Directive(x) => ::type_sitter::Node::raw(x),
                Self::DirectiveEnd(x) => ::type_sitter::Node::raw(x),
                Self::DirectiveStart(x) => ::type_sitter::Node::raw(x),
                Self::Doctype(x) => ::type_sitter::Node::raw(x),
                Self::Element(x) => ::type_sitter::Node::raw(x),
                Self::Entity(x) => ::type_sitter::Node::raw(x),
                Self::Envoy(x) => ::type_sitter::Node::raw(x),
                Self::ErroneousEndTag(x) => ::type_sitter::Node::raw(x),
                Self::Fragment(x) => ::type_sitter::Node::raw(x),
                Self::Keyword(x) => ::type_sitter::Node::raw(x),
                Self::Livewire(x) => ::type_sitter::Node::raw(x),
                Self::Loop(x) => ::type_sitter::Node::raw(x),
                Self::Once(x) => ::type_sitter::Node::raw(x),
                Self::Parameter(x) => ::type_sitter::Node::raw(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw(x),
                Self::ScriptElement(x) => ::type_sitter::Node::raw(x),
                Self::Section(x) => ::type_sitter::Node::raw(x),
                Self::Stack(x) => ::type_sitter::Node::raw(x),
                Self::StyleElement(x) => ::type_sitter::Node::raw(x),
                Self::Switch(x) => ::type_sitter::Node::raw(x),
                Self::Text(x) => ::type_sitter::Node::raw(x),
                Self::Verbatim(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Comment(x) => ::type_sitter::Node::raw_mut(x),
                Self::Conditional(x) => ::type_sitter::Node::raw_mut(x),
                Self::ConditionalKeyword(x) => ::type_sitter::Node::raw_mut(x),
                Self::Directive(x) => ::type_sitter::Node::raw_mut(x),
                Self::DirectiveEnd(x) => ::type_sitter::Node::raw_mut(x),
                Self::DirectiveStart(x) => ::type_sitter::Node::raw_mut(x),
                Self::Doctype(x) => ::type_sitter::Node::raw_mut(x),
                Self::Element(x) => ::type_sitter::Node::raw_mut(x),
                Self::Entity(x) => ::type_sitter::Node::raw_mut(x),
                Self::Envoy(x) => ::type_sitter::Node::raw_mut(x),
                Self::ErroneousEndTag(x) => ::type_sitter::Node::raw_mut(x),
                Self::Fragment(x) => ::type_sitter::Node::raw_mut(x),
                Self::Keyword(x) => ::type_sitter::Node::raw_mut(x),
                Self::Livewire(x) => ::type_sitter::Node::raw_mut(x),
                Self::Loop(x) => ::type_sitter::Node::raw_mut(x),
                Self::Once(x) => ::type_sitter::Node::raw_mut(x),
                Self::Parameter(x) => ::type_sitter::Node::raw_mut(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw_mut(x),
                Self::ScriptElement(x) => ::type_sitter::Node::raw_mut(x),
                Self::Section(x) => ::type_sitter::Node::raw_mut(x),
                Self::Stack(x) => ::type_sitter::Node::raw_mut(x),
                Self::StyleElement(x) => ::type_sitter::Node::raw_mut(x),
                Self::Switch(x) => ::type_sitter::Node::raw_mut(x),
                Self::Text(x) => ::type_sitter::Node::raw_mut(x),
                Self::Verbatim(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Comment(x) => x.into_raw(),
                Self::Conditional(x) => x.into_raw(),
                Self::ConditionalKeyword(x) => x.into_raw(),
                Self::Directive(x) => x.into_raw(),
                Self::DirectiveEnd(x) => x.into_raw(),
                Self::DirectiveStart(x) => x.into_raw(),
                Self::Doctype(x) => x.into_raw(),
                Self::Element(x) => x.into_raw(),
                Self::Entity(x) => x.into_raw(),
                Self::Envoy(x) => x.into_raw(),
                Self::ErroneousEndTag(x) => x.into_raw(),
                Self::Fragment(x) => x.into_raw(),
                Self::Keyword(x) => x.into_raw(),
                Self::Livewire(x) => x.into_raw(),
                Self::Loop(x) => x.into_raw(),
                Self::Once(x) => x.into_raw(),
                Self::Parameter(x) => x.into_raw(),
                Self::PhpStatement(x) => x.into_raw(),
                Self::ScriptElement(x) => x.into_raw(),
                Self::Section(x) => x.into_raw(),
                Self::Stack(x) => x.into_raw(),
                Self::StyleElement(x) => x.into_raw(),
                Self::Switch(x) => x.into_raw(),
                Self::Text(x) => x.into_raw(),
                Self::Verbatim(x) => x.into_raw(),
            }
        }
    }
    /**One of `{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | keyword | loop | parameter | php_statement | script_element | style_element | switch | text}`:
- [`Comment`]
- [`Conditional`]
- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Element`]
- [`Entity`]
- [`ErroneousEndTag`]
- [`Keyword`]
- [`Loop`]
- [`Parameter`]
- [`PhpStatement`]
- [`ScriptElement`]
- [`StyleElement`]
- [`Switch`]
- [`Text`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Anon7748293708392891243390944054945158570<'tree> {
        Comment(Comment<'tree>),
        Conditional(Conditional<'tree>),
        Directive(Directive<'tree>),
        DirectiveEnd(DirectiveEnd<'tree>),
        DirectiveStart(DirectiveStart<'tree>),
        Element(Element<'tree>),
        Entity(Entity<'tree>),
        ErroneousEndTag(ErroneousEndTag<'tree>),
        Keyword(Keyword<'tree>),
        Loop(Loop<'tree>),
        Parameter(Parameter<'tree>),
        PhpStatement(PhpStatement<'tree>),
        ScriptElement(ScriptElement<'tree>),
        StyleElement(StyleElement<'tree>),
        Switch(Switch<'tree>),
        Text(Text<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Anon7748293708392891243390944054945158570<'tree> {
        ///Returns the node if it is of type `comment` ([`Comment`]), otherwise returns `None`
        #[inline]
        pub fn as_comment(self) -> ::std::option::Option<Comment<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Comment(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `conditional` ([`Conditional`]), otherwise returns `None`
        #[inline]
        pub fn as_conditional(self) -> ::std::option::Option<Conditional<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Conditional(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `directive` ([`Directive`]), otherwise returns `None`
        #[inline]
        pub fn as_directive(self) -> ::std::option::Option<Directive<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Directive(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `directive_end` ([`DirectiveEnd`]), otherwise returns `None`
        #[inline]
        pub fn as_directive_end(self) -> ::std::option::Option<DirectiveEnd<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DirectiveEnd(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `directive_start` ([`DirectiveStart`]), otherwise returns `None`
        #[inline]
        pub fn as_directive_start(self) -> ::std::option::Option<DirectiveStart<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DirectiveStart(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `element` ([`Element`]), otherwise returns `None`
        #[inline]
        pub fn as_element(self) -> ::std::option::Option<Element<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Element(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `entity` ([`Entity`]), otherwise returns `None`
        #[inline]
        pub fn as_entity(self) -> ::std::option::Option<Entity<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Entity(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `erroneous_end_tag` ([`ErroneousEndTag`]), otherwise returns `None`
        #[inline]
        pub fn as_erroneous_end_tag(
            self,
        ) -> ::std::option::Option<ErroneousEndTag<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ErroneousEndTag(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `keyword` ([`Keyword`]), otherwise returns `None`
        #[inline]
        pub fn as_keyword(self) -> ::std::option::Option<Keyword<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Keyword(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `loop` ([`Loop`]), otherwise returns `None`
        #[inline]
        pub fn as_loop(self) -> ::std::option::Option<Loop<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Loop(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `parameter` ([`Parameter`]), otherwise returns `None`
        #[inline]
        pub fn as_parameter(self) -> ::std::option::Option<Parameter<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Parameter(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `php_statement` ([`PhpStatement`]), otherwise returns `None`
        #[inline]
        pub fn as_php_statement(self) -> ::std::option::Option<PhpStatement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::PhpStatement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `script_element` ([`ScriptElement`]), otherwise returns `None`
        #[inline]
        pub fn as_script_element(self) -> ::std::option::Option<ScriptElement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ScriptElement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `style_element` ([`StyleElement`]), otherwise returns `None`
        #[inline]
        pub fn as_style_element(self) -> ::std::option::Option<StyleElement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::StyleElement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `switch` ([`Switch`]), otherwise returns `None`
        #[inline]
        pub fn as_switch(self) -> ::std::option::Option<Switch<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Switch(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `text` ([`Text`]), otherwise returns `None`
        #[inline]
        pub fn as_text(self) -> ::std::option::Option<Text<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Text(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
    for Anon7748293708392891243390944054945158570<'tree> {
        type WithLifetime<'a> = Anon7748293708392891243390944054945158570<'a>;
        const KIND: &'static str = "{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | keyword | loop | parameter | php_statement | script_element | style_element | switch | text}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "comment" => {
                    Ok(unsafe {
                        Self::Comment(
                            <Comment<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "conditional" => {
                    Ok(unsafe {
                        Self::Conditional(
                            <Conditional<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "directive" => {
                    Ok(unsafe {
                        Self::Directive(
                            <Directive<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "directive_end" => {
                    Ok(unsafe {
                        Self::DirectiveEnd(
                            <DirectiveEnd<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "directive_start" => {
                    Ok(unsafe {
                        Self::DirectiveStart(
                            <DirectiveStart<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "element" => {
                    Ok(unsafe {
                        Self::Element(
                            <Element<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "entity" => {
                    Ok(unsafe {
                        Self::Entity(
                            <Entity<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "erroneous_end_tag" => {
                    Ok(unsafe {
                        Self::ErroneousEndTag(
                            <ErroneousEndTag<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "keyword" => {
                    Ok(unsafe {
                        Self::Keyword(
                            <Keyword<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "loop" => {
                    Ok(unsafe {
                        Self::Loop(
                            <Loop<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "parameter" => {
                    Ok(unsafe {
                        Self::Parameter(
                            <Parameter<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "php_statement" => {
                    Ok(unsafe {
                        Self::PhpStatement(
                            <PhpStatement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "script_element" => {
                    Ok(unsafe {
                        Self::ScriptElement(
                            <ScriptElement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "style_element" => {
                    Ok(unsafe {
                        Self::StyleElement(
                            <StyleElement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "switch" => {
                    Ok(unsafe {
                        Self::Switch(
                            <Switch<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "text" => {
                    Ok(unsafe {
                        Self::Text(
                            <Text<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Comment(x) => ::type_sitter::Node::raw(x),
                Self::Conditional(x) => ::type_sitter::Node::raw(x),
                Self::Directive(x) => ::type_sitter::Node::raw(x),
                Self::DirectiveEnd(x) => ::type_sitter::Node::raw(x),
                Self::DirectiveStart(x) => ::type_sitter::Node::raw(x),
                Self::Element(x) => ::type_sitter::Node::raw(x),
                Self::Entity(x) => ::type_sitter::Node::raw(x),
                Self::ErroneousEndTag(x) => ::type_sitter::Node::raw(x),
                Self::Keyword(x) => ::type_sitter::Node::raw(x),
                Self::Loop(x) => ::type_sitter::Node::raw(x),
                Self::Parameter(x) => ::type_sitter::Node::raw(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw(x),
                Self::ScriptElement(x) => ::type_sitter::Node::raw(x),
                Self::StyleElement(x) => ::type_sitter::Node::raw(x),
                Self::Switch(x) => ::type_sitter::Node::raw(x),
                Self::Text(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Comment(x) => ::type_sitter::Node::raw_mut(x),
                Self::Conditional(x) => ::type_sitter::Node::raw_mut(x),
                Self::Directive(x) => ::type_sitter::Node::raw_mut(x),
                Self::DirectiveEnd(x) => ::type_sitter::Node::raw_mut(x),
                Self::DirectiveStart(x) => ::type_sitter::Node::raw_mut(x),
                Self::Element(x) => ::type_sitter::Node::raw_mut(x),
                Self::Entity(x) => ::type_sitter::Node::raw_mut(x),
                Self::ErroneousEndTag(x) => ::type_sitter::Node::raw_mut(x),
                Self::Keyword(x) => ::type_sitter::Node::raw_mut(x),
                Self::Loop(x) => ::type_sitter::Node::raw_mut(x),
                Self::Parameter(x) => ::type_sitter::Node::raw_mut(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw_mut(x),
                Self::ScriptElement(x) => ::type_sitter::Node::raw_mut(x),
                Self::StyleElement(x) => ::type_sitter::Node::raw_mut(x),
                Self::Switch(x) => ::type_sitter::Node::raw_mut(x),
                Self::Text(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Comment(x) => x.into_raw(),
                Self::Conditional(x) => x.into_raw(),
                Self::Directive(x) => x.into_raw(),
                Self::DirectiveEnd(x) => x.into_raw(),
                Self::DirectiveStart(x) => x.into_raw(),
                Self::Element(x) => x.into_raw(),
                Self::Entity(x) => x.into_raw(),
                Self::ErroneousEndTag(x) => x.into_raw(),
                Self::Keyword(x) => x.into_raw(),
                Self::Loop(x) => x.into_raw(),
                Self::Parameter(x) => x.into_raw(),
                Self::PhpStatement(x) => x.into_raw(),
                Self::ScriptElement(x) => x.into_raw(),
                Self::StyleElement(x) => x.into_raw(),
                Self::Switch(x) => x.into_raw(),
                Self::Text(x) => x.into_raw(),
            }
        }
    }
    /**One of `{comment | conditional | directive | doctype | element | end_tag | entity | envoy | erroneous_end_tag | fragment | keyword | livewire | loop | once | parameter | php_statement | script_element | section | self_closing_tag | stack | start_tag | style_element | switch | text | verbatim}`:
- [`Comment`]
- [`Conditional`]
- [`Directive`]
- [`Doctype`]
- [`Element`]
- [`EndTag`]
- [`Entity`]
- [`Envoy`]
- [`ErroneousEndTag`]
- [`Fragment`]
- [`Keyword`]
- [`Livewire`]
- [`Loop`]
- [`Once`]
- [`Parameter`]
- [`PhpStatement`]
- [`ScriptElement`]
- [`Section`]
- [`SelfClosingTag`]
- [`Stack`]
- [`StartTag`]
- [`StyleElement`]
- [`Switch`]
- [`Text`]
- [`Verbatim`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Anon87989396667188848098758835341468324800<'tree> {
        Comment(Comment<'tree>),
        Conditional(Conditional<'tree>),
        Directive(Directive<'tree>),
        Doctype(Doctype<'tree>),
        Element(Element<'tree>),
        EndTag(EndTag<'tree>),
        Entity(Entity<'tree>),
        Envoy(Envoy<'tree>),
        ErroneousEndTag(ErroneousEndTag<'tree>),
        Fragment(Fragment<'tree>),
        Keyword(Keyword<'tree>),
        Livewire(Livewire<'tree>),
        Loop(Loop<'tree>),
        Once(Once<'tree>),
        Parameter(Parameter<'tree>),
        PhpStatement(PhpStatement<'tree>),
        ScriptElement(ScriptElement<'tree>),
        Section(Section<'tree>),
        SelfClosingTag(SelfClosingTag<'tree>),
        Stack(Stack<'tree>),
        StartTag(StartTag<'tree>),
        StyleElement(StyleElement<'tree>),
        Switch(Switch<'tree>),
        Text(Text<'tree>),
        Verbatim(Verbatim<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Anon87989396667188848098758835341468324800<'tree> {
        ///Returns the node if it is of type `comment` ([`Comment`]), otherwise returns `None`
        #[inline]
        pub fn as_comment(self) -> ::std::option::Option<Comment<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Comment(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `conditional` ([`Conditional`]), otherwise returns `None`
        #[inline]
        pub fn as_conditional(self) -> ::std::option::Option<Conditional<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Conditional(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `directive` ([`Directive`]), otherwise returns `None`
        #[inline]
        pub fn as_directive(self) -> ::std::option::Option<Directive<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Directive(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `doctype` ([`Doctype`]), otherwise returns `None`
        #[inline]
        pub fn as_doctype(self) -> ::std::option::Option<Doctype<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Doctype(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `element` ([`Element`]), otherwise returns `None`
        #[inline]
        pub fn as_element(self) -> ::std::option::Option<Element<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Element(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `end_tag` ([`EndTag`]), otherwise returns `None`
        #[inline]
        pub fn as_end_tag(self) -> ::std::option::Option<EndTag<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::EndTag(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `entity` ([`Entity`]), otherwise returns `None`
        #[inline]
        pub fn as_entity(self) -> ::std::option::Option<Entity<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Entity(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `envoy` ([`Envoy`]), otherwise returns `None`
        #[inline]
        pub fn as_envoy(self) -> ::std::option::Option<Envoy<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Envoy(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `erroneous_end_tag` ([`ErroneousEndTag`]), otherwise returns `None`
        #[inline]
        pub fn as_erroneous_end_tag(
            self,
        ) -> ::std::option::Option<ErroneousEndTag<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ErroneousEndTag(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `fragment` ([`Fragment`]), otherwise returns `None`
        #[inline]
        pub fn as_fragment(self) -> ::std::option::Option<Fragment<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Fragment(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `keyword` ([`Keyword`]), otherwise returns `None`
        #[inline]
        pub fn as_keyword(self) -> ::std::option::Option<Keyword<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Keyword(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `livewire` ([`Livewire`]), otherwise returns `None`
        #[inline]
        pub fn as_livewire(self) -> ::std::option::Option<Livewire<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Livewire(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `loop` ([`Loop`]), otherwise returns `None`
        #[inline]
        pub fn as_loop(self) -> ::std::option::Option<Loop<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Loop(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `once` ([`Once`]), otherwise returns `None`
        #[inline]
        pub fn as_once(self) -> ::std::option::Option<Once<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Once(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `parameter` ([`Parameter`]), otherwise returns `None`
        #[inline]
        pub fn as_parameter(self) -> ::std::option::Option<Parameter<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Parameter(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `php_statement` ([`PhpStatement`]), otherwise returns `None`
        #[inline]
        pub fn as_php_statement(self) -> ::std::option::Option<PhpStatement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::PhpStatement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `script_element` ([`ScriptElement`]), otherwise returns `None`
        #[inline]
        pub fn as_script_element(self) -> ::std::option::Option<ScriptElement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ScriptElement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `section` ([`Section`]), otherwise returns `None`
        #[inline]
        pub fn as_section(self) -> ::std::option::Option<Section<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Section(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `self_closing_tag` ([`SelfClosingTag`]), otherwise returns `None`
        #[inline]
        pub fn as_self_closing_tag(
            self,
        ) -> ::std::option::Option<SelfClosingTag<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::SelfClosingTag(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `stack` ([`Stack`]), otherwise returns `None`
        #[inline]
        pub fn as_stack(self) -> ::std::option::Option<Stack<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Stack(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `start_tag` ([`StartTag`]), otherwise returns `None`
        #[inline]
        pub fn as_start_tag(self) -> ::std::option::Option<StartTag<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::StartTag(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `style_element` ([`StyleElement`]), otherwise returns `None`
        #[inline]
        pub fn as_style_element(self) -> ::std::option::Option<StyleElement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::StyleElement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `switch` ([`Switch`]), otherwise returns `None`
        #[inline]
        pub fn as_switch(self) -> ::std::option::Option<Switch<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Switch(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `text` ([`Text`]), otherwise returns `None`
        #[inline]
        pub fn as_text(self) -> ::std::option::Option<Text<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Text(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `verbatim` ([`Verbatim`]), otherwise returns `None`
        #[inline]
        pub fn as_verbatim(self) -> ::std::option::Option<Verbatim<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Verbatim(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
    for Anon87989396667188848098758835341468324800<'tree> {
        type WithLifetime<'a> = Anon87989396667188848098758835341468324800<'a>;
        const KIND: &'static str = "{comment | conditional | directive | doctype | element | end_tag | entity | envoy | erroneous_end_tag | fragment | keyword | livewire | loop | once | parameter | php_statement | script_element | section | self_closing_tag | stack | start_tag | style_element | switch | text | verbatim}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "comment" => {
                    Ok(unsafe {
                        Self::Comment(
                            <Comment<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "conditional" => {
                    Ok(unsafe {
                        Self::Conditional(
                            <Conditional<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "directive" => {
                    Ok(unsafe {
                        Self::Directive(
                            <Directive<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "doctype" => {
                    Ok(unsafe {
                        Self::Doctype(
                            <Doctype<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "element" => {
                    Ok(unsafe {
                        Self::Element(
                            <Element<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "end_tag" => {
                    Ok(unsafe {
                        Self::EndTag(
                            <EndTag<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "entity" => {
                    Ok(unsafe {
                        Self::Entity(
                            <Entity<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "envoy" => {
                    Ok(unsafe {
                        Self::Envoy(
                            <Envoy<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "erroneous_end_tag" => {
                    Ok(unsafe {
                        Self::ErroneousEndTag(
                            <ErroneousEndTag<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "fragment" => {
                    Ok(unsafe {
                        Self::Fragment(
                            <Fragment<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "keyword" => {
                    Ok(unsafe {
                        Self::Keyword(
                            <Keyword<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "livewire" => {
                    Ok(unsafe {
                        Self::Livewire(
                            <Livewire<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "loop" => {
                    Ok(unsafe {
                        Self::Loop(
                            <Loop<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "once" => {
                    Ok(unsafe {
                        Self::Once(
                            <Once<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "parameter" => {
                    Ok(unsafe {
                        Self::Parameter(
                            <Parameter<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "php_statement" => {
                    Ok(unsafe {
                        Self::PhpStatement(
                            <PhpStatement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "script_element" => {
                    Ok(unsafe {
                        Self::ScriptElement(
                            <ScriptElement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "section" => {
                    Ok(unsafe {
                        Self::Section(
                            <Section<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "self_closing_tag" => {
                    Ok(unsafe {
                        Self::SelfClosingTag(
                            <SelfClosingTag<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "stack" => {
                    Ok(unsafe {
                        Self::Stack(
                            <Stack<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "start_tag" => {
                    Ok(unsafe {
                        Self::StartTag(
                            <StartTag<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "style_element" => {
                    Ok(unsafe {
                        Self::StyleElement(
                            <StyleElement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "switch" => {
                    Ok(unsafe {
                        Self::Switch(
                            <Switch<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "text" => {
                    Ok(unsafe {
                        Self::Text(
                            <Text<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "verbatim" => {
                    Ok(unsafe {
                        Self::Verbatim(
                            <Verbatim<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Comment(x) => ::type_sitter::Node::raw(x),
                Self::Conditional(x) => ::type_sitter::Node::raw(x),
                Self::Directive(x) => ::type_sitter::Node::raw(x),
                Self::Doctype(x) => ::type_sitter::Node::raw(x),
                Self::Element(x) => ::type_sitter::Node::raw(x),
                Self::EndTag(x) => ::type_sitter::Node::raw(x),
                Self::Entity(x) => ::type_sitter::Node::raw(x),
                Self::Envoy(x) => ::type_sitter::Node::raw(x),
                Self::ErroneousEndTag(x) => ::type_sitter::Node::raw(x),
                Self::Fragment(x) => ::type_sitter::Node::raw(x),
                Self::Keyword(x) => ::type_sitter::Node::raw(x),
                Self::Livewire(x) => ::type_sitter::Node::raw(x),
                Self::Loop(x) => ::type_sitter::Node::raw(x),
                Self::Once(x) => ::type_sitter::Node::raw(x),
                Self::Parameter(x) => ::type_sitter::Node::raw(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw(x),
                Self::ScriptElement(x) => ::type_sitter::Node::raw(x),
                Self::Section(x) => ::type_sitter::Node::raw(x),
                Self::SelfClosingTag(x) => ::type_sitter::Node::raw(x),
                Self::Stack(x) => ::type_sitter::Node::raw(x),
                Self::StartTag(x) => ::type_sitter::Node::raw(x),
                Self::StyleElement(x) => ::type_sitter::Node::raw(x),
                Self::Switch(x) => ::type_sitter::Node::raw(x),
                Self::Text(x) => ::type_sitter::Node::raw(x),
                Self::Verbatim(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Comment(x) => ::type_sitter::Node::raw_mut(x),
                Self::Conditional(x) => ::type_sitter::Node::raw_mut(x),
                Self::Directive(x) => ::type_sitter::Node::raw_mut(x),
                Self::Doctype(x) => ::type_sitter::Node::raw_mut(x),
                Self::Element(x) => ::type_sitter::Node::raw_mut(x),
                Self::EndTag(x) => ::type_sitter::Node::raw_mut(x),
                Self::Entity(x) => ::type_sitter::Node::raw_mut(x),
                Self::Envoy(x) => ::type_sitter::Node::raw_mut(x),
                Self::ErroneousEndTag(x) => ::type_sitter::Node::raw_mut(x),
                Self::Fragment(x) => ::type_sitter::Node::raw_mut(x),
                Self::Keyword(x) => ::type_sitter::Node::raw_mut(x),
                Self::Livewire(x) => ::type_sitter::Node::raw_mut(x),
                Self::Loop(x) => ::type_sitter::Node::raw_mut(x),
                Self::Once(x) => ::type_sitter::Node::raw_mut(x),
                Self::Parameter(x) => ::type_sitter::Node::raw_mut(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw_mut(x),
                Self::ScriptElement(x) => ::type_sitter::Node::raw_mut(x),
                Self::Section(x) => ::type_sitter::Node::raw_mut(x),
                Self::SelfClosingTag(x) => ::type_sitter::Node::raw_mut(x),
                Self::Stack(x) => ::type_sitter::Node::raw_mut(x),
                Self::StartTag(x) => ::type_sitter::Node::raw_mut(x),
                Self::StyleElement(x) => ::type_sitter::Node::raw_mut(x),
                Self::Switch(x) => ::type_sitter::Node::raw_mut(x),
                Self::Text(x) => ::type_sitter::Node::raw_mut(x),
                Self::Verbatim(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Comment(x) => x.into_raw(),
                Self::Conditional(x) => x.into_raw(),
                Self::Directive(x) => x.into_raw(),
                Self::Doctype(x) => x.into_raw(),
                Self::Element(x) => x.into_raw(),
                Self::EndTag(x) => x.into_raw(),
                Self::Entity(x) => x.into_raw(),
                Self::Envoy(x) => x.into_raw(),
                Self::ErroneousEndTag(x) => x.into_raw(),
                Self::Fragment(x) => x.into_raw(),
                Self::Keyword(x) => x.into_raw(),
                Self::Livewire(x) => x.into_raw(),
                Self::Loop(x) => x.into_raw(),
                Self::Once(x) => x.into_raw(),
                Self::Parameter(x) => x.into_raw(),
                Self::PhpStatement(x) => x.into_raw(),
                Self::ScriptElement(x) => x.into_raw(),
                Self::Section(x) => x.into_raw(),
                Self::SelfClosingTag(x) => x.into_raw(),
                Self::Stack(x) => x.into_raw(),
                Self::StartTag(x) => x.into_raw(),
                Self::StyleElement(x) => x.into_raw(),
                Self::Switch(x) => x.into_raw(),
                Self::Text(x) => x.into_raw(),
                Self::Verbatim(x) => x.into_raw(),
            }
        }
    }
    /**One of `{attribute_name | attribute_value | directive | parameter | php_statement | quoted_attribute_value}`:
- [`AttributeName`]
- [`AttributeValue`]
- [`Directive`]
- [`Parameter`]
- [`PhpStatement`]
- [`QuotedAttributeValue`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum AttributeName_AttributeValue_Directive_Parameter_PhpStatement_QuotedAttributeValue<
        'tree,
    > {
        AttributeName(AttributeName<'tree>),
        AttributeValue(AttributeValue<'tree>),
        Directive(Directive<'tree>),
        Parameter(Parameter<'tree>),
        PhpStatement(PhpStatement<'tree>),
        QuotedAttributeValue(QuotedAttributeValue<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<
        'tree,
    > AttributeName_AttributeValue_Directive_Parameter_PhpStatement_QuotedAttributeValue<
        'tree,
    > {
        ///Returns the node if it is of type `attribute_name` ([`AttributeName`]), otherwise returns `None`
        #[inline]
        pub fn as_attribute_name(self) -> ::std::option::Option<AttributeName<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::AttributeName(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `attribute_value` ([`AttributeValue`]), otherwise returns `None`
        #[inline]
        pub fn as_attribute_value(self) -> ::std::option::Option<AttributeValue<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::AttributeValue(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `directive` ([`Directive`]), otherwise returns `None`
        #[inline]
        pub fn as_directive(self) -> ::std::option::Option<Directive<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Directive(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `parameter` ([`Parameter`]), otherwise returns `None`
        #[inline]
        pub fn as_parameter(self) -> ::std::option::Option<Parameter<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Parameter(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `php_statement` ([`PhpStatement`]), otherwise returns `None`
        #[inline]
        pub fn as_php_statement(self) -> ::std::option::Option<PhpStatement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::PhpStatement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `quoted_attribute_value` ([`QuotedAttributeValue`]), otherwise returns `None`
        #[inline]
        pub fn as_quoted_attribute_value(
            self,
        ) -> ::std::option::Option<QuotedAttributeValue<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::QuotedAttributeValue(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
    for AttributeName_AttributeValue_Directive_Parameter_PhpStatement_QuotedAttributeValue<
        'tree,
    > {
        type WithLifetime<'a> = AttributeName_AttributeValue_Directive_Parameter_PhpStatement_QuotedAttributeValue<
            'a,
        >;
        const KIND: &'static str = "{attribute_name | attribute_value | directive | parameter | php_statement | quoted_attribute_value}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "attribute_name" => {
                    Ok(unsafe {
                        Self::AttributeName(
                            <AttributeName<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "attribute_value" => {
                    Ok(unsafe {
                        Self::AttributeValue(
                            <AttributeValue<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "directive" => {
                    Ok(unsafe {
                        Self::Directive(
                            <Directive<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "parameter" => {
                    Ok(unsafe {
                        Self::Parameter(
                            <Parameter<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "php_statement" => {
                    Ok(unsafe {
                        Self::PhpStatement(
                            <PhpStatement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "quoted_attribute_value" => {
                    Ok(unsafe {
                        Self::QuotedAttributeValue(
                            <QuotedAttributeValue<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::AttributeName(x) => ::type_sitter::Node::raw(x),
                Self::AttributeValue(x) => ::type_sitter::Node::raw(x),
                Self::Directive(x) => ::type_sitter::Node::raw(x),
                Self::Parameter(x) => ::type_sitter::Node::raw(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw(x),
                Self::QuotedAttributeValue(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::AttributeName(x) => ::type_sitter::Node::raw_mut(x),
                Self::AttributeValue(x) => ::type_sitter::Node::raw_mut(x),
                Self::Directive(x) => ::type_sitter::Node::raw_mut(x),
                Self::Parameter(x) => ::type_sitter::Node::raw_mut(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw_mut(x),
                Self::QuotedAttributeValue(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::AttributeName(x) => x.into_raw(),
                Self::AttributeValue(x) => x.into_raw(),
                Self::Directive(x) => x.into_raw(),
                Self::Parameter(x) => x.into_raw(),
                Self::PhpStatement(x) => x.into_raw(),
                Self::QuotedAttributeValue(x) => x.into_raw(),
            }
        }
    }
    /**One of `{attribute_value | comment | conditional | directive | parameter | php_statement}`:
- [`AttributeValue`]
- [`Comment`]
- [`Conditional`]
- [`Directive`]
- [`Parameter`]
- [`PhpStatement`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum AttributeValue_Comment_Conditional_Directive_Parameter_PhpStatement<'tree> {
        AttributeValue(AttributeValue<'tree>),
        Comment(Comment<'tree>),
        Conditional(Conditional<'tree>),
        Directive(Directive<'tree>),
        Parameter(Parameter<'tree>),
        PhpStatement(PhpStatement<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<
        'tree,
    > AttributeValue_Comment_Conditional_Directive_Parameter_PhpStatement<'tree> {
        ///Returns the node if it is of type `attribute_value` ([`AttributeValue`]), otherwise returns `None`
        #[inline]
        pub fn as_attribute_value(self) -> ::std::option::Option<AttributeValue<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::AttributeValue(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `comment` ([`Comment`]), otherwise returns `None`
        #[inline]
        pub fn as_comment(self) -> ::std::option::Option<Comment<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Comment(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `conditional` ([`Conditional`]), otherwise returns `None`
        #[inline]
        pub fn as_conditional(self) -> ::std::option::Option<Conditional<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Conditional(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `directive` ([`Directive`]), otherwise returns `None`
        #[inline]
        pub fn as_directive(self) -> ::std::option::Option<Directive<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Directive(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `parameter` ([`Parameter`]), otherwise returns `None`
        #[inline]
        pub fn as_parameter(self) -> ::std::option::Option<Parameter<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Parameter(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `php_statement` ([`PhpStatement`]), otherwise returns `None`
        #[inline]
        pub fn as_php_statement(self) -> ::std::option::Option<PhpStatement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::PhpStatement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
    for AttributeValue_Comment_Conditional_Directive_Parameter_PhpStatement<'tree> {
        type WithLifetime<'a> = AttributeValue_Comment_Conditional_Directive_Parameter_PhpStatement<
            'a,
        >;
        const KIND: &'static str = "{attribute_value | comment | conditional | directive | parameter | php_statement}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "attribute_value" => {
                    Ok(unsafe {
                        Self::AttributeValue(
                            <AttributeValue<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "comment" => {
                    Ok(unsafe {
                        Self::Comment(
                            <Comment<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "conditional" => {
                    Ok(unsafe {
                        Self::Conditional(
                            <Conditional<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "directive" => {
                    Ok(unsafe {
                        Self::Directive(
                            <Directive<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "parameter" => {
                    Ok(unsafe {
                        Self::Parameter(
                            <Parameter<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "php_statement" => {
                    Ok(unsafe {
                        Self::PhpStatement(
                            <PhpStatement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::AttributeValue(x) => ::type_sitter::Node::raw(x),
                Self::Comment(x) => ::type_sitter::Node::raw(x),
                Self::Conditional(x) => ::type_sitter::Node::raw(x),
                Self::Directive(x) => ::type_sitter::Node::raw(x),
                Self::Parameter(x) => ::type_sitter::Node::raw(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::AttributeValue(x) => ::type_sitter::Node::raw_mut(x),
                Self::Comment(x) => ::type_sitter::Node::raw_mut(x),
                Self::Conditional(x) => ::type_sitter::Node::raw_mut(x),
                Self::Directive(x) => ::type_sitter::Node::raw_mut(x),
                Self::Parameter(x) => ::type_sitter::Node::raw_mut(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::AttributeValue(x) => x.into_raw(),
                Self::Comment(x) => x.into_raw(),
                Self::Conditional(x) => x.into_raw(),
                Self::Directive(x) => x.into_raw(),
                Self::Parameter(x) => x.into_raw(),
                Self::PhpStatement(x) => x.into_raw(),
            }
        }
    }
    /**One of `{attribute | tag_name}`:
- [`Attribute`]
- [`TagName`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Attribute_TagName<'tree> {
        Attribute(Attribute<'tree>),
        TagName(TagName<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Attribute_TagName<'tree> {
        ///Returns the node if it is of type `attribute` ([`Attribute`]), otherwise returns `None`
        #[inline]
        pub fn as_attribute(self) -> ::std::option::Option<Attribute<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Attribute(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `tag_name` ([`TagName`]), otherwise returns `None`
        #[inline]
        pub fn as_tag_name(self) -> ::std::option::Option<TagName<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::TagName(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Attribute_TagName<'tree> {
        type WithLifetime<'a> = Attribute_TagName<'a>;
        const KIND: &'static str = "{attribute | tag_name}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "attribute" => {
                    Ok(unsafe {
                        Self::Attribute(
                            <Attribute<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "tag_name" => {
                    Ok(unsafe {
                        Self::TagName(
                            <TagName<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Attribute(x) => ::type_sitter::Node::raw(x),
                Self::TagName(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Attribute(x) => ::type_sitter::Node::raw_mut(x),
                Self::TagName(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Attribute(x) => x.into_raw(),
                Self::TagName(x) => x.into_raw(),
            }
        }
    }
    /**One of `{conditional_keyword | directive | directive_end | directive_start | parameter | php_only | text}`:
- [`ConditionalKeyword`]
- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Parameter`]
- [`PhpOnly`]
- [`Text`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ConditionalKeyword_Directive_DirectiveEnd_DirectiveStart_Parameter_PhpOnly_Text<
        'tree,
    > {
        ConditionalKeyword(ConditionalKeyword<'tree>),
        Directive(Directive<'tree>),
        DirectiveEnd(DirectiveEnd<'tree>),
        DirectiveStart(DirectiveStart<'tree>),
        Parameter(Parameter<'tree>),
        PhpOnly(PhpOnly<'tree>),
        Text(Text<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<
        'tree,
    > ConditionalKeyword_Directive_DirectiveEnd_DirectiveStart_Parameter_PhpOnly_Text<
        'tree,
    > {
        ///Returns the node if it is of type `conditional_keyword` ([`ConditionalKeyword`]), otherwise returns `None`
        #[inline]
        pub fn as_conditional_keyword(
            self,
        ) -> ::std::option::Option<ConditionalKeyword<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ConditionalKeyword(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `directive` ([`Directive`]), otherwise returns `None`
        #[inline]
        pub fn as_directive(self) -> ::std::option::Option<Directive<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Directive(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `directive_end` ([`DirectiveEnd`]), otherwise returns `None`
        #[inline]
        pub fn as_directive_end(self) -> ::std::option::Option<DirectiveEnd<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DirectiveEnd(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `directive_start` ([`DirectiveStart`]), otherwise returns `None`
        #[inline]
        pub fn as_directive_start(self) -> ::std::option::Option<DirectiveStart<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DirectiveStart(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `parameter` ([`Parameter`]), otherwise returns `None`
        #[inline]
        pub fn as_parameter(self) -> ::std::option::Option<Parameter<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Parameter(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `php_only` ([`PhpOnly`]), otherwise returns `None`
        #[inline]
        pub fn as_php_only(self) -> ::std::option::Option<PhpOnly<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::PhpOnly(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `text` ([`Text`]), otherwise returns `None`
        #[inline]
        pub fn as_text(self) -> ::std::option::Option<Text<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Text(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
    for ConditionalKeyword_Directive_DirectiveEnd_DirectiveStart_Parameter_PhpOnly_Text<
        'tree,
    > {
        type WithLifetime<'a> = ConditionalKeyword_Directive_DirectiveEnd_DirectiveStart_Parameter_PhpOnly_Text<
            'a,
        >;
        const KIND: &'static str = "{conditional_keyword | directive | directive_end | directive_start | parameter | php_only | text}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "conditional_keyword" => {
                    Ok(unsafe {
                        Self::ConditionalKeyword(
                            <ConditionalKeyword<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "directive" => {
                    Ok(unsafe {
                        Self::Directive(
                            <Directive<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "directive_end" => {
                    Ok(unsafe {
                        Self::DirectiveEnd(
                            <DirectiveEnd<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "directive_start" => {
                    Ok(unsafe {
                        Self::DirectiveStart(
                            <DirectiveStart<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "parameter" => {
                    Ok(unsafe {
                        Self::Parameter(
                            <Parameter<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "php_only" => {
                    Ok(unsafe {
                        Self::PhpOnly(
                            <PhpOnly<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "text" => {
                    Ok(unsafe {
                        Self::Text(
                            <Text<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::ConditionalKeyword(x) => ::type_sitter::Node::raw(x),
                Self::Directive(x) => ::type_sitter::Node::raw(x),
                Self::DirectiveEnd(x) => ::type_sitter::Node::raw(x),
                Self::DirectiveStart(x) => ::type_sitter::Node::raw(x),
                Self::Parameter(x) => ::type_sitter::Node::raw(x),
                Self::PhpOnly(x) => ::type_sitter::Node::raw(x),
                Self::Text(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::ConditionalKeyword(x) => ::type_sitter::Node::raw_mut(x),
                Self::Directive(x) => ::type_sitter::Node::raw_mut(x),
                Self::DirectiveEnd(x) => ::type_sitter::Node::raw_mut(x),
                Self::DirectiveStart(x) => ::type_sitter::Node::raw_mut(x),
                Self::Parameter(x) => ::type_sitter::Node::raw_mut(x),
                Self::PhpOnly(x) => ::type_sitter::Node::raw_mut(x),
                Self::Text(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::ConditionalKeyword(x) => x.into_raw(),
                Self::Directive(x) => x.into_raw(),
                Self::DirectiveEnd(x) => x.into_raw(),
                Self::DirectiveStart(x) => x.into_raw(),
                Self::Parameter(x) => x.into_raw(),
                Self::PhpOnly(x) => x.into_raw(),
                Self::Text(x) => x.into_raw(),
            }
        }
    }
    /**One of `{directive | directive_end | directive_start | parameter | php_end_tag | php_only | php_tag}`:
- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Parameter`]
- [`PhpEndTag`]
- [`PhpOnly`]
- [`PhpTag`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Directive_DirectiveEnd_DirectiveStart_Parameter_PhpEndTag_PhpOnly_PhpTag<
        'tree,
    > {
        Directive(Directive<'tree>),
        DirectiveEnd(DirectiveEnd<'tree>),
        DirectiveStart(DirectiveStart<'tree>),
        Parameter(Parameter<'tree>),
        PhpEndTag(PhpEndTag<'tree>),
        PhpOnly(PhpOnly<'tree>),
        PhpTag(PhpTag<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<
        'tree,
    > Directive_DirectiveEnd_DirectiveStart_Parameter_PhpEndTag_PhpOnly_PhpTag<'tree> {
        ///Returns the node if it is of type `directive` ([`Directive`]), otherwise returns `None`
        #[inline]
        pub fn as_directive(self) -> ::std::option::Option<Directive<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Directive(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `directive_end` ([`DirectiveEnd`]), otherwise returns `None`
        #[inline]
        pub fn as_directive_end(self) -> ::std::option::Option<DirectiveEnd<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DirectiveEnd(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `directive_start` ([`DirectiveStart`]), otherwise returns `None`
        #[inline]
        pub fn as_directive_start(self) -> ::std::option::Option<DirectiveStart<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DirectiveStart(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `parameter` ([`Parameter`]), otherwise returns `None`
        #[inline]
        pub fn as_parameter(self) -> ::std::option::Option<Parameter<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Parameter(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `php_end_tag` ([`PhpEndTag`]), otherwise returns `None`
        #[inline]
        pub fn as_php_end_tag(self) -> ::std::option::Option<PhpEndTag<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::PhpEndTag(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `php_only` ([`PhpOnly`]), otherwise returns `None`
        #[inline]
        pub fn as_php_only(self) -> ::std::option::Option<PhpOnly<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::PhpOnly(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `php_tag` ([`PhpTag`]), otherwise returns `None`
        #[inline]
        pub fn as_php_tag(self) -> ::std::option::Option<PhpTag<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::PhpTag(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
    for Directive_DirectiveEnd_DirectiveStart_Parameter_PhpEndTag_PhpOnly_PhpTag<'tree> {
        type WithLifetime<'a> = Directive_DirectiveEnd_DirectiveStart_Parameter_PhpEndTag_PhpOnly_PhpTag<
            'a,
        >;
        const KIND: &'static str = "{directive | directive_end | directive_start | parameter | php_end_tag | php_only | php_tag}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "directive" => {
                    Ok(unsafe {
                        Self::Directive(
                            <Directive<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "directive_end" => {
                    Ok(unsafe {
                        Self::DirectiveEnd(
                            <DirectiveEnd<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "directive_start" => {
                    Ok(unsafe {
                        Self::DirectiveStart(
                            <DirectiveStart<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "parameter" => {
                    Ok(unsafe {
                        Self::Parameter(
                            <Parameter<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "php_end_tag" => {
                    Ok(unsafe {
                        Self::PhpEndTag(
                            <PhpEndTag<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "php_only" => {
                    Ok(unsafe {
                        Self::PhpOnly(
                            <PhpOnly<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "php_tag" => {
                    Ok(unsafe {
                        Self::PhpTag(
                            <PhpTag<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Directive(x) => ::type_sitter::Node::raw(x),
                Self::DirectiveEnd(x) => ::type_sitter::Node::raw(x),
                Self::DirectiveStart(x) => ::type_sitter::Node::raw(x),
                Self::Parameter(x) => ::type_sitter::Node::raw(x),
                Self::PhpEndTag(x) => ::type_sitter::Node::raw(x),
                Self::PhpOnly(x) => ::type_sitter::Node::raw(x),
                Self::PhpTag(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Directive(x) => ::type_sitter::Node::raw_mut(x),
                Self::DirectiveEnd(x) => ::type_sitter::Node::raw_mut(x),
                Self::DirectiveStart(x) => ::type_sitter::Node::raw_mut(x),
                Self::Parameter(x) => ::type_sitter::Node::raw_mut(x),
                Self::PhpEndTag(x) => ::type_sitter::Node::raw_mut(x),
                Self::PhpOnly(x) => ::type_sitter::Node::raw_mut(x),
                Self::PhpTag(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Directive(x) => x.into_raw(),
                Self::DirectiveEnd(x) => x.into_raw(),
                Self::DirectiveStart(x) => x.into_raw(),
                Self::Parameter(x) => x.into_raw(),
                Self::PhpEndTag(x) => x.into_raw(),
                Self::PhpOnly(x) => x.into_raw(),
                Self::PhpTag(x) => x.into_raw(),
            }
        }
    }
    /**One of `{directive | parameter}`:
- [`Directive`]
- [`Parameter`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Directive_Parameter<'tree> {
        Directive(Directive<'tree>),
        Parameter(Parameter<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Directive_Parameter<'tree> {
        ///Returns the node if it is of type `directive` ([`Directive`]), otherwise returns `None`
        #[inline]
        pub fn as_directive(self) -> ::std::option::Option<Directive<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Directive(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `parameter` ([`Parameter`]), otherwise returns `None`
        #[inline]
        pub fn as_parameter(self) -> ::std::option::Option<Parameter<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Parameter(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Directive_Parameter<'tree> {
        type WithLifetime<'a> = Directive_Parameter<'a>;
        const KIND: &'static str = "{directive | parameter}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "directive" => {
                    Ok(unsafe {
                        Self::Directive(
                            <Directive<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "parameter" => {
                    Ok(unsafe {
                        Self::Parameter(
                            <Parameter<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Directive(x) => ::type_sitter::Node::raw(x),
                Self::Parameter(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Directive(x) => ::type_sitter::Node::raw_mut(x),
                Self::Parameter(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Directive(x) => x.into_raw(),
                Self::Parameter(x) => x.into_raw(),
            }
        }
    }
    /**One of `{end_tag | raw_text | start_tag}`:
- [`EndTag`]
- [`RawText`]
- [`StartTag`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum EndTag_RawText_StartTag<'tree> {
        EndTag(EndTag<'tree>),
        RawText(RawText<'tree>),
        StartTag(StartTag<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> EndTag_RawText_StartTag<'tree> {
        ///Returns the node if it is of type `end_tag` ([`EndTag`]), otherwise returns `None`
        #[inline]
        pub fn as_end_tag(self) -> ::std::option::Option<EndTag<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::EndTag(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `raw_text` ([`RawText`]), otherwise returns `None`
        #[inline]
        pub fn as_raw_text(self) -> ::std::option::Option<RawText<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::RawText(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `start_tag` ([`StartTag`]), otherwise returns `None`
        #[inline]
        pub fn as_start_tag(self) -> ::std::option::Option<StartTag<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::StartTag(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for EndTag_RawText_StartTag<'tree> {
        type WithLifetime<'a> = EndTag_RawText_StartTag<'a>;
        const KIND: &'static str = "{end_tag | raw_text | start_tag}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "end_tag" => {
                    Ok(unsafe {
                        Self::EndTag(
                            <EndTag<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "raw_text" => {
                    Ok(unsafe {
                        Self::RawText(
                            <RawText<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "start_tag" => {
                    Ok(unsafe {
                        Self::StartTag(
                            <StartTag<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }
        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::EndTag(x) => ::type_sitter::Node::raw(x),
                Self::RawText(x) => ::type_sitter::Node::raw(x),
                Self::StartTag(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::EndTag(x) => ::type_sitter::Node::raw_mut(x),
                Self::RawText(x) => ::type_sitter::Node::raw_mut(x),
                Self::StartTag(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::EndTag(x) => x.into_raw(),
                Self::RawText(x) => x.into_raw(),
                Self::StartTag(x) => x.into_raw(),
            }
        }
    }
}
