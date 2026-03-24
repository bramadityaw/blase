/**Typed node `anonymous_function`

This node has these fields:

- `attributes`: `attribute_list?` ([`AttributeList`])
- `body`: `{php_only | { | }}+` ([`PhpOnly`] | [`symbols::LBrace`] | [`symbols::RBrace`])
- `parameters`: `formal_parameters` ([`FormalParameters`])
- `reference_modifier`: `reference_modifier?` ([`ReferenceModifier`])
- `return_type`: `{bottom_type | type}?` ([`BottomType`] | [`Type`])
- `static_modifier`: `static_modifier?` ([`StaticModifier`])

And an optional additional named child of type `anonymous_function_use_clause?` ([`AnonymousFunctionUseClause`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct AnonymousFunction<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> AnonymousFunction<'tree> {
    /**Get the optional field `attributes`.

This child has type `attribute_list?` ([`AttributeList`])*/
    #[inline]
    pub fn attributes(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, AttributeList<'tree>>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("attributes")
            .map(<AttributeList<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
    /**Get the children of field `body`.

These children have type `{php_only | { | }}+`:

- [`PhpOnly`]
- [`symbols::LBrace`]
- [`symbols::RBrace`]
*/
    /**

This is guaranteed to return at least one child.*/
    #[inline]
    pub fn bodys<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<
            'tree,
            anon_unions::PhpOnly_LBrace_RBrace<'tree>,
        >,
    > + 'a {
        ::type_sitter::Node::raw(self)
            .children_by_field_name("body", &mut c.0)
            .map(
                <anon_unions::PhpOnly_LBrace_RBrace<
                    'tree,
                > as ::type_sitter::Node<'tree>>::try_from_raw,
            )
    }
    /**Get the field `parameters`.

This child has type `formal_parameters` ([`FormalParameters`])*/
    #[inline]
    pub fn parameters(
        &self,
    ) -> ::type_sitter::NodeResult<'tree, FormalParameters<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("parameters")
            .map(<FormalParameters<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the optional field `reference_modifier`.

This child has type `reference_modifier?` ([`ReferenceModifier`])*/
    #[inline]
    pub fn reference_modifier(
        &self,
    ) -> ::std::option::Option<
        ::type_sitter::NodeResult<'tree, ReferenceModifier<'tree>>,
    > {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("reference_modifier")
            .map(<ReferenceModifier<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
    /**Get the optional field `return_type`.

This child has type `{bottom_type | type}?`:

- [`BottomType`]
- [`Type`]
*/
    #[inline]
    pub fn return_type(
        &self,
    ) -> ::std::option::Option<
        ::type_sitter::NodeResult<'tree, anon_unions::BottomType_Type<'tree>>,
    > {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("return_type")
            .map(
                <anon_unions::BottomType_Type<
                    'tree,
                > as ::type_sitter::Node<'tree>>::try_from_raw,
            )
    }
    /**Get the optional field `static_modifier`.

This child has type `static_modifier?` ([`StaticModifier`])*/
    #[inline]
    pub fn static_modifier(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, StaticModifier<'tree>>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("static_modifier")
            .map(<StaticModifier<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
    /**Get the node's only non-field not-extra named child, if it has one.

This child has type `anonymous_function_use_clause?` ([`AnonymousFunctionUseClause`])*/
    #[inline]
    pub fn anonymous_function_use_clause(
        &self,
    ) -> ::std::option::Option<
        ::type_sitter::NodeResult<'tree, AnonymousFunctionUseClause<'tree>>,
    > {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .filter(|i| {
                ::type_sitter::Node::raw(self)
                    .field_name_for_named_child(*i as _)
                    .is_none()
            })
            .map(|i| ::type_sitter::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(
                <AnonymousFunctionUseClause<
                    'tree,
                > as ::type_sitter::Node<'tree>>::try_from_raw,
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for AnonymousFunction<'tree> {
    type WithLifetime<'a> = AnonymousFunction<'a>;
    const KIND: &'static str = "anonymous_function";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "anonymous_function" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "anonymous_function");
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
/**Typed node `anonymous_function_use_clause`

This node has named children of type `{by_ref | variable_name}+`:

- [`ByRef`]
- [`VariableName`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct AnonymousFunctionUseClause<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> AnonymousFunctionUseClause<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for AnonymousFunctionUseClause<'tree> {
    type Child = anon_unions::ByRef_VariableName<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for AnonymousFunctionUseClause<'tree> {
    type WithLifetime<'a> = AnonymousFunctionUseClause<'a>;
    const KIND: &'static str = "anonymous_function_use_clause";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "anonymous_function_use_clause" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "anonymous_function_use_clause");
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
/**Typed node `argument`

This node has these fields:

- `name`: `name?` ([`Name`])

And an additional named child of type `{expression | variadic_unpacking}`:

- [`Expression`]
- [`VariadicUnpacking`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Argument<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Argument<'tree> {
    /**Get the optional field `name`.

This child has type `name?` ([`Name`])*/
    #[inline]
    pub fn name(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, Name<'tree>>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("name")
            .map(<Name<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
    /**Get the node's only non-field not-extra named child.

This child has type `{expression | variadic_unpacking}`:

- [`Expression`]
- [`VariadicUnpacking`]
*/
    #[inline]
    pub fn other(
        &self,
    ) -> ::type_sitter::NodeResult<
        'tree,
        anon_unions::Expression_VariadicUnpacking<'tree>,
    > {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .filter(|i| {
                ::type_sitter::Node::raw(self)
                    .field_name_for_named_child(*i as _)
                    .is_none()
            })
            .map(|i| ::type_sitter::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(
                <anon_unions::Expression_VariadicUnpacking<
                    'tree,
                > as ::type_sitter::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Argument<'tree> {
    type WithLifetime<'a> = Argument<'a>;
    const KIND: &'static str = "argument";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "argument" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "argument");
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
/**Typed node `arguments`

This node has named children of type `argument*` ([`Argument`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Arguments<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Arguments<'tree> {
    /**Get the node's not-extra named children.

These children have type `argument*` ([`Argument`])*/
    #[inline]
    pub fn arguments<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<'tree, Argument<'tree>>,
    > + 'a {
        ::type_sitter::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<Argument<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for Arguments<'tree> {
    type Child = Argument<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Arguments<'tree> {
    type WithLifetime<'a> = Arguments<'a>;
    const KIND: &'static str = "arguments";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "arguments" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "arguments");
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
/**Typed node `array_creation_expression`

This node has named children of type `array_element_initializer*` ([`ArrayElementInitializer`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ArrayCreationExpression<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ArrayCreationExpression<'tree> {
    /**Get the node's not-extra named children.

These children have type `array_element_initializer*` ([`ArrayElementInitializer`])*/
    #[inline]
    pub fn array_element_initializers<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<'tree, ArrayElementInitializer<'tree>>,
    > + 'a {
        ::type_sitter::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(
                <ArrayElementInitializer<
                    'tree,
                > as ::type_sitter::Node<'tree>>::try_from_raw,
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for ArrayCreationExpression<'tree> {
    type Child = ArrayElementInitializer<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for ArrayCreationExpression<'tree> {
    type WithLifetime<'a> = ArrayCreationExpression<'a>;
    const KIND: &'static str = "array_creation_expression";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "array_creation_expression" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "array_creation_expression");
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
/**Typed node `array_element_initializer`

This node has a named child of type `{array_element_key_value_initializer | array_element_spreading_initializer | array_element_value_initializer}`:

- [`ArrayElementKeyValueInitializer`]
- [`ArrayElementSpreadingInitializer`]
- [`ArrayElementValueInitializer`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ArrayElementInitializer<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ArrayElementInitializer<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChild<'tree> for ArrayElementInitializer<'tree> {
    type Child = anon_unions::ArrayElementKeyValueInitializer_ArrayElementSpreadingInitializer_ArrayElementValueInitializer<
        'tree,
    >;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for ArrayElementInitializer<'tree> {
    type WithLifetime<'a> = ArrayElementInitializer<'a>;
    const KIND: &'static str = "array_element_initializer";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "array_element_initializer" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "array_element_initializer");
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
/**Typed node `array_element_key_value_initializer`

This node has these fields:

- `key`: `expression` ([`Expression`])
- `value`: `expression` ([`Expression`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ArrayElementKeyValueInitializer<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ArrayElementKeyValueInitializer<'tree> {
    /**Get the field `key`.

This child has type `expression` ([`Expression`])*/
    #[inline]
    pub fn key(&self) -> ::type_sitter::NodeResult<'tree, Expression<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("key")
            .map(<Expression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `value`.

This child has type `expression` ([`Expression`])*/
    #[inline]
    pub fn value(&self) -> ::type_sitter::NodeResult<'tree, Expression<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("value")
            .map(<Expression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for ArrayElementKeyValueInitializer<'tree> {
    type WithLifetime<'a> = ArrayElementKeyValueInitializer<'a>;
    const KIND: &'static str = "array_element_key_value_initializer";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "array_element_key_value_initializer" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "array_element_key_value_initializer");
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
/**Typed node `array_element_spreading_initializer`

This node has a named child of type `expression` ([`Expression`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ArrayElementSpreadingInitializer<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ArrayElementSpreadingInitializer<'tree> {
    /**Get the node's only not-extra named child.

This child has type `expression` ([`Expression`])*/
    #[inline]
    pub fn expression(&self) -> ::type_sitter::NodeResult<'tree, Expression<'tree>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .map(|i| ::type_sitter::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(<Expression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChild<'tree> for ArrayElementSpreadingInitializer<'tree> {
    type Child = Expression<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for ArrayElementSpreadingInitializer<'tree> {
    type WithLifetime<'a> = ArrayElementSpreadingInitializer<'a>;
    const KIND: &'static str = "array_element_spreading_initializer";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "array_element_spreading_initializer" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "array_element_spreading_initializer");
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
/**Typed node `array_element_value_initializer`

This node has a named child of type `expression` ([`Expression`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ArrayElementValueInitializer<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ArrayElementValueInitializer<'tree> {
    /**Get the node's only not-extra named child.

This child has type `expression` ([`Expression`])*/
    #[inline]
    pub fn expression(&self) -> ::type_sitter::NodeResult<'tree, Expression<'tree>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .map(|i| ::type_sitter::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(<Expression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChild<'tree> for ArrayElementValueInitializer<'tree> {
    type Child = Expression<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for ArrayElementValueInitializer<'tree> {
    type WithLifetime<'a> = ArrayElementValueInitializer<'a>;
    const KIND: &'static str = "array_element_value_initializer";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "array_element_value_initializer" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "array_element_value_initializer");
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
/**Typed node `arrow_function`

This node has these fields:

- `attributes`: `attribute_list?` ([`AttributeList`])
- `body`: `expression` ([`Expression`])
- `parameters`: `formal_parameters` ([`FormalParameters`])
- `reference_modifier`: `reference_modifier?` ([`ReferenceModifier`])
- `return_type`: `{bottom_type | type}?` ([`BottomType`] | [`Type`])
- `static_modifier`: `static_modifier?` ([`StaticModifier`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ArrowFunction<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ArrowFunction<'tree> {
    /**Get the optional field `attributes`.

This child has type `attribute_list?` ([`AttributeList`])*/
    #[inline]
    pub fn attributes(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, AttributeList<'tree>>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("attributes")
            .map(<AttributeList<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
    /**Get the field `body`.

This child has type `expression` ([`Expression`])*/
    #[inline]
    pub fn body(&self) -> ::type_sitter::NodeResult<'tree, Expression<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("body")
            .map(<Expression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `parameters`.

This child has type `formal_parameters` ([`FormalParameters`])*/
    #[inline]
    pub fn parameters(
        &self,
    ) -> ::type_sitter::NodeResult<'tree, FormalParameters<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("parameters")
            .map(<FormalParameters<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the optional field `reference_modifier`.

This child has type `reference_modifier?` ([`ReferenceModifier`])*/
    #[inline]
    pub fn reference_modifier(
        &self,
    ) -> ::std::option::Option<
        ::type_sitter::NodeResult<'tree, ReferenceModifier<'tree>>,
    > {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("reference_modifier")
            .map(<ReferenceModifier<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
    /**Get the optional field `return_type`.

This child has type `{bottom_type | type}?`:

- [`BottomType`]
- [`Type`]
*/
    #[inline]
    pub fn return_type(
        &self,
    ) -> ::std::option::Option<
        ::type_sitter::NodeResult<'tree, anon_unions::BottomType_Type<'tree>>,
    > {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("return_type")
            .map(
                <anon_unions::BottomType_Type<
                    'tree,
                > as ::type_sitter::Node<'tree>>::try_from_raw,
            )
    }
    /**Get the optional field `static_modifier`.

This child has type `static_modifier?` ([`StaticModifier`])*/
    #[inline]
    pub fn static_modifier(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, StaticModifier<'tree>>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("static_modifier")
            .map(<StaticModifier<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for ArrowFunction<'tree> {
    type WithLifetime<'a> = ArrowFunction<'a>;
    const KIND: &'static str = "arrow_function";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "arrow_function" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "arrow_function");
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
/**Typed node `assignment_expression`

This node has these fields:

- `left`: `{member_access_expression | subscript_expression | variable_name}` ([`MemberAccessExpression`] | [`SubscriptExpression`] | [`VariableName`])
- `right`: `expression` ([`Expression`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct AssignmentExpression<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> AssignmentExpression<'tree> {
    /**Get the field `left`.

This child has type `{member_access_expression | subscript_expression | variable_name}`:

- [`MemberAccessExpression`]
- [`SubscriptExpression`]
- [`VariableName`]
*/
    #[inline]
    pub fn left(
        &self,
    ) -> ::type_sitter::NodeResult<
        'tree,
        anon_unions::MemberAccessExpression_SubscriptExpression_VariableName<'tree>,
    > {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("left")
            .map(
                <anon_unions::MemberAccessExpression_SubscriptExpression_VariableName<
                    'tree,
                > as ::type_sitter::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `right`.

This child has type `expression` ([`Expression`])*/
    #[inline]
    pub fn right(&self) -> ::type_sitter::NodeResult<'tree, Expression<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("right")
            .map(<Expression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for AssignmentExpression<'tree> {
    type WithLifetime<'a> = AssignmentExpression<'a>;
    const KIND: &'static str = "assignment_expression";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "assignment_expression" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "assignment_expression");
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
/**Typed node `attribute`

This node has named children of type `{attribute_name | attribute_value | directive | expression | php_statement | quoted_attribute_value}+`:

- [`AttributeName`]
- [`AttributeValue`]
- [`Directive`]
- [`Expression`]
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
    type Child = anon_unions::AttributeName_AttributeValue_Directive_Expression_PhpStatement_QuotedAttributeValue<
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
/**Typed node `attribute_group`

This node has named children of type `attribute+` ([`Attribute`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct AttributeGroup<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> AttributeGroup<'tree> {
    /**Get the node's not-extra named children.

These children have type `attribute+` ([`Attribute`])*/
    /**

This is guaranteed to return at least one child.*/
    #[inline]
    pub fn attributes<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<'tree, Attribute<'tree>>,
    > + 'a {
        ::type_sitter::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<Attribute<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for AttributeGroup<'tree> {
    type Child = Attribute<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for AttributeGroup<'tree> {
    type WithLifetime<'a> = AttributeGroup<'a>;
    const KIND: &'static str = "attribute_group";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "attribute_group" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "attribute_group");
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
/**Typed node `attribute_list`

This node has named children of type `attribute_group+` ([`AttributeGroup`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct AttributeList<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> AttributeList<'tree> {
    /**Get the node's not-extra named children.

These children have type `attribute_group+` ([`AttributeGroup`])*/
    /**

This is guaranteed to return at least one child.*/
    #[inline]
    pub fn attribute_groups<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<'tree, AttributeGroup<'tree>>,
    > + 'a {
        ::type_sitter::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<AttributeGroup<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for AttributeList<'tree> {
    type Child = AttributeGroup<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for AttributeList<'tree> {
    type WithLifetime<'a> = AttributeList<'a>;
    const KIND: &'static str = "attribute_list";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "attribute_list" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "attribute_list");
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
/**Typed node `binary_expression`

This node has these fields:

- `left`: `{cast_expression | expression | primary_expression | unary_op_expression}` ([`CastExpression`] | [`Expression`] | [`PrimaryExpression`] | [`UnaryOpExpression`])
- `operator`: `{!= | !== | % | & | && | * | ** | + | - | . | / | < | << | <= | <=> | <> | == | === | > | >= | >> | ?? | ^ | and | instanceof | or | xor | | | |> | ||}` ([`symbols::NotEq`] | [`symbols::NotEqEq`] | [`symbols::Mod`] | [`symbols::And`] | [`symbols::AndAnd`] | [`symbols::Mul`] | [`symbols::MulMul`] | [`symbols::Add`] | [`symbols::Sub`] | [`symbols::Dot`] | [`symbols::Div`] | [`symbols::Lt`] | [`symbols::LtLt`] | [`symbols::LtEq`] | [`symbols::LtEqGt`] | [`symbols::LtGt`] | [`symbols::EqEq`] | [`symbols::EqEqEq`] | [`symbols::Gt`] | [`symbols::GtEq`] | [`symbols::GtGt`] | [`symbols::QuestionQuestion`] | [`symbols::BitXor`] | [`unnamed::And`] | [`unnamed::Instanceof`] | [`unnamed::Or`] | [`unnamed::Xor`] | [`symbols::Or`] | [`symbols::OrGt`] | [`symbols::OrOr`])
- `right`: `{dynamic_variable_name | expression | member_access_expression | name | nullsafe_member_access_expression | parenthesized_expression | qualified_name | scoped_property_access_expression | subscript_expression | variable_name}` ([`DynamicVariableName`] | [`Expression`] | [`MemberAccessExpression`] | [`Name`] | [`NullsafeMemberAccessExpression`] | [`ParenthesizedExpression`] | [`QualifiedName`] | [`ScopedPropertyAccessExpression`] | [`SubscriptExpression`] | [`VariableName`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct BinaryExpression<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> BinaryExpression<'tree> {
    /**Get the field `left`.

This child has type `{cast_expression | expression | primary_expression | unary_op_expression}`:

- [`CastExpression`]
- [`Expression`]
- [`PrimaryExpression`]
- [`UnaryOpExpression`]
*/
    #[inline]
    pub fn left(
        &self,
    ) -> ::type_sitter::NodeResult<
        'tree,
        anon_unions::CastExpression_Expression_PrimaryExpression_UnaryOpExpression<'tree>,
    > {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("left")
            .map(
                <anon_unions::CastExpression_Expression_PrimaryExpression_UnaryOpExpression<
                    'tree,
                > as ::type_sitter::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `operator`.

This child has type `{!= | !== | % | & | && | * | ** | + | - | . | / | < | << | <= | <=> | <> | == | === | > | >= | >> | ?? | ^ | and | instanceof | or | xor | | | |> | ||}`:

- [`symbols::NotEq`]
- [`symbols::NotEqEq`]
- [`symbols::Mod`]
- [`symbols::And`]
- [`symbols::AndAnd`]
- [`symbols::Mul`]
- [`symbols::MulMul`]
- [`symbols::Add`]
- [`symbols::Sub`]
- [`symbols::Dot`]
- [`symbols::Div`]
- [`symbols::Lt`]
- [`symbols::LtLt`]
- [`symbols::LtEq`]
- [`symbols::LtEqGt`]
- [`symbols::LtGt`]
- [`symbols::EqEq`]
- [`symbols::EqEqEq`]
- [`symbols::Gt`]
- [`symbols::GtEq`]
- [`symbols::GtGt`]
- [`symbols::QuestionQuestion`]
- [`symbols::BitXor`]
- [`unnamed::And`]
- [`unnamed::Instanceof`]
- [`unnamed::Or`]
- [`unnamed::Xor`]
- [`symbols::Or`]
- [`symbols::OrGt`]
- [`symbols::OrOr`]
*/
    #[inline]
    pub fn operator(
        &self,
    ) -> ::type_sitter::NodeResult<
        'tree,
        anon_unions::Anon156865598636788927821460262849502993268<'tree>,
    > {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("operator")
            .map(
                <anon_unions::Anon156865598636788927821460262849502993268<
                    'tree,
                > as ::type_sitter::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `right`.

This child has type `{dynamic_variable_name | expression | member_access_expression | name | nullsafe_member_access_expression | parenthesized_expression | qualified_name | scoped_property_access_expression | subscript_expression | variable_name}`:

- [`DynamicVariableName`]
- [`Expression`]
- [`MemberAccessExpression`]
- [`Name`]
- [`NullsafeMemberAccessExpression`]
- [`ParenthesizedExpression`]
- [`QualifiedName`]
- [`ScopedPropertyAccessExpression`]
- [`SubscriptExpression`]
- [`VariableName`]
*/
    #[inline]
    pub fn right(
        &self,
    ) -> ::type_sitter::NodeResult<
        'tree,
        anon_unions::Anon308160872237808256834920196883287176613<'tree>,
    > {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("right")
            .map(
                <anon_unions::Anon308160872237808256834920196883287176613<
                    'tree,
                > as ::type_sitter::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for BinaryExpression<'tree> {
    type WithLifetime<'a> = BinaryExpression<'a>;
    const KIND: &'static str = "binary_expression";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "binary_expression" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "binary_expression");
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
/**Typed node `boolean`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Boolean<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Boolean<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Boolean<'tree> {
    type WithLifetime<'a> = Boolean<'a>;
    const KIND: &'static str = "boolean";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "boolean" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "boolean");
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
/**Typed node `bottom_type`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct BottomType<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> BottomType<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for BottomType<'tree> {
    type WithLifetime<'a> = BottomType<'a>;
    const KIND: &'static str = "bottom_type";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "bottom_type" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "bottom_type");
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
/**Typed node `by_ref`

This node has a named child of type `{member_access_expression | subscript_expression | variable_name}`:

- [`MemberAccessExpression`]
- [`SubscriptExpression`]
- [`VariableName`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ByRef<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ByRef<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChild<'tree> for ByRef<'tree> {
    type Child = anon_unions::MemberAccessExpression_SubscriptExpression_VariableName<
        'tree,
    >;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for ByRef<'tree> {
    type WithLifetime<'a> = ByRef<'a>;
    const KIND: &'static str = "by_ref";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "by_ref" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "by_ref");
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
/**Typed node `cast_expression`

This node has these fields:

- `type`: `cast_type` ([`CastType`])
- `value`: `{cast_expression | primary_expression | unary_op_expression}` ([`CastExpression`] | [`PrimaryExpression`] | [`UnaryOpExpression`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct CastExpression<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> CastExpression<'tree> {
    /**Get the field `type`.

This child has type `cast_type` ([`CastType`])*/
    #[inline]
    pub fn r#type(&self) -> ::type_sitter::NodeResult<'tree, CastType<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("type")
            .map(<CastType<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `value`.

This child has type `{cast_expression | primary_expression | unary_op_expression}`:

- [`CastExpression`]
- [`PrimaryExpression`]
- [`UnaryOpExpression`]
*/
    #[inline]
    pub fn value(
        &self,
    ) -> ::type_sitter::NodeResult<
        'tree,
        anon_unions::CastExpression_PrimaryExpression_UnaryOpExpression<'tree>,
    > {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("value")
            .map(
                <anon_unions::CastExpression_PrimaryExpression_UnaryOpExpression<
                    'tree,
                > as ::type_sitter::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for CastExpression<'tree> {
    type WithLifetime<'a> = CastExpression<'a>;
    const KIND: &'static str = "cast_expression";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "cast_expression" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "cast_expression");
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
/**Typed node `cast_type`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct CastType<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> CastType<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for CastType<'tree> {
    type WithLifetime<'a> = CastType<'a>;
    const KIND: &'static str = "cast_type";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "cast_type" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "cast_type");
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
/**Typed node `class_constant_access_expression`

This node has these fields:

- `name`: `name` ([`Name`])
- `scope`: `{name | qualified_name | relative_scope}` ([`Name`] | [`QualifiedName`] | [`RelativeScope`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ClassConstantAccessExpression<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ClassConstantAccessExpression<'tree> {
    /**Get the field `name`.

This child has type `name` ([`Name`])*/
    #[inline]
    pub fn name(&self) -> ::type_sitter::NodeResult<'tree, Name<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("name")
            .map(<Name<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `scope`.

This child has type `{name | qualified_name | relative_scope}`:

- [`Name`]
- [`QualifiedName`]
- [`RelativeScope`]
*/
    #[inline]
    pub fn scope(
        &self,
    ) -> ::type_sitter::NodeResult<
        'tree,
        anon_unions::Name_QualifiedName_RelativeScope<'tree>,
    > {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("scope")
            .map(
                <anon_unions::Name_QualifiedName_RelativeScope<
                    'tree,
                > as ::type_sitter::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for ClassConstantAccessExpression<'tree> {
    type WithLifetime<'a> = ClassConstantAccessExpression<'a>;
    const KIND: &'static str = "class_constant_access_expression";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "class_constant_access_expression" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "class_constant_access_expression");
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

This node has named children of type `{comment | conditional | conditional_keyword | directive | directive_end | directive_start | doctype | element | entity | envoy | erroneous_end_tag | expression | fragment | keyword | livewire | loop | once | php_statement | props | script_element | section | stack | style_element | switch | text | verbatim}*`:

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
- [`Expression`]
- [`Fragment`]
- [`Keyword`]
- [`Livewire`]
- [`Loop`]
- [`Once`]
- [`PhpStatement`]
- [`Props`]
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
    type Child = anon_unions::Anon71931883397758457417774301653197094678<'tree>;
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
/**Typed node `conditional_expression`

This node has these fields:

- `alternative`: `expression` ([`Expression`])
- `body`: `expression?` ([`Expression`])
- `condition`: `expression` ([`Expression`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ConditionalExpression<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ConditionalExpression<'tree> {
    /**Get the field `alternative`.

This child has type `expression` ([`Expression`])*/
    #[inline]
    pub fn alternative(&self) -> ::type_sitter::NodeResult<'tree, Expression<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("alternative")
            .map(<Expression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the optional field `body`.

This child has type `expression?` ([`Expression`])*/
    #[inline]
    pub fn body(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, Expression<'tree>>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("body")
            .map(<Expression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
    /**Get the field `condition`.

This child has type `expression` ([`Expression`])*/
    #[inline]
    pub fn condition(&self) -> ::type_sitter::NodeResult<'tree, Expression<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("condition")
            .map(<Expression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for ConditionalExpression<'tree> {
    type WithLifetime<'a> = ConditionalExpression<'a>;
    const KIND: &'static str = "conditional_expression";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "conditional_expression" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "conditional_expression");
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

This node has named children of type `{directive | expression}+`:

- [`Directive`]
- [`Expression`]

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
    type Child = anon_unions::Directive_Expression<'tree>;
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
/**Typed node `disjunctive_normal_form_type`

This node has named children of type `{intersection_type | named_type | optional_type | primitive_type}+`:

- [`IntersectionType`]
- [`NamedType`]
- [`OptionalType`]
- [`PrimitiveType`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct DisjunctiveNormalFormType<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> DisjunctiveNormalFormType<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for DisjunctiveNormalFormType<'tree> {
    type Child = anon_unions::IntersectionType_NamedType_OptionalType_PrimitiveType<
        'tree,
    >;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for DisjunctiveNormalFormType<'tree> {
    type WithLifetime<'a> = DisjunctiveNormalFormType<'a>;
    const KIND: &'static str = "disjunctive_normal_form_type";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "disjunctive_normal_form_type" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "disjunctive_normal_form_type");
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

This node has named children of type `{comment | conditional | directive | doctype | element | entity | envoy | erroneous_end_tag | expression | fragment | keyword | livewire | loop | once | php_statement | props | script_element | section | stack | style_element | switch | text | verbatim}*`:

- [`Comment`]
- [`Conditional`]
- [`Directive`]
- [`Doctype`]
- [`Element`]
- [`Entity`]
- [`Envoy`]
- [`ErroneousEndTag`]
- [`Expression`]
- [`Fragment`]
- [`Keyword`]
- [`Livewire`]
- [`Loop`]
- [`Once`]
- [`PhpStatement`]
- [`Props`]
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
    type Child = anon_unions::Anon228412328637052745595247458993952546707<'tree>;
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
/**Typed node `dynamic_variable_name`

This node has a named child of type `{dynamic_variable_name | expression | variable_name}`:

- [`DynamicVariableName`]
- [`Expression`]
- [`VariableName`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct DynamicVariableName<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> DynamicVariableName<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChild<'tree> for DynamicVariableName<'tree> {
    type Child = anon_unions::DynamicVariableName_Expression_VariableName<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for DynamicVariableName<'tree> {
    type WithLifetime<'a> = DynamicVariableName<'a>;
    const KIND: &'static str = "dynamic_variable_name";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "dynamic_variable_name" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "dynamic_variable_name");
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

This node has named children of type `{comment | conditional | directive | doctype | element | end_tag | entity | envoy | erroneous_end_tag | expression | fragment | keyword | livewire | loop | once | php_statement | props | script_element | section | self_closing_tag | stack | start_tag | style_element | switch | text | verbatim}+`:

- [`Comment`]
- [`Conditional`]
- [`Directive`]
- [`Doctype`]
- [`Element`]
- [`EndTag`]
- [`Entity`]
- [`Envoy`]
- [`ErroneousEndTag`]
- [`Expression`]
- [`Fragment`]
- [`Keyword`]
- [`Livewire`]
- [`Loop`]
- [`Once`]
- [`PhpStatement`]
- [`Props`]
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
    type Child = anon_unions::Anon227303402907815333350325131840109493873<'tree>;
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
/**Typed node `encapsed_string`

This node has named children of type `{escape_sequence | variable_name}*`:

- [`EscapeSequence`]
- [`VariableName`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct EncapsedString<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> EncapsedString<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for EncapsedString<'tree> {
    type Child = anon_unions::EscapeSequence_VariableName<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for EncapsedString<'tree> {
    type WithLifetime<'a> = EncapsedString<'a>;
    const KIND: &'static str = "encapsed_string";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "encapsed_string" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "encapsed_string");
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

This node has named children of type `{conditional_keyword | directive | directive_end | directive_start | expression | php_only | text}*`:

- [`ConditionalKeyword`]
- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Expression`]
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
    type Child = anon_unions::ConditionalKeyword_Directive_DirectiveEnd_DirectiveStart_Expression_PhpOnly_Text<
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
/**Typed node `escape_sequence`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct EscapeSequence<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> EscapeSequence<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for EscapeSequence<'tree> {
    type WithLifetime<'a> = EscapeSequence<'a>;
    const KIND: &'static str = "escape_sequence";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "escape_sequence" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "escape_sequence");
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
/**Typed node `expression`

This node has a named child of type `{assignment_expression | binary_expression | cast_expression | conditional_expression | primary_expression | unary_op_expression}`:

- [`AssignmentExpression`]
- [`BinaryExpression`]
- [`CastExpression`]
- [`ConditionalExpression`]
- [`PrimaryExpression`]
- [`UnaryOpExpression`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Expression<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Expression<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChild<'tree> for Expression<'tree> {
    type Child = anon_unions::Anon249583382270925116125355034815407047852<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Expression<'tree> {
    type WithLifetime<'a> = Expression<'a>;
    const KIND: &'static str = "expression";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "expression" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "expression");
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
/**Typed node `float`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Float<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Float<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Float<'tree> {
    type WithLifetime<'a> = Float<'a>;
    const KIND: &'static str = "float";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "float" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "float");
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
/**Typed node `formal_parameters`

This node has named children of type `{property_promotion_parameter | simple_parameter | variadic_parameter}*`:

- [`PropertyPromotionParameter`]
- [`SimpleParameter`]
- [`VariadicParameter`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct FormalParameters<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> FormalParameters<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for FormalParameters<'tree> {
    type Child = anon_unions::PropertyPromotionParameter_SimpleParameter_VariadicParameter<
        'tree,
    >;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for FormalParameters<'tree> {
    type WithLifetime<'a> = FormalParameters<'a>;
    const KIND: &'static str = "formal_parameters";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "formal_parameters" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "formal_parameters");
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

This node has named children of type `{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | expression | keyword | loop | php_statement | props | script_element | style_element | switch | text | verbatim}+`:

- [`Comment`]
- [`Conditional`]
- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Element`]
- [`Entity`]
- [`ErroneousEndTag`]
- [`Expression`]
- [`Keyword`]
- [`Loop`]
- [`PhpStatement`]
- [`Props`]
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
    type Child = anon_unions::Anon94390688163068428215780624704411407183<'tree>;
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
/**Typed node `function_call_expression`

This node has these fields:

- `arguments`: `arguments` ([`Arguments`])
- `function`: `name` ([`Name`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct FunctionCallExpression<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> FunctionCallExpression<'tree> {
    /**Get the field `arguments`.

This child has type `arguments` ([`Arguments`])*/
    #[inline]
    pub fn arguments(&self) -> ::type_sitter::NodeResult<'tree, Arguments<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("arguments")
            .map(<Arguments<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `function`.

This child has type `name` ([`Name`])*/
    #[inline]
    pub fn function(&self) -> ::type_sitter::NodeResult<'tree, Name<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("function")
            .map(<Name<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for FunctionCallExpression<'tree> {
    type WithLifetime<'a> = FunctionCallExpression<'a>;
    const KIND: &'static str = "function_call_expression";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "function_call_expression" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "function_call_expression");
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
/**Typed node `integer`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Integer<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Integer<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Integer<'tree> {
    type WithLifetime<'a> = Integer<'a>;
    const KIND: &'static str = "integer";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "integer" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "integer");
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
/**Typed node `intersection_type`

This node has named children of type `{named_type | optional_type | primitive_type}+`:

- [`NamedType`]
- [`OptionalType`]
- [`PrimitiveType`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct IntersectionType<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> IntersectionType<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for IntersectionType<'tree> {
    type Child = anon_unions::NamedType_OptionalType_PrimitiveType<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for IntersectionType<'tree> {
    type WithLifetime<'a> = IntersectionType<'a>;
    const KIND: &'static str = "intersection_type";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "intersection_type" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "intersection_type");
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
/**Typed node `literal`

This node has a named child of type `{boolean | encapsed_string | float | integer | null | string}`:

- [`Boolean`]
- [`EncapsedString`]
- [`Float`]
- [`Integer`]
- [`Null`]
- [`String`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Literal<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Literal<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChild<'tree> for Literal<'tree> {
    type Child = anon_unions::Boolean_EncapsedString_Float_Integer_Null_String<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Literal<'tree> {
    type WithLifetime<'a> = Literal<'a>;
    const KIND: &'static str = "literal";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "literal" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "literal");
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

This node has named children of type `{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | expression | keyword | livewire | loop | php_statement | props | script_element | style_element | switch | text}+`:

- [`Comment`]
- [`Conditional`]
- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Element`]
- [`Entity`]
- [`ErroneousEndTag`]
- [`Expression`]
- [`Keyword`]
- [`Livewire`]
- [`Loop`]
- [`PhpStatement`]
- [`Props`]
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
    type Child = anon_unions::Anon145495983449542552864963724180875256821<'tree>;
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

This node has named children of type `{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | expression | keyword | loop | php_statement | props | script_element | style_element | switch | text}+`:

- [`Comment`]
- [`Conditional`]
- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Element`]
- [`Entity`]
- [`ErroneousEndTag`]
- [`Expression`]
- [`Keyword`]
- [`Loop`]
- [`PhpStatement`]
- [`Props`]
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
    type Child = anon_unions::Anon310217771700970732338886093335230545339<'tree>;
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
/**Typed node `member_access_expression`

This node has these fields:

- `name`: `{dynamic_variable_name | expression | name | variable_name}` ([`DynamicVariableName`] | [`Expression`] | [`Name`] | [`VariableName`])
- `object`: `{array_creation_expression | dynamic_variable_name | member_access_expression | nullsafe_member_access_expression | parenthesized_expression | scoped_property_access_expression | subscript_expression | variable_name}` ([`ArrayCreationExpression`] | [`DynamicVariableName`] | [`MemberAccessExpression`] | [`NullsafeMemberAccessExpression`] | [`ParenthesizedExpression`] | [`ScopedPropertyAccessExpression`] | [`SubscriptExpression`] | [`VariableName`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct MemberAccessExpression<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> MemberAccessExpression<'tree> {
    /**Get the field `name`.

This child has type `{dynamic_variable_name | expression | name | variable_name}`:

- [`DynamicVariableName`]
- [`Expression`]
- [`Name`]
- [`VariableName`]
*/
    #[inline]
    pub fn name(
        &self,
    ) -> ::type_sitter::NodeResult<
        'tree,
        anon_unions::DynamicVariableName_Expression_Name_VariableName<'tree>,
    > {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("name")
            .map(
                <anon_unions::DynamicVariableName_Expression_Name_VariableName<
                    'tree,
                > as ::type_sitter::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `object`.

This child has type `{array_creation_expression | dynamic_variable_name | member_access_expression | nullsafe_member_access_expression | parenthesized_expression | scoped_property_access_expression | subscript_expression | variable_name}`:

- [`ArrayCreationExpression`]
- [`DynamicVariableName`]
- [`MemberAccessExpression`]
- [`NullsafeMemberAccessExpression`]
- [`ParenthesizedExpression`]
- [`ScopedPropertyAccessExpression`]
- [`SubscriptExpression`]
- [`VariableName`]
*/
    #[inline]
    pub fn object(
        &self,
    ) -> ::type_sitter::NodeResult<
        'tree,
        anon_unions::Anon314770949262501397840657149534365951507<'tree>,
    > {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("object")
            .map(
                <anon_unions::Anon314770949262501397840657149534365951507<
                    'tree,
                > as ::type_sitter::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for MemberAccessExpression<'tree> {
    type WithLifetime<'a> = MemberAccessExpression<'a>;
    const KIND: &'static str = "member_access_expression";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "member_access_expression" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "member_access_expression");
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
/**Typed node `name`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Name<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Name<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Name<'tree> {
    type WithLifetime<'a> = Name<'a>;
    const KIND: &'static str = "name";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "name" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "name");
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
/**Typed node `named_type`

This node has a named child of type `{name | qualified_name | relative_name}`:

- [`Name`]
- [`QualifiedName`]
- [`RelativeName`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct NamedType<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> NamedType<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChild<'tree> for NamedType<'tree> {
    type Child = anon_unions::Name_QualifiedName_RelativeName<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for NamedType<'tree> {
    type WithLifetime<'a> = NamedType<'a>;
    const KIND: &'static str = "named_type";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "named_type" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "named_type");
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
/**Typed node `namespace_name`

This node has named children of type `name+` ([`Name`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct NamespaceName<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> NamespaceName<'tree> {
    /**Get the node's not-extra named children.

These children have type `name+` ([`Name`])*/
    /**

This is guaranteed to return at least one child.*/
    #[inline]
    pub fn names<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<'tree, Name<'tree>>,
    > + 'a {
        ::type_sitter::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<Name<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for NamespaceName<'tree> {
    type Child = Name<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for NamespaceName<'tree> {
    type WithLifetime<'a> = NamespaceName<'a>;
    const KIND: &'static str = "namespace_name";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "namespace_name" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "namespace_name");
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
/**Typed node `null`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Null<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Null<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Null<'tree> {
    type WithLifetime<'a> = Null<'a>;
    const KIND: &'static str = "null";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "null" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "null");
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
/**Typed node `nullsafe_member_access_expression`

This node has these fields:

- `name`: `{dynamic_variable_name | expression | name | variable_name}` ([`DynamicVariableName`] | [`Expression`] | [`Name`] | [`VariableName`])
- `object`: `{dynamic_variable_name | member_access_expression | nullsafe_member_access_expression | scoped_property_access_expression | subscript_expression | variable_name}` ([`DynamicVariableName`] | [`MemberAccessExpression`] | [`NullsafeMemberAccessExpression`] | [`ScopedPropertyAccessExpression`] | [`SubscriptExpression`] | [`VariableName`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct NullsafeMemberAccessExpression<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> NullsafeMemberAccessExpression<'tree> {
    /**Get the field `name`.

This child has type `{dynamic_variable_name | expression | name | variable_name}`:

- [`DynamicVariableName`]
- [`Expression`]
- [`Name`]
- [`VariableName`]
*/
    #[inline]
    pub fn name(
        &self,
    ) -> ::type_sitter::NodeResult<
        'tree,
        anon_unions::DynamicVariableName_Expression_Name_VariableName<'tree>,
    > {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("name")
            .map(
                <anon_unions::DynamicVariableName_Expression_Name_VariableName<
                    'tree,
                > as ::type_sitter::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `object`.

This child has type `{dynamic_variable_name | member_access_expression | nullsafe_member_access_expression | scoped_property_access_expression | subscript_expression | variable_name}`:

- [`DynamicVariableName`]
- [`MemberAccessExpression`]
- [`NullsafeMemberAccessExpression`]
- [`ScopedPropertyAccessExpression`]
- [`SubscriptExpression`]
- [`VariableName`]
*/
    #[inline]
    pub fn object(
        &self,
    ) -> ::type_sitter::NodeResult<
        'tree,
        anon_unions::Anon128718376371871210692312782388999108927<'tree>,
    > {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("object")
            .map(
                <anon_unions::Anon128718376371871210692312782388999108927<
                    'tree,
                > as ::type_sitter::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for NullsafeMemberAccessExpression<'tree> {
    type WithLifetime<'a> = NullsafeMemberAccessExpression<'a>;
    const KIND: &'static str = "nullsafe_member_access_expression";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "nullsafe_member_access_expression" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "nullsafe_member_access_expression");
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

This node has named children of type `{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | expression | fragment | keyword | livewire | loop | once | php_statement | props | script_element | stack | style_element | switch | text | verbatim}+`:

- [`Comment`]
- [`Conditional`]
- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Element`]
- [`Entity`]
- [`ErroneousEndTag`]
- [`Expression`]
- [`Fragment`]
- [`Keyword`]
- [`Livewire`]
- [`Loop`]
- [`Once`]
- [`PhpStatement`]
- [`Props`]
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
    type Child = anon_unions::Anon245876123772096930279926916143829070986<'tree>;
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
/**Typed node `operation`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Operation<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Operation<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Operation<'tree> {
    type WithLifetime<'a> = Operation<'a>;
    const KIND: &'static str = "operation";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "operation" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "operation");
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
/**Typed node `optional_type`

This node has a named child of type `{named_type | primitive_type}`:

- [`NamedType`]
- [`PrimitiveType`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct OptionalType<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> OptionalType<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChild<'tree> for OptionalType<'tree> {
    type Child = anon_unions::NamedType_PrimitiveType<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for OptionalType<'tree> {
    type WithLifetime<'a> = OptionalType<'a>;
    const KIND: &'static str = "optional_type";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "optional_type" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "optional_type");
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

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Parameter<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Parameter<'tree> {}
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
/**Typed node `parenthesized_expression`

This node has a named child of type `expression` ([`Expression`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ParenthesizedExpression<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ParenthesizedExpression<'tree> {
    /**Get the node's only not-extra named child.

This child has type `expression` ([`Expression`])*/
    #[inline]
    pub fn expression(&self) -> ::type_sitter::NodeResult<'tree, Expression<'tree>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .map(|i| ::type_sitter::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(<Expression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChild<'tree> for ParenthesizedExpression<'tree> {
    type Child = Expression<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for ParenthesizedExpression<'tree> {
    type WithLifetime<'a> = ParenthesizedExpression<'a>;
    const KIND: &'static str = "parenthesized_expression";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "parenthesized_expression" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "parenthesized_expression");
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

This node has named children of type `{directive | directive_end | directive_start | expression | php_end_tag | php_only | php_tag}*`:

- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Expression`]
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
    type Child = anon_unions::Directive_DirectiveEnd_DirectiveStart_Expression_PhpEndTag_PhpOnly_PhpTag<
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
/**Typed node `primary_expression`

This node has a named child of type `{anonymous_function | array_creation_expression | arrow_function | class_constant_access_expression | function_call_expression | literal | member_access_expression | name | parenthesized_expression | qualified_name | relative_name | subscript_expression | update_expression | variable_name}`:

- [`AnonymousFunction`]
- [`ArrayCreationExpression`]
- [`ArrowFunction`]
- [`ClassConstantAccessExpression`]
- [`FunctionCallExpression`]
- [`Literal`]
- [`MemberAccessExpression`]
- [`Name`]
- [`ParenthesizedExpression`]
- [`QualifiedName`]
- [`RelativeName`]
- [`SubscriptExpression`]
- [`UpdateExpression`]
- [`VariableName`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct PrimaryExpression<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> PrimaryExpression<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChild<'tree> for PrimaryExpression<'tree> {
    type Child = anon_unions::Anon307706018955403244961867585570518755608<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for PrimaryExpression<'tree> {
    type WithLifetime<'a> = PrimaryExpression<'a>;
    const KIND: &'static str = "primary_expression";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "primary_expression" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "primary_expression");
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
/**Typed node `primitive_type`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct PrimitiveType<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> PrimitiveType<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for PrimitiveType<'tree> {
    type WithLifetime<'a> = PrimitiveType<'a>;
    const KIND: &'static str = "primitive_type";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "primitive_type" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "primitive_type");
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
/**Typed node `property_hook_list`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct PropertyHookList<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> PropertyHookList<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for PropertyHookList<'tree> {
    type WithLifetime<'a> = PropertyHookList<'a>;
    const KIND: &'static str = "property_hook_list";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "property_hook_list" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "property_hook_list");
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
/**Typed node `property_promotion_parameter`

This node has these fields:

- `attributes`: `attribute_list?` ([`AttributeList`])
- `default_value`: `expression?` ([`Expression`])
- `name`: `{by_ref | variable_name}` ([`ByRef`] | [`VariableName`])
- `readonly`: `readonly_modifier?` ([`ReadonlyModifier`])
- `type`: `type?` ([`Type`])
- `visibility`: `visibility_modifier` ([`VisibilityModifier`])

And an optional additional named child of type `property_hook_list?` ([`PropertyHookList`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct PropertyPromotionParameter<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> PropertyPromotionParameter<'tree> {
    /**Get the optional field `attributes`.

This child has type `attribute_list?` ([`AttributeList`])*/
    #[inline]
    pub fn attributes(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, AttributeList<'tree>>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("attributes")
            .map(<AttributeList<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
    /**Get the optional field `default_value`.

This child has type `expression?` ([`Expression`])*/
    #[inline]
    pub fn default_value(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, Expression<'tree>>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("default_value")
            .map(<Expression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
    /**Get the field `name`.

This child has type `{by_ref | variable_name}`:

- [`ByRef`]
- [`VariableName`]
*/
    #[inline]
    pub fn name(
        &self,
    ) -> ::type_sitter::NodeResult<'tree, anon_unions::ByRef_VariableName<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("name")
            .map(
                <anon_unions::ByRef_VariableName<
                    'tree,
                > as ::type_sitter::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the optional field `readonly`.

This child has type `readonly_modifier?` ([`ReadonlyModifier`])*/
    #[inline]
    pub fn readonly(
        &self,
    ) -> ::std::option::Option<
        ::type_sitter::NodeResult<'tree, ReadonlyModifier<'tree>>,
    > {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("readonly")
            .map(<ReadonlyModifier<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
    /**Get the optional field `type`.

This child has type `type?` ([`Type`])*/
    #[inline]
    pub fn r#type(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, Type<'tree>>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("type")
            .map(<Type<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
    /**Get the field `visibility`.

This child has type `visibility_modifier` ([`VisibilityModifier`])*/
    #[inline]
    pub fn visibility(
        &self,
    ) -> ::type_sitter::NodeResult<'tree, VisibilityModifier<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("visibility")
            .map(<VisibilityModifier<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the node's only non-field not-extra named child, if it has one.

This child has type `property_hook_list?` ([`PropertyHookList`])*/
    #[inline]
    pub fn property_hook_list(
        &self,
    ) -> ::std::option::Option<
        ::type_sitter::NodeResult<'tree, PropertyHookList<'tree>>,
    > {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .filter(|i| {
                ::type_sitter::Node::raw(self)
                    .field_name_for_named_child(*i as _)
                    .is_none()
            })
            .map(|i| ::type_sitter::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(<PropertyHookList<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for PropertyPromotionParameter<'tree> {
    type WithLifetime<'a> = PropertyPromotionParameter<'a>;
    const KIND: &'static str = "property_promotion_parameter";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "property_promotion_parameter" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "property_promotion_parameter");
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
/**Typed node `props`

This node has a named child of type `array_creation_expression` ([`ArrayCreationExpression`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Props<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Props<'tree> {
    /**Get the node's only not-extra named child.

This child has type `array_creation_expression` ([`ArrayCreationExpression`])*/
    #[inline]
    pub fn array_creation_expression(
        &self,
    ) -> ::type_sitter::NodeResult<'tree, ArrayCreationExpression<'tree>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .map(|i| ::type_sitter::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(
                <ArrayCreationExpression<
                    'tree,
                > as ::type_sitter::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChild<'tree> for Props<'tree> {
    type Child = ArrayCreationExpression<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Props<'tree> {
    type WithLifetime<'a> = Props<'a>;
    const KIND: &'static str = "props";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "props" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "props");
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
/**Typed node `qualified_name`

This node has these fields:

- `prefix`: `{\ | namespace_name}+` ([`symbols::Backslash`] | [`NamespaceName`])

And an additional named child of type `name` ([`Name`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct QualifiedName<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> QualifiedName<'tree> {
    /**Get the children of field `prefix`.

These children have type `{\ | namespace_name}+`:

- [`symbols::Backslash`]
- [`NamespaceName`]
*/
    /**

This is guaranteed to return at least one child.*/
    #[inline]
    pub fn prefixs<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<
            'tree,
            anon_unions::Backslash_NamespaceName<'tree>,
        >,
    > + 'a {
        ::type_sitter::Node::raw(self)
            .children_by_field_name("prefix", &mut c.0)
            .map(
                <anon_unions::Backslash_NamespaceName<
                    'tree,
                > as ::type_sitter::Node<'tree>>::try_from_raw,
            )
    }
    /**Get the node's only non-field not-extra named child.

This child has type `name` ([`Name`])*/
    #[inline]
    pub fn name(&self) -> ::type_sitter::NodeResult<'tree, Name<'tree>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .filter(|i| {
                ::type_sitter::Node::raw(self)
                    .field_name_for_named_child(*i as _)
                    .is_none()
            })
            .map(|i| ::type_sitter::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(<Name<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for QualifiedName<'tree> {
    type WithLifetime<'a> = QualifiedName<'a>;
    const KIND: &'static str = "qualified_name";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "qualified_name" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "qualified_name");
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

This node has named children of type `{attribute_value | comment | conditional | directive | expression | php_statement | props}*`:

- [`AttributeValue`]
- [`Comment`]
- [`Conditional`]
- [`Directive`]
- [`Expression`]
- [`PhpStatement`]
- [`Props`]

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
    type Child = anon_unions::AttributeValue_Comment_Conditional_Directive_Expression_PhpStatement_Props<
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
/**Typed node `readonly_modifier`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ReadonlyModifier<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ReadonlyModifier<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for ReadonlyModifier<'tree> {
    type WithLifetime<'a> = ReadonlyModifier<'a>;
    const KIND: &'static str = "readonly_modifier";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "readonly_modifier" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "readonly_modifier");
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
/**Typed node `reference_modifier`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ReferenceModifier<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ReferenceModifier<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for ReferenceModifier<'tree> {
    type WithLifetime<'a> = ReferenceModifier<'a>;
    const KIND: &'static str = "reference_modifier";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "reference_modifier" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "reference_modifier");
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
/**Typed node `relative_name`

This node has these fields:

- `prefix`: `{\ | namespace | namespace_name}+` ([`symbols::Backslash`] | [`unnamed::Namespace`] | [`NamespaceName`])

And an additional named child of type `name` ([`Name`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct RelativeName<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> RelativeName<'tree> {
    /**Get the children of field `prefix`.

These children have type `{\ | namespace | namespace_name}+`:

- [`symbols::Backslash`]
- [`unnamed::Namespace`]
- [`NamespaceName`]
*/
    /**

This is guaranteed to return at least one child.*/
    #[inline]
    pub fn prefixs<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<
            'tree,
            anon_unions::Backslash_Namespace_NamespaceName<'tree>,
        >,
    > + 'a {
        ::type_sitter::Node::raw(self)
            .children_by_field_name("prefix", &mut c.0)
            .map(
                <anon_unions::Backslash_Namespace_NamespaceName<
                    'tree,
                > as ::type_sitter::Node<'tree>>::try_from_raw,
            )
    }
    /**Get the node's only non-field not-extra named child.

This child has type `name` ([`Name`])*/
    #[inline]
    pub fn name(&self) -> ::type_sitter::NodeResult<'tree, Name<'tree>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .filter(|i| {
                ::type_sitter::Node::raw(self)
                    .field_name_for_named_child(*i as _)
                    .is_none()
            })
            .map(|i| ::type_sitter::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(<Name<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for RelativeName<'tree> {
    type WithLifetime<'a> = RelativeName<'a>;
    const KIND: &'static str = "relative_name";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "relative_name" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "relative_name");
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
/**Typed node `relative_scope`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct RelativeScope<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> RelativeScope<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for RelativeScope<'tree> {
    type WithLifetime<'a> = RelativeScope<'a>;
    const KIND: &'static str = "relative_scope";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "relative_scope" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "relative_scope");
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
/**Typed node `scoped_property_access_expression`

This node has these fields:

- `name`: `{dynamic_variable_name | variable_name}` ([`DynamicVariableName`] | [`VariableName`])
- `scope`: `{dynamic_variable_name | member_access_expression | name | nullsafe_member_access_expression | qualified_name | scoped_property_access_expression | subscript_expression | variable_name}` ([`DynamicVariableName`] | [`MemberAccessExpression`] | [`Name`] | [`NullsafeMemberAccessExpression`] | [`QualifiedName`] | [`ScopedPropertyAccessExpression`] | [`SubscriptExpression`] | [`VariableName`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ScopedPropertyAccessExpression<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> ScopedPropertyAccessExpression<'tree> {
    /**Get the field `name`.

This child has type `{dynamic_variable_name | variable_name}`:

- [`DynamicVariableName`]
- [`VariableName`]
*/
    #[inline]
    pub fn name(
        &self,
    ) -> ::type_sitter::NodeResult<
        'tree,
        anon_unions::DynamicVariableName_VariableName<'tree>,
    > {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("name")
            .map(
                <anon_unions::DynamicVariableName_VariableName<
                    'tree,
                > as ::type_sitter::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `scope`.

This child has type `{dynamic_variable_name | member_access_expression | name | nullsafe_member_access_expression | qualified_name | scoped_property_access_expression | subscript_expression | variable_name}`:

- [`DynamicVariableName`]
- [`MemberAccessExpression`]
- [`Name`]
- [`NullsafeMemberAccessExpression`]
- [`QualifiedName`]
- [`ScopedPropertyAccessExpression`]
- [`SubscriptExpression`]
- [`VariableName`]
*/
    #[inline]
    pub fn scope(
        &self,
    ) -> ::type_sitter::NodeResult<
        'tree,
        anon_unions::Anon182904659458560044835768290653647831886<'tree>,
    > {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("scope")
            .map(
                <anon_unions::Anon182904659458560044835768290653647831886<
                    'tree,
                > as ::type_sitter::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for ScopedPropertyAccessExpression<'tree> {
    type WithLifetime<'a> = ScopedPropertyAccessExpression<'a>;
    const KIND: &'static str = "scoped_property_access_expression";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "scoped_property_access_expression" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "scoped_property_access_expression");
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

This node has named children of type `{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | expression | keyword | livewire | loop | parameter | php_statement | props | script_element | stack | style_element | switch | text | verbatim}+`:

- [`Comment`]
- [`Conditional`]
- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Element`]
- [`Entity`]
- [`ErroneousEndTag`]
- [`Expression`]
- [`Keyword`]
- [`Livewire`]
- [`Loop`]
- [`Parameter`]
- [`PhpStatement`]
- [`Props`]
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
    type Child = anon_unions::Anon152400393147957412014589918373492536309<'tree>;
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

This node has these fields:

- `attribute`: `attribute*` ([`Attribute`])

And an additional named child of type `tag_name` ([`TagName`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct SelfClosingTag<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> SelfClosingTag<'tree> {
    /**Get the children of field `attribute`.

These children have type `attribute*` ([`Attribute`])*/
    #[inline]
    pub fn attributes<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<'tree, Attribute<'tree>>,
    > + 'a {
        ::type_sitter::Node::raw(self)
            .children_by_field_name("attribute", &mut c.0)
            .map(<Attribute<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
    /**Get the node's only non-field not-extra named child.

This child has type `tag_name` ([`TagName`])*/
    #[inline]
    pub fn tag_name(&self) -> ::type_sitter::NodeResult<'tree, TagName<'tree>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .filter(|i| {
                ::type_sitter::Node::raw(self)
                    .field_name_for_named_child(*i as _)
                    .is_none()
            })
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
/**Typed node `simple_parameter`

This node has these fields:

- `attributes`: `attribute_list?` ([`AttributeList`])
- `default_value`: `expression?` ([`Expression`])
- `name`: `variable_name` ([`VariableName`])
- `reference_modifier`: `reference_modifier?` ([`ReferenceModifier`])
- `type`: `type?` ([`Type`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct SimpleParameter<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> SimpleParameter<'tree> {
    /**Get the optional field `attributes`.

This child has type `attribute_list?` ([`AttributeList`])*/
    #[inline]
    pub fn attributes(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, AttributeList<'tree>>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("attributes")
            .map(<AttributeList<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
    /**Get the optional field `default_value`.

This child has type `expression?` ([`Expression`])*/
    #[inline]
    pub fn default_value(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, Expression<'tree>>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("default_value")
            .map(<Expression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
    /**Get the field `name`.

This child has type `variable_name` ([`VariableName`])*/
    #[inline]
    pub fn name(&self) -> ::type_sitter::NodeResult<'tree, VariableName<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("name")
            .map(<VariableName<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the optional field `reference_modifier`.

This child has type `reference_modifier?` ([`ReferenceModifier`])*/
    #[inline]
    pub fn reference_modifier(
        &self,
    ) -> ::std::option::Option<
        ::type_sitter::NodeResult<'tree, ReferenceModifier<'tree>>,
    > {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("reference_modifier")
            .map(<ReferenceModifier<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
    /**Get the optional field `type`.

This child has type `type?` ([`Type`])*/
    #[inline]
    pub fn r#type(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, Type<'tree>>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("type")
            .map(<Type<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for SimpleParameter<'tree> {
    type WithLifetime<'a> = SimpleParameter<'a>;
    const KIND: &'static str = "simple_parameter";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "simple_parameter" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "simple_parameter");
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

This node has named children of type `{comment | directive | directive_end | directive_start | element | entity | erroneous_end_tag | expression | keyword | php_statement | props | script_element | style_element | switch | text}+`:

- [`Comment`]
- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Element`]
- [`Entity`]
- [`ErroneousEndTag`]
- [`Expression`]
- [`Keyword`]
- [`PhpStatement`]
- [`Props`]
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
    type Child = anon_unions::Anon263080000079809860076926378912201300104<'tree>;
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

This node has these fields:

- `attribute`: `attribute*` ([`Attribute`])

And an additional named child of type `tag_name` ([`TagName`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct StartTag<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> StartTag<'tree> {
    /**Get the children of field `attribute`.

These children have type `attribute*` ([`Attribute`])*/
    #[inline]
    pub fn attributes<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<'tree, Attribute<'tree>>,
    > + 'a {
        ::type_sitter::Node::raw(self)
            .children_by_field_name("attribute", &mut c.0)
            .map(<Attribute<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
    /**Get the node's only non-field not-extra named child.

This child has type `tag_name` ([`TagName`])*/
    #[inline]
    pub fn tag_name(&self) -> ::type_sitter::NodeResult<'tree, TagName<'tree>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .filter(|i| {
                ::type_sitter::Node::raw(self)
                    .field_name_for_named_child(*i as _)
                    .is_none()
            })
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
/**Typed node `static_modifier`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct StaticModifier<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> StaticModifier<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for StaticModifier<'tree> {
    type WithLifetime<'a> = StaticModifier<'a>;
    const KIND: &'static str = "static_modifier";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "static_modifier" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "static_modifier");
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
/**Typed node `string`

This node has no named children
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct String<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> String<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for String<'tree> {
    type WithLifetime<'a> = String<'a>;
    const KIND: &'static str = "string";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "string" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "string");
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
/**Typed node `subscript_expression`

This node has these fields:

- `object`: `{array_creation_expression | member_access_expression | parenthesized_expression | subscript_expression | variable_name}?` ([`ArrayCreationExpression`] | [`MemberAccessExpression`] | [`ParenthesizedExpression`] | [`SubscriptExpression`] | [`VariableName`])

And additional named children of type `{dynamic_variable_name | expression | member_access_expression | nullsafe_member_access_expression | scoped_property_access_expression | subscript_expression | variable_name}*`:

- [`DynamicVariableName`]
- [`Expression`]
- [`MemberAccessExpression`]
- [`NullsafeMemberAccessExpression`]
- [`ScopedPropertyAccessExpression`]
- [`SubscriptExpression`]
- [`VariableName`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct SubscriptExpression<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> SubscriptExpression<'tree> {
    /**Get the optional field `object`.

This child has type `{array_creation_expression | member_access_expression | parenthesized_expression | subscript_expression | variable_name}?`:

- [`ArrayCreationExpression`]
- [`MemberAccessExpression`]
- [`ParenthesizedExpression`]
- [`SubscriptExpression`]
- [`VariableName`]
*/
    #[inline]
    pub fn object(
        &self,
    ) -> ::std::option::Option<
        ::type_sitter::NodeResult<
            'tree,
            anon_unions::Anon148842522943207813739644966039431447539<'tree>,
        >,
    > {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("object")
            .map(
                <anon_unions::Anon148842522943207813739644966039431447539<
                    'tree,
                > as ::type_sitter::Node<'tree>>::try_from_raw,
            )
    }
    /**Get the node's non-field not-extra named children.

These children have type `{dynamic_variable_name | expression | member_access_expression | nullsafe_member_access_expression | scoped_property_access_expression | subscript_expression | variable_name}*`:

- [`DynamicVariableName`]
- [`Expression`]
- [`MemberAccessExpression`]
- [`NullsafeMemberAccessExpression`]
- [`ScopedPropertyAccessExpression`]
- [`SubscriptExpression`]
- [`VariableName`]
*/
    #[inline]
    pub fn others<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<
            'tree,
            anon_unions::Anon58897980112289426888429290682076362032<'tree>,
        >,
    > + 'a {
        {
            let me = *::type_sitter::Node::raw(self);
            ::type_sitter::Node::raw(self)
                .named_children(&mut c.0)
                .enumerate()
                .filter(move |(i, n)| {
                    !n.is_extra() && me.field_name_for_named_child(*i as _).is_none()
                })
                .map(|(_, n)| n)
        }
            .map(
                <anon_unions::Anon58897980112289426888429290682076362032<
                    'tree,
                > as ::type_sitter::Node<'tree>>::try_from_raw,
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for SubscriptExpression<'tree> {
    type WithLifetime<'a> = SubscriptExpression<'a>;
    const KIND: &'static str = "subscript_expression";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "subscript_expression" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "subscript_expression");
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

This node has named children of type `{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | expression | keyword | livewire | loop | php_statement | props | script_element | style_element | text}+`:

- [`Comment`]
- [`Conditional`]
- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Element`]
- [`Entity`]
- [`ErroneousEndTag`]
- [`Expression`]
- [`Keyword`]
- [`Livewire`]
- [`Loop`]
- [`PhpStatement`]
- [`Props`]
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
    type Child = anon_unions::Anon117478531041884665886116169677315060836<'tree>;
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
/**Typed node `type`

This node has a named child of type `{disjunctive_normal_form_type | intersection_type | named_type | optional_type | primitive_type | union_type}`:

- [`DisjunctiveNormalFormType`]
- [`IntersectionType`]
- [`NamedType`]
- [`OptionalType`]
- [`PrimitiveType`]
- [`UnionType`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Type<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> Type<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChild<'tree> for Type<'tree> {
    type Child = anon_unions::DisjunctiveNormalFormType_IntersectionType_NamedType_OptionalType_PrimitiveType_UnionType<
        'tree,
    >;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Type<'tree> {
    type WithLifetime<'a> = Type<'a>;
    const KIND: &'static str = "type";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "type" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "type");
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
/**Typed node `unary_op_expression`

This node has these fields:

- `argument`: `expression` ([`Expression`])
- `operator`: `{! | + | - | ~}` ([`symbols::Not`] | [`symbols::Add`] | [`symbols::Sub`] | [`symbols::BitNot`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct UnaryOpExpression<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> UnaryOpExpression<'tree> {
    /**Get the field `argument`.

This child has type `expression` ([`Expression`])*/
    #[inline]
    pub fn argument(&self) -> ::type_sitter::NodeResult<'tree, Expression<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("argument")
            .map(<Expression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `operator`.

This child has type `{! | + | - | ~}`:

- [`symbols::Not`]
- [`symbols::Add`]
- [`symbols::Sub`]
- [`symbols::BitNot`]
*/
    #[inline]
    pub fn operator(
        &self,
    ) -> ::type_sitter::NodeResult<'tree, anon_unions::Not_Add_Sub_BitNot<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("operator")
            .map(
                <anon_unions::Not_Add_Sub_BitNot<
                    'tree,
                > as ::type_sitter::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for UnaryOpExpression<'tree> {
    type WithLifetime<'a> = UnaryOpExpression<'a>;
    const KIND: &'static str = "unary_op_expression";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "unary_op_expression" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "unary_op_expression");
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
/**Typed node `union_type`

This node has named children of type `{named_type | optional_type | primitive_type}+`:

- [`NamedType`]
- [`OptionalType`]
- [`PrimitiveType`]

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct UnionType<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> UnionType<'tree> {}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for UnionType<'tree> {
    type Child = anon_unions::NamedType_OptionalType_PrimitiveType<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for UnionType<'tree> {
    type WithLifetime<'a> = UnionType<'a>;
    const KIND: &'static str = "union_type";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "union_type" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "union_type");
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
/**Typed node `update_expression`

This node has these fields:

- `argument`: `{member_access_expression | subscript_expression | variable_name}` ([`MemberAccessExpression`] | [`SubscriptExpression`] | [`VariableName`])
- `operator`: `{++ | --}` ([`symbols::AddAdd`] | [`symbols::SubSub`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct UpdateExpression<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> UpdateExpression<'tree> {
    /**Get the field `argument`.

This child has type `{member_access_expression | subscript_expression | variable_name}`:

- [`MemberAccessExpression`]
- [`SubscriptExpression`]
- [`VariableName`]
*/
    #[inline]
    pub fn argument(
        &self,
    ) -> ::type_sitter::NodeResult<
        'tree,
        anon_unions::MemberAccessExpression_SubscriptExpression_VariableName<'tree>,
    > {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("argument")
            .map(
                <anon_unions::MemberAccessExpression_SubscriptExpression_VariableName<
                    'tree,
                > as ::type_sitter::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the field `operator`.

This child has type `{++ | --}`:

- [`symbols::AddAdd`]
- [`symbols::SubSub`]
*/
    #[inline]
    pub fn operator(
        &self,
    ) -> ::type_sitter::NodeResult<'tree, anon_unions::AddAdd_SubSub<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("operator")
            .map(
                <anon_unions::AddAdd_SubSub<
                    'tree,
                > as ::type_sitter::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for UpdateExpression<'tree> {
    type WithLifetime<'a> = UpdateExpression<'a>;
    const KIND: &'static str = "update_expression";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "update_expression" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "update_expression");
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
/**Typed node `variable_name`

This node has a named child of type `name` ([`Name`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct VariableName<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> VariableName<'tree> {
    /**Get the node's only not-extra named child.

This child has type `name` ([`Name`])*/
    #[inline]
    pub fn name(&self) -> ::type_sitter::NodeResult<'tree, Name<'tree>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .map(|i| ::type_sitter::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(<Name<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChild<'tree> for VariableName<'tree> {
    type Child = Name<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for VariableName<'tree> {
    type WithLifetime<'a> = VariableName<'a>;
    const KIND: &'static str = "variable_name";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "variable_name" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "variable_name");
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
/**Typed node `variadic_parameter`

This node has these fields:

- `attributes`: `attribute_list?` ([`AttributeList`])
- `name`: `variable_name` ([`VariableName`])
- `reference_modifier`: `reference_modifier?` ([`ReferenceModifier`])
- `type`: `type?` ([`Type`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct VariadicParameter<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> VariadicParameter<'tree> {
    /**Get the optional field `attributes`.

This child has type `attribute_list?` ([`AttributeList`])*/
    #[inline]
    pub fn attributes(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, AttributeList<'tree>>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("attributes")
            .map(<AttributeList<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
    /**Get the field `name`.

This child has type `variable_name` ([`VariableName`])*/
    #[inline]
    pub fn name(&self) -> ::type_sitter::NodeResult<'tree, VariableName<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("name")
            .map(<VariableName<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
    /**Get the optional field `reference_modifier`.

This child has type `reference_modifier?` ([`ReferenceModifier`])*/
    #[inline]
    pub fn reference_modifier(
        &self,
    ) -> ::std::option::Option<
        ::type_sitter::NodeResult<'tree, ReferenceModifier<'tree>>,
    > {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("reference_modifier")
            .map(<ReferenceModifier<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
    /**Get the optional field `type`.

This child has type `type?` ([`Type`])*/
    #[inline]
    pub fn r#type(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, Type<'tree>>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("type")
            .map(<Type<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for VariadicParameter<'tree> {
    type WithLifetime<'a> = VariadicParameter<'a>;
    const KIND: &'static str = "variadic_parameter";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "variadic_parameter" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "variadic_parameter");
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
/**Typed node `variadic_unpacking`

This node has a named child of type `expression` ([`Expression`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct VariadicUnpacking<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> VariadicUnpacking<'tree> {
    /**Get the node's only not-extra named child.

This child has type `expression` ([`Expression`])*/
    #[inline]
    pub fn expression(&self) -> ::type_sitter::NodeResult<'tree, Expression<'tree>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .map(|i| ::type_sitter::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(<Expression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChild<'tree> for VariadicUnpacking<'tree> {
    type Child = Expression<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for VariadicUnpacking<'tree> {
    type WithLifetime<'a> = VariadicUnpacking<'a>;
    const KIND: &'static str = "variadic_unpacking";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "variadic_unpacking" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "variadic_unpacking");
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

This node has named children of type `{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | expression | fragment | keyword | loop | once | php_statement | props | script_element | section | stack | style_element | switch | text | verbatim}+`:

- [`Comment`]
- [`Conditional`]
- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Element`]
- [`Entity`]
- [`ErroneousEndTag`]
- [`Expression`]
- [`Fragment`]
- [`Keyword`]
- [`Loop`]
- [`Once`]
- [`PhpStatement`]
- [`Props`]
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
    type Child = anon_unions::Anon240876909159937813961889479930182217457<'tree>;
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
/**Typed node `visibility_modifier`

This node has an optional named child of type `operation?` ([`Operation`])
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct VisibilityModifier<'tree>(::type_sitter::raw::Node<'tree>);
#[automatically_derived]
#[allow(unused)]
impl<'tree> VisibilityModifier<'tree> {
    /**Get the node's only not-extra named child, if it has one.

This child has type `operation?` ([`Operation`])*/
    #[inline]
    pub fn operation(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, Operation<'tree>>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .map(|i| ::type_sitter::Node::raw(self).named_child(i).unwrap())
            .filter(|n| !n.is_extra())
            .next()
            .map(<Operation<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::HasOptionalChild<'tree> for VisibilityModifier<'tree> {
    type Child = Operation<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for VisibilityModifier<'tree> {
    type WithLifetime<'a> = VisibilityModifier<'a>;
    const KIND: &'static str = "visibility_modifier";
    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "visibility_modifier" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }
    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "visibility_modifier");
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
    /**Typed node `and`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct And<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> And<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for And<'tree> {
        type WithLifetime<'a> = And<'a>;
        const KIND: &'static str = "and";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "and" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "and");
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
    /**Typed node `array`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Array<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Array<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Array<'tree> {
        type WithLifetime<'a> = Array<'a>;
        const KIND: &'static str = "array";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "array" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "array");
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
    /**Typed node `bool`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Bool<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Bool<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Bool<'tree> {
        type WithLifetime<'a> = Bool<'a>;
        const KIND: &'static str = "bool";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "bool" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "bool");
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
    /**Typed node `float`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Float<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Float<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Float<'tree> {
        type WithLifetime<'a> = Float<'a>;
        const KIND: &'static str = "float";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "float" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "float");
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
    /**Typed node `fn`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Fn<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Fn<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Fn<'tree> {
        type WithLifetime<'a> = Fn<'a>;
        const KIND: &'static str = "fn";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "fn" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "fn");
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
    /**Typed node `function`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Function<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Function<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Function<'tree> {
        type WithLifetime<'a> = Function<'a>;
        const KIND: &'static str = "function";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "function" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "function");
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
    /**Typed node `instanceof`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Instanceof<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Instanceof<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Instanceof<'tree> {
        type WithLifetime<'a> = Instanceof<'a>;
        const KIND: &'static str = "instanceof";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "instanceof" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "instanceof");
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
    /**Typed node `int`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Int<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Int<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Int<'tree> {
        type WithLifetime<'a> = Int<'a>;
        const KIND: &'static str = "int";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "int" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "int");
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
    /**Typed node `namespace`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Namespace<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Namespace<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Namespace<'tree> {
        type WithLifetime<'a> = Namespace<'a>;
        const KIND: &'static str = "namespace";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "namespace" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "namespace");
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
    /**Typed node `null`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Null<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Null<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Null<'tree> {
        type WithLifetime<'a> = Null<'a>;
        const KIND: &'static str = "null";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "null" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "null");
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
    /**Typed node `object`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Object<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Object<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Object<'tree> {
        type WithLifetime<'a> = Object<'a>;
        const KIND: &'static str = "object";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "object" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "object");
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
    /**Typed node `or`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Or<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Or<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Or<'tree> {
        type WithLifetime<'a> = Or<'a>;
        const KIND: &'static str = "or";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "or" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "or");
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
    /**Typed node `parent`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Parent<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Parent<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Parent<'tree> {
        type WithLifetime<'a> = Parent<'a>;
        const KIND: &'static str = "parent";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "parent" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "parent");
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
    /**Typed node `private`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Private<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Private<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Private<'tree> {
        type WithLifetime<'a> = Private<'a>;
        const KIND: &'static str = "private";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "private" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "private");
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
    /**Typed node `protected`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Protected<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Protected<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Protected<'tree> {
        type WithLifetime<'a> = Protected<'a>;
        const KIND: &'static str = "protected";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "protected" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "protected");
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
    /**Typed node `public`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Public<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Public<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Public<'tree> {
        type WithLifetime<'a> = Public<'a>;
        const KIND: &'static str = "public";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "public" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "public");
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
    /**Typed node `readonly`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Readonly<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Readonly<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Readonly<'tree> {
        type WithLifetime<'a> = Readonly<'a>;
        const KIND: &'static str = "readonly";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "readonly" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "readonly");
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
    /**Typed node `self`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Self_<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Self_<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Self_<'tree> {
        type WithLifetime<'a> = Self_<'a>;
        const KIND: &'static str = "self";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "self" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "self");
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
    /**Typed node `static`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Static<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Static<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Static<'tree> {
        type WithLifetime<'a> = Static<'a>;
        const KIND: &'static str = "static";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "static" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "static");
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
    /**Typed node `string`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct String<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> String<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for String<'tree> {
        type WithLifetime<'a> = String<'a>;
        const KIND: &'static str = "string";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "string" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "string");
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
    /**Typed node `use`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Use<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Use<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Use<'tree> {
        type WithLifetime<'a> = Use<'a>;
        const KIND: &'static str = "use";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "use" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "use");
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
    /**Typed node `xor`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Xor<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Xor<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Xor<'tree> {
        type WithLifetime<'a> = Xor<'a>;
        const KIND: &'static str = "xor";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "xor" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "xor");
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
    /**Typed node `!`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Not<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Not<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Not<'tree> {
        type WithLifetime<'a> = Not<'a>;
        const KIND: &'static str = "!";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "!" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "!");
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
    /**Typed node `!=`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct NotEq<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> NotEq<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for NotEq<'tree> {
        type WithLifetime<'a> = NotEq<'a>;
        const KIND: &'static str = "!=";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "!=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "!=");
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
    /**Typed node `!==`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct NotEqEq<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> NotEqEq<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for NotEqEq<'tree> {
        type WithLifetime<'a> = NotEqEq<'a>;
        const KIND: &'static str = "!==";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "!==" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "!==");
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
    /**Typed node `#[`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct HashLBracket<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> HashLBracket<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for HashLBracket<'tree> {
        type WithLifetime<'a> = HashLBracket<'a>;
        const KIND: &'static str = "#[";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "#[" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "#[");
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
    /**Typed node `$`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Dollar<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Dollar<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Dollar<'tree> {
        type WithLifetime<'a> = Dollar<'a>;
        const KIND: &'static str = "$";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "$" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "$");
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
    /**Typed node `%`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Mod<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Mod<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Mod<'tree> {
        type WithLifetime<'a> = Mod<'a>;
        const KIND: &'static str = "%";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "%" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "%");
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
    /**Typed node `&`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct And<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> And<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for And<'tree> {
        type WithLifetime<'a> = And<'a>;
        const KIND: &'static str = "&";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "&" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "&");
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
    /**Typed node `&&`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct AndAnd<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> AndAnd<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for AndAnd<'tree> {
        type WithLifetime<'a> = AndAnd<'a>;
        const KIND: &'static str = "&&";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "&&" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "&&");
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
    /**Typed node `*`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Mul<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Mul<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Mul<'tree> {
        type WithLifetime<'a> = Mul<'a>;
        const KIND: &'static str = "*";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "*" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "*");
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
    /**Typed node `**`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct MulMul<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> MulMul<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for MulMul<'tree> {
        type WithLifetime<'a> = MulMul<'a>;
        const KIND: &'static str = "**";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "**" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "**");
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
    /**Typed node `+`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Add<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Add<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Add<'tree> {
        type WithLifetime<'a> = Add<'a>;
        const KIND: &'static str = "+";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "+" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "+");
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
    /**Typed node `++`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct AddAdd<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> AddAdd<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for AddAdd<'tree> {
        type WithLifetime<'a> = AddAdd<'a>;
        const KIND: &'static str = "++";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "++" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "++");
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
    /**Typed node `-`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Sub<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Sub<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Sub<'tree> {
        type WithLifetime<'a> = Sub<'a>;
        const KIND: &'static str = "-";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "-" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "-");
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
    /**Typed node `--`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct SubSub<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> SubSub<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for SubSub<'tree> {
        type WithLifetime<'a> = SubSub<'a>;
        const KIND: &'static str = "--";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "--" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "--");
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
    /**Typed node `->`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct SubGt<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> SubGt<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for SubGt<'tree> {
        type WithLifetime<'a> = SubGt<'a>;
        const KIND: &'static str = "->";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "->" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "->");
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
    /**Typed node `.`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Dot<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Dot<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Dot<'tree> {
        type WithLifetime<'a> = Dot<'a>;
        const KIND: &'static str = ".";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "." {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), ".");
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
    /**Typed node `...`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct DotDotDot<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> DotDotDot<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for DotDotDot<'tree> {
        type WithLifetime<'a> = DotDotDot<'a>;
        const KIND: &'static str = "...";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "..." {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "...");
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
    /**Typed node `/`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Div<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Div<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Div<'tree> {
        type WithLifetime<'a> = Div<'a>;
        const KIND: &'static str = "/";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "/" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "/");
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
    /**Typed node `:`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Colon<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Colon<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Colon<'tree> {
        type WithLifetime<'a> = Colon<'a>;
        const KIND: &'static str = ":";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == ":" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), ":");
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
    /**Typed node `::`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct ColonColon<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> ColonColon<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for ColonColon<'tree> {
        type WithLifetime<'a> = ColonColon<'a>;
        const KIND: &'static str = "::";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "::" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "::");
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
    /**Typed node `<<`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct LtLt<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> LtLt<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for LtLt<'tree> {
        type WithLifetime<'a> = LtLt<'a>;
        const KIND: &'static str = "<<";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "<<" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "<<");
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
    /**Typed node `<=`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct LtEq<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> LtEq<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for LtEq<'tree> {
        type WithLifetime<'a> = LtEq<'a>;
        const KIND: &'static str = "<=";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "<=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "<=");
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
    /**Typed node `<=>`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct LtEqGt<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> LtEqGt<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for LtEqGt<'tree> {
        type WithLifetime<'a> = LtEqGt<'a>;
        const KIND: &'static str = "<=>";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "<=>" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "<=>");
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
    /**Typed node `<>`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct LtGt<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> LtGt<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for LtGt<'tree> {
        type WithLifetime<'a> = LtGt<'a>;
        const KIND: &'static str = "<>";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "<>" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "<>");
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
    /**Typed node `==`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct EqEq<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> EqEq<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for EqEq<'tree> {
        type WithLifetime<'a> = EqEq<'a>;
        const KIND: &'static str = "==";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "==" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "==");
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
    /**Typed node `===`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct EqEqEq<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> EqEqEq<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for EqEqEq<'tree> {
        type WithLifetime<'a> = EqEqEq<'a>;
        const KIND: &'static str = "===";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "===" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "===");
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
    /**Typed node `=>`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct EqGt<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> EqGt<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for EqGt<'tree> {
        type WithLifetime<'a> = EqGt<'a>;
        const KIND: &'static str = "=>";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "=>" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "=>");
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
    /**Typed node `>=`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct GtEq<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> GtEq<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for GtEq<'tree> {
        type WithLifetime<'a> = GtEq<'a>;
        const KIND: &'static str = ">=";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == ">=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), ">=");
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
    /**Typed node `>>`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct GtGt<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> GtGt<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for GtGt<'tree> {
        type WithLifetime<'a> = GtGt<'a>;
        const KIND: &'static str = ">>";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == ">>" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), ">>");
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
    /**Typed node `?`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Question<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Question<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Question<'tree> {
        type WithLifetime<'a> = Question<'a>;
        const KIND: &'static str = "?";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "?" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "?");
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
    /**Typed node `?->`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct QuestionSubGt<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> QuestionSubGt<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for QuestionSubGt<'tree> {
        type WithLifetime<'a> = QuestionSubGt<'a>;
        const KIND: &'static str = "?->";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "?->" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "?->");
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
    /**Typed node `??`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct QuestionQuestion<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> QuestionQuestion<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for QuestionQuestion<'tree> {
        type WithLifetime<'a> = QuestionQuestion<'a>;
        const KIND: &'static str = "??";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "??" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "??");
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
    /**Typed node `@props`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Atprops<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Atprops<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Atprops<'tree> {
        type WithLifetime<'a> = Atprops<'a>;
        const KIND: &'static str = "@props";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "@props" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "@props");
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
    /**Typed node `[`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct LBracket<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> LBracket<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for LBracket<'tree> {
        type WithLifetime<'a> = LBracket<'a>;
        const KIND: &'static str = "[";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "[" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "[");
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
    /**Typed node `\`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Backslash<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Backslash<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Backslash<'tree> {
        type WithLifetime<'a> = Backslash<'a>;
        const KIND: &'static str = "\\";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "\\" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "\\");
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
    /**Typed node `\'`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct BackslashQuote<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> BackslashQuote<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for BackslashQuote<'tree> {
        type WithLifetime<'a> = BackslashQuote<'a>;
        const KIND: &'static str = "\\'";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "\\'" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "\\'");
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
    /**Typed node `]`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct RBracket<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> RBracket<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for RBracket<'tree> {
        type WithLifetime<'a> = RBracket<'a>;
        const KIND: &'static str = "]";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "]" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "]");
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
    /**Typed node `^`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct BitXor<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> BitXor<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for BitXor<'tree> {
        type WithLifetime<'a> = BitXor<'a>;
        const KIND: &'static str = "^";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "^" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "^");
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
    /**Typed node `{`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct LBrace<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> LBrace<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for LBrace<'tree> {
        type WithLifetime<'a> = LBrace<'a>;
        const KIND: &'static str = "{";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "{" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "{");
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
    /**Typed node `|`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Or<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Or<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Or<'tree> {
        type WithLifetime<'a> = Or<'a>;
        const KIND: &'static str = "|";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "|" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "|");
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
    /**Typed node `|>`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct OrGt<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> OrGt<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for OrGt<'tree> {
        type WithLifetime<'a> = OrGt<'a>;
        const KIND: &'static str = "|>";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "|>" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "|>");
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
    /**Typed node `||`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct OrOr<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> OrOr<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for OrOr<'tree> {
        type WithLifetime<'a> = OrOr<'a>;
        const KIND: &'static str = "||";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "||" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "||");
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
    /**Typed node `}`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct RBrace<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> RBrace<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for RBrace<'tree> {
        type WithLifetime<'a> = RBrace<'a>;
        const KIND: &'static str = "}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "}" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "}");
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
    /**Typed node `~`

This node has no named children
*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct BitNot<'tree>(::type_sitter::raw::Node<'tree>);
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> BitNot<'tree> {}
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for BitNot<'tree> {
        type WithLifetime<'a> = BitNot<'a>;
        const KIND: &'static str = "~";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "~" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }
        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "~");
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
    /**One of `{++ | --}`:
- [`symbols::AddAdd`]
- [`symbols::SubSub`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum AddAdd_SubSub<'tree> {
        AddAdd(symbols::AddAdd<'tree>),
        SubSub(symbols::SubSub<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> AddAdd_SubSub<'tree> {
        ///Returns the node if it is of type `++` ([`symbols::AddAdd`]), otherwise returns `None`
        #[inline]
        pub fn as_add_add(self) -> ::std::option::Option<symbols::AddAdd<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::AddAdd(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `--` ([`symbols::SubSub`]), otherwise returns `None`
        #[inline]
        pub fn as_sub_sub(self) -> ::std::option::Option<symbols::SubSub<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::SubSub(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for AddAdd_SubSub<'tree> {
        type WithLifetime<'a> = AddAdd_SubSub<'a>;
        const KIND: &'static str = "{++ | --}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "++" => {
                    Ok(unsafe {
                        Self::AddAdd(
                            <symbols::AddAdd<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "--" => {
                    Ok(unsafe {
                        Self::SubSub(
                            <symbols::SubSub<
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
                Self::AddAdd(x) => ::type_sitter::Node::raw(x),
                Self::SubSub(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::AddAdd(x) => ::type_sitter::Node::raw_mut(x),
                Self::SubSub(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::AddAdd(x) => x.into_raw(),
                Self::SubSub(x) => x.into_raw(),
            }
        }
    }
    /**One of `{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | expression | keyword | livewire | loop | php_statement | props | script_element | style_element | text}`:
- [`Comment`]
- [`Conditional`]
- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Element`]
- [`Entity`]
- [`ErroneousEndTag`]
- [`Expression`]
- [`Keyword`]
- [`Livewire`]
- [`Loop`]
- [`PhpStatement`]
- [`Props`]
- [`ScriptElement`]
- [`StyleElement`]
- [`Text`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Anon117478531041884665886116169677315060836<'tree> {
        Comment(Comment<'tree>),
        Conditional(Conditional<'tree>),
        Directive(Directive<'tree>),
        DirectiveEnd(DirectiveEnd<'tree>),
        DirectiveStart(DirectiveStart<'tree>),
        Element(Element<'tree>),
        Entity(Entity<'tree>),
        ErroneousEndTag(ErroneousEndTag<'tree>),
        Expression(Expression<'tree>),
        Keyword(Keyword<'tree>),
        Livewire(Livewire<'tree>),
        Loop(Loop<'tree>),
        PhpStatement(PhpStatement<'tree>),
        Props(Props<'tree>),
        ScriptElement(ScriptElement<'tree>),
        StyleElement(StyleElement<'tree>),
        Text(Text<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Anon117478531041884665886116169677315060836<'tree> {
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
        ///Returns the node if it is of type `expression` ([`Expression`]), otherwise returns `None`
        #[inline]
        pub fn as_expression(self) -> ::std::option::Option<Expression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Expression(x) = self {
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
        ///Returns the node if it is of type `props` ([`Props`]), otherwise returns `None`
        #[inline]
        pub fn as_props(self) -> ::std::option::Option<Props<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Props(x) = self {
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
    for Anon117478531041884665886116169677315060836<'tree> {
        type WithLifetime<'a> = Anon117478531041884665886116169677315060836<'a>;
        const KIND: &'static str = "{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | expression | keyword | livewire | loop | php_statement | props | script_element | style_element | text}";
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
                "expression" => {
                    Ok(unsafe {
                        Self::Expression(
                            <Expression<
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
                "php_statement" => {
                    Ok(unsafe {
                        Self::PhpStatement(
                            <PhpStatement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "props" => {
                    Ok(unsafe {
                        Self::Props(
                            <Props<
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
                Self::Expression(x) => ::type_sitter::Node::raw(x),
                Self::Keyword(x) => ::type_sitter::Node::raw(x),
                Self::Livewire(x) => ::type_sitter::Node::raw(x),
                Self::Loop(x) => ::type_sitter::Node::raw(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw(x),
                Self::Props(x) => ::type_sitter::Node::raw(x),
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
                Self::Expression(x) => ::type_sitter::Node::raw_mut(x),
                Self::Keyword(x) => ::type_sitter::Node::raw_mut(x),
                Self::Livewire(x) => ::type_sitter::Node::raw_mut(x),
                Self::Loop(x) => ::type_sitter::Node::raw_mut(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw_mut(x),
                Self::Props(x) => ::type_sitter::Node::raw_mut(x),
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
                Self::Expression(x) => x.into_raw(),
                Self::Keyword(x) => x.into_raw(),
                Self::Livewire(x) => x.into_raw(),
                Self::Loop(x) => x.into_raw(),
                Self::PhpStatement(x) => x.into_raw(),
                Self::Props(x) => x.into_raw(),
                Self::ScriptElement(x) => x.into_raw(),
                Self::StyleElement(x) => x.into_raw(),
                Self::Text(x) => x.into_raw(),
            }
        }
    }
    /**One of `{dynamic_variable_name | member_access_expression | nullsafe_member_access_expression | scoped_property_access_expression | subscript_expression | variable_name}`:
- [`DynamicVariableName`]
- [`MemberAccessExpression`]
- [`NullsafeMemberAccessExpression`]
- [`ScopedPropertyAccessExpression`]
- [`SubscriptExpression`]
- [`VariableName`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Anon128718376371871210692312782388999108927<'tree> {
        DynamicVariableName(DynamicVariableName<'tree>),
        MemberAccessExpression(MemberAccessExpression<'tree>),
        NullsafeMemberAccessExpression(NullsafeMemberAccessExpression<'tree>),
        ScopedPropertyAccessExpression(ScopedPropertyAccessExpression<'tree>),
        SubscriptExpression(SubscriptExpression<'tree>),
        VariableName(VariableName<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Anon128718376371871210692312782388999108927<'tree> {
        ///Returns the node if it is of type `dynamic_variable_name` ([`DynamicVariableName`]), otherwise returns `None`
        #[inline]
        pub fn as_dynamic_variable_name(
            self,
        ) -> ::std::option::Option<DynamicVariableName<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DynamicVariableName(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `member_access_expression` ([`MemberAccessExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_member_access_expression(
            self,
        ) -> ::std::option::Option<MemberAccessExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::MemberAccessExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `nullsafe_member_access_expression` ([`NullsafeMemberAccessExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_nullsafe_member_access_expression(
            self,
        ) -> ::std::option::Option<NullsafeMemberAccessExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::NullsafeMemberAccessExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `scoped_property_access_expression` ([`ScopedPropertyAccessExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_scoped_property_access_expression(
            self,
        ) -> ::std::option::Option<ScopedPropertyAccessExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ScopedPropertyAccessExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `subscript_expression` ([`SubscriptExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_subscript_expression(
            self,
        ) -> ::std::option::Option<SubscriptExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::SubscriptExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `variable_name` ([`VariableName`]), otherwise returns `None`
        #[inline]
        pub fn as_variable_name(self) -> ::std::option::Option<VariableName<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::VariableName(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
    for Anon128718376371871210692312782388999108927<'tree> {
        type WithLifetime<'a> = Anon128718376371871210692312782388999108927<'a>;
        const KIND: &'static str = "{dynamic_variable_name | member_access_expression | nullsafe_member_access_expression | scoped_property_access_expression | subscript_expression | variable_name}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "dynamic_variable_name" => {
                    Ok(unsafe {
                        Self::DynamicVariableName(
                            <DynamicVariableName<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "member_access_expression" => {
                    Ok(unsafe {
                        Self::MemberAccessExpression(
                            <MemberAccessExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "nullsafe_member_access_expression" => {
                    Ok(unsafe {
                        Self::NullsafeMemberAccessExpression(
                            <NullsafeMemberAccessExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "scoped_property_access_expression" => {
                    Ok(unsafe {
                        Self::ScopedPropertyAccessExpression(
                            <ScopedPropertyAccessExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "subscript_expression" => {
                    Ok(unsafe {
                        Self::SubscriptExpression(
                            <SubscriptExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "variable_name" => {
                    Ok(unsafe {
                        Self::VariableName(
                            <VariableName<
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
                Self::DynamicVariableName(x) => ::type_sitter::Node::raw(x),
                Self::MemberAccessExpression(x) => ::type_sitter::Node::raw(x),
                Self::NullsafeMemberAccessExpression(x) => ::type_sitter::Node::raw(x),
                Self::ScopedPropertyAccessExpression(x) => ::type_sitter::Node::raw(x),
                Self::SubscriptExpression(x) => ::type_sitter::Node::raw(x),
                Self::VariableName(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::DynamicVariableName(x) => ::type_sitter::Node::raw_mut(x),
                Self::MemberAccessExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::NullsafeMemberAccessExpression(x) => {
                    ::type_sitter::Node::raw_mut(x)
                }
                Self::ScopedPropertyAccessExpression(x) => {
                    ::type_sitter::Node::raw_mut(x)
                }
                Self::SubscriptExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::VariableName(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::DynamicVariableName(x) => x.into_raw(),
                Self::MemberAccessExpression(x) => x.into_raw(),
                Self::NullsafeMemberAccessExpression(x) => x.into_raw(),
                Self::ScopedPropertyAccessExpression(x) => x.into_raw(),
                Self::SubscriptExpression(x) => x.into_raw(),
                Self::VariableName(x) => x.into_raw(),
            }
        }
    }
    /**One of `{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | expression | keyword | livewire | loop | php_statement | props | script_element | style_element | switch | text}`:
- [`Comment`]
- [`Conditional`]
- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Element`]
- [`Entity`]
- [`ErroneousEndTag`]
- [`Expression`]
- [`Keyword`]
- [`Livewire`]
- [`Loop`]
- [`PhpStatement`]
- [`Props`]
- [`ScriptElement`]
- [`StyleElement`]
- [`Switch`]
- [`Text`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Anon145495983449542552864963724180875256821<'tree> {
        Comment(Comment<'tree>),
        Conditional(Conditional<'tree>),
        Directive(Directive<'tree>),
        DirectiveEnd(DirectiveEnd<'tree>),
        DirectiveStart(DirectiveStart<'tree>),
        Element(Element<'tree>),
        Entity(Entity<'tree>),
        ErroneousEndTag(ErroneousEndTag<'tree>),
        Expression(Expression<'tree>),
        Keyword(Keyword<'tree>),
        Livewire(Livewire<'tree>),
        Loop(Loop<'tree>),
        PhpStatement(PhpStatement<'tree>),
        Props(Props<'tree>),
        ScriptElement(ScriptElement<'tree>),
        StyleElement(StyleElement<'tree>),
        Switch(Switch<'tree>),
        Text(Text<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Anon145495983449542552864963724180875256821<'tree> {
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
        ///Returns the node if it is of type `expression` ([`Expression`]), otherwise returns `None`
        #[inline]
        pub fn as_expression(self) -> ::std::option::Option<Expression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Expression(x) = self {
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
        ///Returns the node if it is of type `props` ([`Props`]), otherwise returns `None`
        #[inline]
        pub fn as_props(self) -> ::std::option::Option<Props<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Props(x) = self {
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
    for Anon145495983449542552864963724180875256821<'tree> {
        type WithLifetime<'a> = Anon145495983449542552864963724180875256821<'a>;
        const KIND: &'static str = "{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | expression | keyword | livewire | loop | php_statement | props | script_element | style_element | switch | text}";
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
                "expression" => {
                    Ok(unsafe {
                        Self::Expression(
                            <Expression<
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
                "php_statement" => {
                    Ok(unsafe {
                        Self::PhpStatement(
                            <PhpStatement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "props" => {
                    Ok(unsafe {
                        Self::Props(
                            <Props<
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
                Self::Expression(x) => ::type_sitter::Node::raw(x),
                Self::Keyword(x) => ::type_sitter::Node::raw(x),
                Self::Livewire(x) => ::type_sitter::Node::raw(x),
                Self::Loop(x) => ::type_sitter::Node::raw(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw(x),
                Self::Props(x) => ::type_sitter::Node::raw(x),
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
                Self::Expression(x) => ::type_sitter::Node::raw_mut(x),
                Self::Keyword(x) => ::type_sitter::Node::raw_mut(x),
                Self::Livewire(x) => ::type_sitter::Node::raw_mut(x),
                Self::Loop(x) => ::type_sitter::Node::raw_mut(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw_mut(x),
                Self::Props(x) => ::type_sitter::Node::raw_mut(x),
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
                Self::Expression(x) => x.into_raw(),
                Self::Keyword(x) => x.into_raw(),
                Self::Livewire(x) => x.into_raw(),
                Self::Loop(x) => x.into_raw(),
                Self::PhpStatement(x) => x.into_raw(),
                Self::Props(x) => x.into_raw(),
                Self::ScriptElement(x) => x.into_raw(),
                Self::StyleElement(x) => x.into_raw(),
                Self::Switch(x) => x.into_raw(),
                Self::Text(x) => x.into_raw(),
            }
        }
    }
    /**One of `{array_creation_expression | member_access_expression | parenthesized_expression | subscript_expression | variable_name}`:
- [`ArrayCreationExpression`]
- [`MemberAccessExpression`]
- [`ParenthesizedExpression`]
- [`SubscriptExpression`]
- [`VariableName`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Anon148842522943207813739644966039431447539<'tree> {
        ArrayCreationExpression(ArrayCreationExpression<'tree>),
        MemberAccessExpression(MemberAccessExpression<'tree>),
        ParenthesizedExpression(ParenthesizedExpression<'tree>),
        SubscriptExpression(SubscriptExpression<'tree>),
        VariableName(VariableName<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Anon148842522943207813739644966039431447539<'tree> {
        ///Returns the node if it is of type `array_creation_expression` ([`ArrayCreationExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_array_creation_expression(
            self,
        ) -> ::std::option::Option<ArrayCreationExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ArrayCreationExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `member_access_expression` ([`MemberAccessExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_member_access_expression(
            self,
        ) -> ::std::option::Option<MemberAccessExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::MemberAccessExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `parenthesized_expression` ([`ParenthesizedExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_parenthesized_expression(
            self,
        ) -> ::std::option::Option<ParenthesizedExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ParenthesizedExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `subscript_expression` ([`SubscriptExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_subscript_expression(
            self,
        ) -> ::std::option::Option<SubscriptExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::SubscriptExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `variable_name` ([`VariableName`]), otherwise returns `None`
        #[inline]
        pub fn as_variable_name(self) -> ::std::option::Option<VariableName<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::VariableName(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
    for Anon148842522943207813739644966039431447539<'tree> {
        type WithLifetime<'a> = Anon148842522943207813739644966039431447539<'a>;
        const KIND: &'static str = "{array_creation_expression | member_access_expression | parenthesized_expression | subscript_expression | variable_name}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "array_creation_expression" => {
                    Ok(unsafe {
                        Self::ArrayCreationExpression(
                            <ArrayCreationExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "member_access_expression" => {
                    Ok(unsafe {
                        Self::MemberAccessExpression(
                            <MemberAccessExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "parenthesized_expression" => {
                    Ok(unsafe {
                        Self::ParenthesizedExpression(
                            <ParenthesizedExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "subscript_expression" => {
                    Ok(unsafe {
                        Self::SubscriptExpression(
                            <SubscriptExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "variable_name" => {
                    Ok(unsafe {
                        Self::VariableName(
                            <VariableName<
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
                Self::ArrayCreationExpression(x) => ::type_sitter::Node::raw(x),
                Self::MemberAccessExpression(x) => ::type_sitter::Node::raw(x),
                Self::ParenthesizedExpression(x) => ::type_sitter::Node::raw(x),
                Self::SubscriptExpression(x) => ::type_sitter::Node::raw(x),
                Self::VariableName(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::ArrayCreationExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::MemberAccessExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::ParenthesizedExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::SubscriptExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::VariableName(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::ArrayCreationExpression(x) => x.into_raw(),
                Self::MemberAccessExpression(x) => x.into_raw(),
                Self::ParenthesizedExpression(x) => x.into_raw(),
                Self::SubscriptExpression(x) => x.into_raw(),
                Self::VariableName(x) => x.into_raw(),
            }
        }
    }
    /**One of `{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | expression | keyword | livewire | loop | parameter | php_statement | props | script_element | stack | style_element | switch | text | verbatim}`:
- [`Comment`]
- [`Conditional`]
- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Element`]
- [`Entity`]
- [`ErroneousEndTag`]
- [`Expression`]
- [`Keyword`]
- [`Livewire`]
- [`Loop`]
- [`Parameter`]
- [`PhpStatement`]
- [`Props`]
- [`ScriptElement`]
- [`Stack`]
- [`StyleElement`]
- [`Switch`]
- [`Text`]
- [`Verbatim`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Anon152400393147957412014589918373492536309<'tree> {
        Comment(Comment<'tree>),
        Conditional(Conditional<'tree>),
        Directive(Directive<'tree>),
        DirectiveEnd(DirectiveEnd<'tree>),
        DirectiveStart(DirectiveStart<'tree>),
        Element(Element<'tree>),
        Entity(Entity<'tree>),
        ErroneousEndTag(ErroneousEndTag<'tree>),
        Expression(Expression<'tree>),
        Keyword(Keyword<'tree>),
        Livewire(Livewire<'tree>),
        Loop(Loop<'tree>),
        Parameter(Parameter<'tree>),
        PhpStatement(PhpStatement<'tree>),
        Props(Props<'tree>),
        ScriptElement(ScriptElement<'tree>),
        Stack(Stack<'tree>),
        StyleElement(StyleElement<'tree>),
        Switch(Switch<'tree>),
        Text(Text<'tree>),
        Verbatim(Verbatim<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Anon152400393147957412014589918373492536309<'tree> {
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
        ///Returns the node if it is of type `expression` ([`Expression`]), otherwise returns `None`
        #[inline]
        pub fn as_expression(self) -> ::std::option::Option<Expression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Expression(x) = self {
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
        ///Returns the node if it is of type `props` ([`Props`]), otherwise returns `None`
        #[inline]
        pub fn as_props(self) -> ::std::option::Option<Props<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Props(x) = self {
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
    for Anon152400393147957412014589918373492536309<'tree> {
        type WithLifetime<'a> = Anon152400393147957412014589918373492536309<'a>;
        const KIND: &'static str = "{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | expression | keyword | livewire | loop | parameter | php_statement | props | script_element | stack | style_element | switch | text | verbatim}";
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
                "expression" => {
                    Ok(unsafe {
                        Self::Expression(
                            <Expression<
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
                "props" => {
                    Ok(unsafe {
                        Self::Props(
                            <Props<
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
                Self::Expression(x) => ::type_sitter::Node::raw(x),
                Self::Keyword(x) => ::type_sitter::Node::raw(x),
                Self::Livewire(x) => ::type_sitter::Node::raw(x),
                Self::Loop(x) => ::type_sitter::Node::raw(x),
                Self::Parameter(x) => ::type_sitter::Node::raw(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw(x),
                Self::Props(x) => ::type_sitter::Node::raw(x),
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
                Self::Expression(x) => ::type_sitter::Node::raw_mut(x),
                Self::Keyword(x) => ::type_sitter::Node::raw_mut(x),
                Self::Livewire(x) => ::type_sitter::Node::raw_mut(x),
                Self::Loop(x) => ::type_sitter::Node::raw_mut(x),
                Self::Parameter(x) => ::type_sitter::Node::raw_mut(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw_mut(x),
                Self::Props(x) => ::type_sitter::Node::raw_mut(x),
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
                Self::Expression(x) => x.into_raw(),
                Self::Keyword(x) => x.into_raw(),
                Self::Livewire(x) => x.into_raw(),
                Self::Loop(x) => x.into_raw(),
                Self::Parameter(x) => x.into_raw(),
                Self::PhpStatement(x) => x.into_raw(),
                Self::Props(x) => x.into_raw(),
                Self::ScriptElement(x) => x.into_raw(),
                Self::Stack(x) => x.into_raw(),
                Self::StyleElement(x) => x.into_raw(),
                Self::Switch(x) => x.into_raw(),
                Self::Text(x) => x.into_raw(),
                Self::Verbatim(x) => x.into_raw(),
            }
        }
    }
    /**One of `{!= | !== | % | & | && | * | ** | + | - | . | / | < | << | <= | <=> | <> | == | === | > | >= | >> | ?? | ^ | and | instanceof | or | xor | | | |> | ||}`:
- [`symbols::NotEq`]
- [`symbols::NotEqEq`]
- [`symbols::Mod`]
- [`symbols::And`]
- [`symbols::AndAnd`]
- [`symbols::Mul`]
- [`symbols::MulMul`]
- [`symbols::Add`]
- [`symbols::Sub`]
- [`symbols::Dot`]
- [`symbols::Div`]
- [`symbols::Lt`]
- [`symbols::LtLt`]
- [`symbols::LtEq`]
- [`symbols::LtEqGt`]
- [`symbols::LtGt`]
- [`symbols::EqEq`]
- [`symbols::EqEqEq`]
- [`symbols::Gt`]
- [`symbols::GtEq`]
- [`symbols::GtGt`]
- [`symbols::QuestionQuestion`]
- [`symbols::BitXor`]
- [`unnamed::And`]
- [`unnamed::Instanceof`]
- [`unnamed::Or`]
- [`unnamed::Xor`]
- [`symbols::Or`]
- [`symbols::OrGt`]
- [`symbols::OrOr`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Anon156865598636788927821460262849502993268<'tree> {
        NotEq(symbols::NotEq<'tree>),
        NotEqEq(symbols::NotEqEq<'tree>),
        Mod(symbols::Mod<'tree>),
        And(symbols::And<'tree>),
        AndAnd(symbols::AndAnd<'tree>),
        Mul(symbols::Mul<'tree>),
        MulMul(symbols::MulMul<'tree>),
        Add(symbols::Add<'tree>),
        Sub(symbols::Sub<'tree>),
        Dot(symbols::Dot<'tree>),
        Div(symbols::Div<'tree>),
        Lt(symbols::Lt<'tree>),
        LtLt(symbols::LtLt<'tree>),
        LtEq(symbols::LtEq<'tree>),
        LtEqGt(symbols::LtEqGt<'tree>),
        LtGt(symbols::LtGt<'tree>),
        EqEq(symbols::EqEq<'tree>),
        EqEqEq(symbols::EqEqEq<'tree>),
        Gt(symbols::Gt<'tree>),
        GtEq(symbols::GtEq<'tree>),
        GtGt(symbols::GtGt<'tree>),
        QuestionQuestion(symbols::QuestionQuestion<'tree>),
        BitXor(symbols::BitXor<'tree>),
        And_(unnamed::And<'tree>),
        Instanceof(unnamed::Instanceof<'tree>),
        Or(unnamed::Or<'tree>),
        Xor(unnamed::Xor<'tree>),
        Or_(symbols::Or<'tree>),
        OrGt(symbols::OrGt<'tree>),
        OrOr(symbols::OrOr<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Anon156865598636788927821460262849502993268<'tree> {
        ///Returns the node if it is of type `!=` ([`symbols::NotEq`]), otherwise returns `None`
        #[inline]
        pub fn as_not_eq(self) -> ::std::option::Option<symbols::NotEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::NotEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `!==` ([`symbols::NotEqEq`]), otherwise returns `None`
        #[inline]
        pub fn as_not_eq_eq(self) -> ::std::option::Option<symbols::NotEqEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::NotEqEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `%` ([`symbols::Mod`]), otherwise returns `None`
        #[inline]
        pub fn as_mod(self) -> ::std::option::Option<symbols::Mod<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Mod(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `&` ([`symbols::And`]), otherwise returns `None`
        #[inline]
        pub fn as_and(self) -> ::std::option::Option<symbols::And<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::And(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `&&` ([`symbols::AndAnd`]), otherwise returns `None`
        #[inline]
        pub fn as_and_and(self) -> ::std::option::Option<symbols::AndAnd<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::AndAnd(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `*` ([`symbols::Mul`]), otherwise returns `None`
        #[inline]
        pub fn as_mul(self) -> ::std::option::Option<symbols::Mul<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Mul(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `**` ([`symbols::MulMul`]), otherwise returns `None`
        #[inline]
        pub fn as_mul_mul(self) -> ::std::option::Option<symbols::MulMul<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::MulMul(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `+` ([`symbols::Add`]), otherwise returns `None`
        #[inline]
        pub fn as_add(self) -> ::std::option::Option<symbols::Add<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Add(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `-` ([`symbols::Sub`]), otherwise returns `None`
        #[inline]
        pub fn as_sub(self) -> ::std::option::Option<symbols::Sub<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Sub(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `.` ([`symbols::Dot`]), otherwise returns `None`
        #[inline]
        pub fn as_dot(self) -> ::std::option::Option<symbols::Dot<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Dot(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `/` ([`symbols::Div`]), otherwise returns `None`
        #[inline]
        pub fn as_div(self) -> ::std::option::Option<symbols::Div<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Div(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `<` ([`symbols::Lt`]), otherwise returns `None`
        #[inline]
        pub fn as_lt(self) -> ::std::option::Option<symbols::Lt<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Lt(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `<<` ([`symbols::LtLt`]), otherwise returns `None`
        #[inline]
        pub fn as_lt_lt(self) -> ::std::option::Option<symbols::LtLt<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::LtLt(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `<=` ([`symbols::LtEq`]), otherwise returns `None`
        #[inline]
        pub fn as_lt_eq(self) -> ::std::option::Option<symbols::LtEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::LtEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `<=>` ([`symbols::LtEqGt`]), otherwise returns `None`
        #[inline]
        pub fn as_lt_eq_gt(self) -> ::std::option::Option<symbols::LtEqGt<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::LtEqGt(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `<>` ([`symbols::LtGt`]), otherwise returns `None`
        #[inline]
        pub fn as_lt_gt(self) -> ::std::option::Option<symbols::LtGt<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::LtGt(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `==` ([`symbols::EqEq`]), otherwise returns `None`
        #[inline]
        pub fn as_eq_eq(self) -> ::std::option::Option<symbols::EqEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::EqEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `===` ([`symbols::EqEqEq`]), otherwise returns `None`
        #[inline]
        pub fn as_eq_eq_eq(self) -> ::std::option::Option<symbols::EqEqEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::EqEqEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `>` ([`symbols::Gt`]), otherwise returns `None`
        #[inline]
        pub fn as_gt(self) -> ::std::option::Option<symbols::Gt<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Gt(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `>=` ([`symbols::GtEq`]), otherwise returns `None`
        #[inline]
        pub fn as_gt_eq(self) -> ::std::option::Option<symbols::GtEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::GtEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `>>` ([`symbols::GtGt`]), otherwise returns `None`
        #[inline]
        pub fn as_gt_gt(self) -> ::std::option::Option<symbols::GtGt<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::GtGt(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `??` ([`symbols::QuestionQuestion`]), otherwise returns `None`
        #[inline]
        pub fn as_question_question(
            self,
        ) -> ::std::option::Option<symbols::QuestionQuestion<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::QuestionQuestion(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `^` ([`symbols::BitXor`]), otherwise returns `None`
        #[inline]
        pub fn as_bit_xor(self) -> ::std::option::Option<symbols::BitXor<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::BitXor(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `and` ([`unnamed::And`]), otherwise returns `None`
        #[inline]
        pub fn as_and_(self) -> ::std::option::Option<unnamed::And<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::And_(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `instanceof` ([`unnamed::Instanceof`]), otherwise returns `None`
        #[inline]
        pub fn as_instanceof(self) -> ::std::option::Option<unnamed::Instanceof<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Instanceof(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `or` ([`unnamed::Or`]), otherwise returns `None`
        #[inline]
        pub fn as_or(self) -> ::std::option::Option<unnamed::Or<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Or(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `xor` ([`unnamed::Xor`]), otherwise returns `None`
        #[inline]
        pub fn as_xor(self) -> ::std::option::Option<unnamed::Xor<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Xor(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `|` ([`symbols::Or`]), otherwise returns `None`
        #[inline]
        pub fn as_or_(self) -> ::std::option::Option<symbols::Or<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Or_(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `|>` ([`symbols::OrGt`]), otherwise returns `None`
        #[inline]
        pub fn as_or_gt(self) -> ::std::option::Option<symbols::OrGt<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::OrGt(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `||` ([`symbols::OrOr`]), otherwise returns `None`
        #[inline]
        pub fn as_or_or(self) -> ::std::option::Option<symbols::OrOr<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::OrOr(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
    for Anon156865598636788927821460262849502993268<'tree> {
        type WithLifetime<'a> = Anon156865598636788927821460262849502993268<'a>;
        const KIND: &'static str = "{!= | !== | % | & | && | * | ** | + | - | . | / | < | << | <= | <=> | <> | == | === | > | >= | >> | ?? | ^ | and | instanceof | or | xor | | | |> | ||}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "!=" => {
                    Ok(unsafe {
                        Self::NotEq(
                            <symbols::NotEq<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "!==" => {
                    Ok(unsafe {
                        Self::NotEqEq(
                            <symbols::NotEqEq<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "%" => {
                    Ok(unsafe {
                        Self::Mod(
                            <symbols::Mod<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "&" => {
                    Ok(unsafe {
                        Self::And(
                            <symbols::And<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "&&" => {
                    Ok(unsafe {
                        Self::AndAnd(
                            <symbols::AndAnd<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "*" => {
                    Ok(unsafe {
                        Self::Mul(
                            <symbols::Mul<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "**" => {
                    Ok(unsafe {
                        Self::MulMul(
                            <symbols::MulMul<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "+" => {
                    Ok(unsafe {
                        Self::Add(
                            <symbols::Add<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "-" => {
                    Ok(unsafe {
                        Self::Sub(
                            <symbols::Sub<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "." => {
                    Ok(unsafe {
                        Self::Dot(
                            <symbols::Dot<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "/" => {
                    Ok(unsafe {
                        Self::Div(
                            <symbols::Div<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "<" => {
                    Ok(unsafe {
                        Self::Lt(
                            <symbols::Lt<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "<<" => {
                    Ok(unsafe {
                        Self::LtLt(
                            <symbols::LtLt<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "<=" => {
                    Ok(unsafe {
                        Self::LtEq(
                            <symbols::LtEq<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "<=>" => {
                    Ok(unsafe {
                        Self::LtEqGt(
                            <symbols::LtEqGt<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "<>" => {
                    Ok(unsafe {
                        Self::LtGt(
                            <symbols::LtGt<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "==" => {
                    Ok(unsafe {
                        Self::EqEq(
                            <symbols::EqEq<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "===" => {
                    Ok(unsafe {
                        Self::EqEqEq(
                            <symbols::EqEqEq<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                ">" => {
                    Ok(unsafe {
                        Self::Gt(
                            <symbols::Gt<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                ">=" => {
                    Ok(unsafe {
                        Self::GtEq(
                            <symbols::GtEq<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                ">>" => {
                    Ok(unsafe {
                        Self::GtGt(
                            <symbols::GtGt<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "??" => {
                    Ok(unsafe {
                        Self::QuestionQuestion(
                            <symbols::QuestionQuestion<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "^" => {
                    Ok(unsafe {
                        Self::BitXor(
                            <symbols::BitXor<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "and" => {
                    Ok(unsafe {
                        Self::And_(
                            <unnamed::And<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "instanceof" => {
                    Ok(unsafe {
                        Self::Instanceof(
                            <unnamed::Instanceof<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "or" => {
                    Ok(unsafe {
                        Self::Or(
                            <unnamed::Or<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "xor" => {
                    Ok(unsafe {
                        Self::Xor(
                            <unnamed::Xor<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "|" => {
                    Ok(unsafe {
                        Self::Or_(
                            <symbols::Or<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "|>" => {
                    Ok(unsafe {
                        Self::OrGt(
                            <symbols::OrGt<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "||" => {
                    Ok(unsafe {
                        Self::OrOr(
                            <symbols::OrOr<
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
                Self::NotEq(x) => ::type_sitter::Node::raw(x),
                Self::NotEqEq(x) => ::type_sitter::Node::raw(x),
                Self::Mod(x) => ::type_sitter::Node::raw(x),
                Self::And(x) => ::type_sitter::Node::raw(x),
                Self::AndAnd(x) => ::type_sitter::Node::raw(x),
                Self::Mul(x) => ::type_sitter::Node::raw(x),
                Self::MulMul(x) => ::type_sitter::Node::raw(x),
                Self::Add(x) => ::type_sitter::Node::raw(x),
                Self::Sub(x) => ::type_sitter::Node::raw(x),
                Self::Dot(x) => ::type_sitter::Node::raw(x),
                Self::Div(x) => ::type_sitter::Node::raw(x),
                Self::Lt(x) => ::type_sitter::Node::raw(x),
                Self::LtLt(x) => ::type_sitter::Node::raw(x),
                Self::LtEq(x) => ::type_sitter::Node::raw(x),
                Self::LtEqGt(x) => ::type_sitter::Node::raw(x),
                Self::LtGt(x) => ::type_sitter::Node::raw(x),
                Self::EqEq(x) => ::type_sitter::Node::raw(x),
                Self::EqEqEq(x) => ::type_sitter::Node::raw(x),
                Self::Gt(x) => ::type_sitter::Node::raw(x),
                Self::GtEq(x) => ::type_sitter::Node::raw(x),
                Self::GtGt(x) => ::type_sitter::Node::raw(x),
                Self::QuestionQuestion(x) => ::type_sitter::Node::raw(x),
                Self::BitXor(x) => ::type_sitter::Node::raw(x),
                Self::And_(x) => ::type_sitter::Node::raw(x),
                Self::Instanceof(x) => ::type_sitter::Node::raw(x),
                Self::Or(x) => ::type_sitter::Node::raw(x),
                Self::Xor(x) => ::type_sitter::Node::raw(x),
                Self::Or_(x) => ::type_sitter::Node::raw(x),
                Self::OrGt(x) => ::type_sitter::Node::raw(x),
                Self::OrOr(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::NotEq(x) => ::type_sitter::Node::raw_mut(x),
                Self::NotEqEq(x) => ::type_sitter::Node::raw_mut(x),
                Self::Mod(x) => ::type_sitter::Node::raw_mut(x),
                Self::And(x) => ::type_sitter::Node::raw_mut(x),
                Self::AndAnd(x) => ::type_sitter::Node::raw_mut(x),
                Self::Mul(x) => ::type_sitter::Node::raw_mut(x),
                Self::MulMul(x) => ::type_sitter::Node::raw_mut(x),
                Self::Add(x) => ::type_sitter::Node::raw_mut(x),
                Self::Sub(x) => ::type_sitter::Node::raw_mut(x),
                Self::Dot(x) => ::type_sitter::Node::raw_mut(x),
                Self::Div(x) => ::type_sitter::Node::raw_mut(x),
                Self::Lt(x) => ::type_sitter::Node::raw_mut(x),
                Self::LtLt(x) => ::type_sitter::Node::raw_mut(x),
                Self::LtEq(x) => ::type_sitter::Node::raw_mut(x),
                Self::LtEqGt(x) => ::type_sitter::Node::raw_mut(x),
                Self::LtGt(x) => ::type_sitter::Node::raw_mut(x),
                Self::EqEq(x) => ::type_sitter::Node::raw_mut(x),
                Self::EqEqEq(x) => ::type_sitter::Node::raw_mut(x),
                Self::Gt(x) => ::type_sitter::Node::raw_mut(x),
                Self::GtEq(x) => ::type_sitter::Node::raw_mut(x),
                Self::GtGt(x) => ::type_sitter::Node::raw_mut(x),
                Self::QuestionQuestion(x) => ::type_sitter::Node::raw_mut(x),
                Self::BitXor(x) => ::type_sitter::Node::raw_mut(x),
                Self::And_(x) => ::type_sitter::Node::raw_mut(x),
                Self::Instanceof(x) => ::type_sitter::Node::raw_mut(x),
                Self::Or(x) => ::type_sitter::Node::raw_mut(x),
                Self::Xor(x) => ::type_sitter::Node::raw_mut(x),
                Self::Or_(x) => ::type_sitter::Node::raw_mut(x),
                Self::OrGt(x) => ::type_sitter::Node::raw_mut(x),
                Self::OrOr(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::NotEq(x) => x.into_raw(),
                Self::NotEqEq(x) => x.into_raw(),
                Self::Mod(x) => x.into_raw(),
                Self::And(x) => x.into_raw(),
                Self::AndAnd(x) => x.into_raw(),
                Self::Mul(x) => x.into_raw(),
                Self::MulMul(x) => x.into_raw(),
                Self::Add(x) => x.into_raw(),
                Self::Sub(x) => x.into_raw(),
                Self::Dot(x) => x.into_raw(),
                Self::Div(x) => x.into_raw(),
                Self::Lt(x) => x.into_raw(),
                Self::LtLt(x) => x.into_raw(),
                Self::LtEq(x) => x.into_raw(),
                Self::LtEqGt(x) => x.into_raw(),
                Self::LtGt(x) => x.into_raw(),
                Self::EqEq(x) => x.into_raw(),
                Self::EqEqEq(x) => x.into_raw(),
                Self::Gt(x) => x.into_raw(),
                Self::GtEq(x) => x.into_raw(),
                Self::GtGt(x) => x.into_raw(),
                Self::QuestionQuestion(x) => x.into_raw(),
                Self::BitXor(x) => x.into_raw(),
                Self::And_(x) => x.into_raw(),
                Self::Instanceof(x) => x.into_raw(),
                Self::Or(x) => x.into_raw(),
                Self::Xor(x) => x.into_raw(),
                Self::Or_(x) => x.into_raw(),
                Self::OrGt(x) => x.into_raw(),
                Self::OrOr(x) => x.into_raw(),
            }
        }
    }
    /**One of `{dynamic_variable_name | member_access_expression | name | nullsafe_member_access_expression | qualified_name | scoped_property_access_expression | subscript_expression | variable_name}`:
- [`DynamicVariableName`]
- [`MemberAccessExpression`]
- [`Name`]
- [`NullsafeMemberAccessExpression`]
- [`QualifiedName`]
- [`ScopedPropertyAccessExpression`]
- [`SubscriptExpression`]
- [`VariableName`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Anon182904659458560044835768290653647831886<'tree> {
        DynamicVariableName(DynamicVariableName<'tree>),
        MemberAccessExpression(MemberAccessExpression<'tree>),
        Name(Name<'tree>),
        NullsafeMemberAccessExpression(NullsafeMemberAccessExpression<'tree>),
        QualifiedName(QualifiedName<'tree>),
        ScopedPropertyAccessExpression(ScopedPropertyAccessExpression<'tree>),
        SubscriptExpression(SubscriptExpression<'tree>),
        VariableName(VariableName<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Anon182904659458560044835768290653647831886<'tree> {
        ///Returns the node if it is of type `dynamic_variable_name` ([`DynamicVariableName`]), otherwise returns `None`
        #[inline]
        pub fn as_dynamic_variable_name(
            self,
        ) -> ::std::option::Option<DynamicVariableName<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DynamicVariableName(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `member_access_expression` ([`MemberAccessExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_member_access_expression(
            self,
        ) -> ::std::option::Option<MemberAccessExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::MemberAccessExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `name` ([`Name`]), otherwise returns `None`
        #[inline]
        pub fn as_name(self) -> ::std::option::Option<Name<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Name(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `nullsafe_member_access_expression` ([`NullsafeMemberAccessExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_nullsafe_member_access_expression(
            self,
        ) -> ::std::option::Option<NullsafeMemberAccessExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::NullsafeMemberAccessExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `qualified_name` ([`QualifiedName`]), otherwise returns `None`
        #[inline]
        pub fn as_qualified_name(self) -> ::std::option::Option<QualifiedName<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::QualifiedName(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `scoped_property_access_expression` ([`ScopedPropertyAccessExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_scoped_property_access_expression(
            self,
        ) -> ::std::option::Option<ScopedPropertyAccessExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ScopedPropertyAccessExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `subscript_expression` ([`SubscriptExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_subscript_expression(
            self,
        ) -> ::std::option::Option<SubscriptExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::SubscriptExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `variable_name` ([`VariableName`]), otherwise returns `None`
        #[inline]
        pub fn as_variable_name(self) -> ::std::option::Option<VariableName<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::VariableName(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
    for Anon182904659458560044835768290653647831886<'tree> {
        type WithLifetime<'a> = Anon182904659458560044835768290653647831886<'a>;
        const KIND: &'static str = "{dynamic_variable_name | member_access_expression | name | nullsafe_member_access_expression | qualified_name | scoped_property_access_expression | subscript_expression | variable_name}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "dynamic_variable_name" => {
                    Ok(unsafe {
                        Self::DynamicVariableName(
                            <DynamicVariableName<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "member_access_expression" => {
                    Ok(unsafe {
                        Self::MemberAccessExpression(
                            <MemberAccessExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "name" => {
                    Ok(unsafe {
                        Self::Name(
                            <Name<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "nullsafe_member_access_expression" => {
                    Ok(unsafe {
                        Self::NullsafeMemberAccessExpression(
                            <NullsafeMemberAccessExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "qualified_name" => {
                    Ok(unsafe {
                        Self::QualifiedName(
                            <QualifiedName<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "scoped_property_access_expression" => {
                    Ok(unsafe {
                        Self::ScopedPropertyAccessExpression(
                            <ScopedPropertyAccessExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "subscript_expression" => {
                    Ok(unsafe {
                        Self::SubscriptExpression(
                            <SubscriptExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "variable_name" => {
                    Ok(unsafe {
                        Self::VariableName(
                            <VariableName<
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
                Self::DynamicVariableName(x) => ::type_sitter::Node::raw(x),
                Self::MemberAccessExpression(x) => ::type_sitter::Node::raw(x),
                Self::Name(x) => ::type_sitter::Node::raw(x),
                Self::NullsafeMemberAccessExpression(x) => ::type_sitter::Node::raw(x),
                Self::QualifiedName(x) => ::type_sitter::Node::raw(x),
                Self::ScopedPropertyAccessExpression(x) => ::type_sitter::Node::raw(x),
                Self::SubscriptExpression(x) => ::type_sitter::Node::raw(x),
                Self::VariableName(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::DynamicVariableName(x) => ::type_sitter::Node::raw_mut(x),
                Self::MemberAccessExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::Name(x) => ::type_sitter::Node::raw_mut(x),
                Self::NullsafeMemberAccessExpression(x) => {
                    ::type_sitter::Node::raw_mut(x)
                }
                Self::QualifiedName(x) => ::type_sitter::Node::raw_mut(x),
                Self::ScopedPropertyAccessExpression(x) => {
                    ::type_sitter::Node::raw_mut(x)
                }
                Self::SubscriptExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::VariableName(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::DynamicVariableName(x) => x.into_raw(),
                Self::MemberAccessExpression(x) => x.into_raw(),
                Self::Name(x) => x.into_raw(),
                Self::NullsafeMemberAccessExpression(x) => x.into_raw(),
                Self::QualifiedName(x) => x.into_raw(),
                Self::ScopedPropertyAccessExpression(x) => x.into_raw(),
                Self::SubscriptExpression(x) => x.into_raw(),
                Self::VariableName(x) => x.into_raw(),
            }
        }
    }
    /**One of `{comment | conditional | directive | doctype | element | end_tag | entity | envoy | erroneous_end_tag | expression | fragment | keyword | livewire | loop | once | php_statement | props | script_element | section | self_closing_tag | stack | start_tag | style_element | switch | text | verbatim}`:
- [`Comment`]
- [`Conditional`]
- [`Directive`]
- [`Doctype`]
- [`Element`]
- [`EndTag`]
- [`Entity`]
- [`Envoy`]
- [`ErroneousEndTag`]
- [`Expression`]
- [`Fragment`]
- [`Keyword`]
- [`Livewire`]
- [`Loop`]
- [`Once`]
- [`PhpStatement`]
- [`Props`]
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
    pub enum Anon227303402907815333350325131840109493873<'tree> {
        Comment(Comment<'tree>),
        Conditional(Conditional<'tree>),
        Directive(Directive<'tree>),
        Doctype(Doctype<'tree>),
        Element(Element<'tree>),
        EndTag(EndTag<'tree>),
        Entity(Entity<'tree>),
        Envoy(Envoy<'tree>),
        ErroneousEndTag(ErroneousEndTag<'tree>),
        Expression(Expression<'tree>),
        Fragment(Fragment<'tree>),
        Keyword(Keyword<'tree>),
        Livewire(Livewire<'tree>),
        Loop(Loop<'tree>),
        Once(Once<'tree>),
        PhpStatement(PhpStatement<'tree>),
        Props(Props<'tree>),
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
    impl<'tree> Anon227303402907815333350325131840109493873<'tree> {
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
        ///Returns the node if it is of type `expression` ([`Expression`]), otherwise returns `None`
        #[inline]
        pub fn as_expression(self) -> ::std::option::Option<Expression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Expression(x) = self {
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
        ///Returns the node if it is of type `props` ([`Props`]), otherwise returns `None`
        #[inline]
        pub fn as_props(self) -> ::std::option::Option<Props<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Props(x) = self {
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
    for Anon227303402907815333350325131840109493873<'tree> {
        type WithLifetime<'a> = Anon227303402907815333350325131840109493873<'a>;
        const KIND: &'static str = "{comment | conditional | directive | doctype | element | end_tag | entity | envoy | erroneous_end_tag | expression | fragment | keyword | livewire | loop | once | php_statement | props | script_element | section | self_closing_tag | stack | start_tag | style_element | switch | text | verbatim}";
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
                "expression" => {
                    Ok(unsafe {
                        Self::Expression(
                            <Expression<
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
                "php_statement" => {
                    Ok(unsafe {
                        Self::PhpStatement(
                            <PhpStatement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "props" => {
                    Ok(unsafe {
                        Self::Props(
                            <Props<
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
                Self::Expression(x) => ::type_sitter::Node::raw(x),
                Self::Fragment(x) => ::type_sitter::Node::raw(x),
                Self::Keyword(x) => ::type_sitter::Node::raw(x),
                Self::Livewire(x) => ::type_sitter::Node::raw(x),
                Self::Loop(x) => ::type_sitter::Node::raw(x),
                Self::Once(x) => ::type_sitter::Node::raw(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw(x),
                Self::Props(x) => ::type_sitter::Node::raw(x),
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
                Self::Expression(x) => ::type_sitter::Node::raw_mut(x),
                Self::Fragment(x) => ::type_sitter::Node::raw_mut(x),
                Self::Keyword(x) => ::type_sitter::Node::raw_mut(x),
                Self::Livewire(x) => ::type_sitter::Node::raw_mut(x),
                Self::Loop(x) => ::type_sitter::Node::raw_mut(x),
                Self::Once(x) => ::type_sitter::Node::raw_mut(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw_mut(x),
                Self::Props(x) => ::type_sitter::Node::raw_mut(x),
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
                Self::Expression(x) => x.into_raw(),
                Self::Fragment(x) => x.into_raw(),
                Self::Keyword(x) => x.into_raw(),
                Self::Livewire(x) => x.into_raw(),
                Self::Loop(x) => x.into_raw(),
                Self::Once(x) => x.into_raw(),
                Self::PhpStatement(x) => x.into_raw(),
                Self::Props(x) => x.into_raw(),
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
    /**One of `{comment | conditional | directive | doctype | element | entity | envoy | erroneous_end_tag | expression | fragment | keyword | livewire | loop | once | php_statement | props | script_element | section | stack | style_element | switch | text | verbatim}`:
- [`Comment`]
- [`Conditional`]
- [`Directive`]
- [`Doctype`]
- [`Element`]
- [`Entity`]
- [`Envoy`]
- [`ErroneousEndTag`]
- [`Expression`]
- [`Fragment`]
- [`Keyword`]
- [`Livewire`]
- [`Loop`]
- [`Once`]
- [`PhpStatement`]
- [`Props`]
- [`ScriptElement`]
- [`Section`]
- [`Stack`]
- [`StyleElement`]
- [`Switch`]
- [`Text`]
- [`Verbatim`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Anon228412328637052745595247458993952546707<'tree> {
        Comment(Comment<'tree>),
        Conditional(Conditional<'tree>),
        Directive(Directive<'tree>),
        Doctype(Doctype<'tree>),
        Element(Element<'tree>),
        Entity(Entity<'tree>),
        Envoy(Envoy<'tree>),
        ErroneousEndTag(ErroneousEndTag<'tree>),
        Expression(Expression<'tree>),
        Fragment(Fragment<'tree>),
        Keyword(Keyword<'tree>),
        Livewire(Livewire<'tree>),
        Loop(Loop<'tree>),
        Once(Once<'tree>),
        PhpStatement(PhpStatement<'tree>),
        Props(Props<'tree>),
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
    impl<'tree> Anon228412328637052745595247458993952546707<'tree> {
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
        ///Returns the node if it is of type `expression` ([`Expression`]), otherwise returns `None`
        #[inline]
        pub fn as_expression(self) -> ::std::option::Option<Expression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Expression(x) = self {
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
        ///Returns the node if it is of type `props` ([`Props`]), otherwise returns `None`
        #[inline]
        pub fn as_props(self) -> ::std::option::Option<Props<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Props(x) = self {
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
    for Anon228412328637052745595247458993952546707<'tree> {
        type WithLifetime<'a> = Anon228412328637052745595247458993952546707<'a>;
        const KIND: &'static str = "{comment | conditional | directive | doctype | element | entity | envoy | erroneous_end_tag | expression | fragment | keyword | livewire | loop | once | php_statement | props | script_element | section | stack | style_element | switch | text | verbatim}";
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
                "expression" => {
                    Ok(unsafe {
                        Self::Expression(
                            <Expression<
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
                "php_statement" => {
                    Ok(unsafe {
                        Self::PhpStatement(
                            <PhpStatement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "props" => {
                    Ok(unsafe {
                        Self::Props(
                            <Props<
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
                Self::Expression(x) => ::type_sitter::Node::raw(x),
                Self::Fragment(x) => ::type_sitter::Node::raw(x),
                Self::Keyword(x) => ::type_sitter::Node::raw(x),
                Self::Livewire(x) => ::type_sitter::Node::raw(x),
                Self::Loop(x) => ::type_sitter::Node::raw(x),
                Self::Once(x) => ::type_sitter::Node::raw(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw(x),
                Self::Props(x) => ::type_sitter::Node::raw(x),
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
                Self::Expression(x) => ::type_sitter::Node::raw_mut(x),
                Self::Fragment(x) => ::type_sitter::Node::raw_mut(x),
                Self::Keyword(x) => ::type_sitter::Node::raw_mut(x),
                Self::Livewire(x) => ::type_sitter::Node::raw_mut(x),
                Self::Loop(x) => ::type_sitter::Node::raw_mut(x),
                Self::Once(x) => ::type_sitter::Node::raw_mut(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw_mut(x),
                Self::Props(x) => ::type_sitter::Node::raw_mut(x),
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
                Self::Expression(x) => x.into_raw(),
                Self::Fragment(x) => x.into_raw(),
                Self::Keyword(x) => x.into_raw(),
                Self::Livewire(x) => x.into_raw(),
                Self::Loop(x) => x.into_raw(),
                Self::Once(x) => x.into_raw(),
                Self::PhpStatement(x) => x.into_raw(),
                Self::Props(x) => x.into_raw(),
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
    /**One of `{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | expression | fragment | keyword | loop | once | php_statement | props | script_element | section | stack | style_element | switch | text | verbatim}`:
- [`Comment`]
- [`Conditional`]
- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Element`]
- [`Entity`]
- [`ErroneousEndTag`]
- [`Expression`]
- [`Fragment`]
- [`Keyword`]
- [`Loop`]
- [`Once`]
- [`PhpStatement`]
- [`Props`]
- [`ScriptElement`]
- [`Section`]
- [`Stack`]
- [`StyleElement`]
- [`Switch`]
- [`Text`]
- [`Verbatim`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Anon240876909159937813961889479930182217457<'tree> {
        Comment(Comment<'tree>),
        Conditional(Conditional<'tree>),
        Directive(Directive<'tree>),
        DirectiveEnd(DirectiveEnd<'tree>),
        DirectiveStart(DirectiveStart<'tree>),
        Element(Element<'tree>),
        Entity(Entity<'tree>),
        ErroneousEndTag(ErroneousEndTag<'tree>),
        Expression(Expression<'tree>),
        Fragment(Fragment<'tree>),
        Keyword(Keyword<'tree>),
        Loop(Loop<'tree>),
        Once(Once<'tree>),
        PhpStatement(PhpStatement<'tree>),
        Props(Props<'tree>),
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
    impl<'tree> Anon240876909159937813961889479930182217457<'tree> {
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
        ///Returns the node if it is of type `expression` ([`Expression`]), otherwise returns `None`
        #[inline]
        pub fn as_expression(self) -> ::std::option::Option<Expression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Expression(x) = self {
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
        ///Returns the node if it is of type `props` ([`Props`]), otherwise returns `None`
        #[inline]
        pub fn as_props(self) -> ::std::option::Option<Props<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Props(x) = self {
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
    for Anon240876909159937813961889479930182217457<'tree> {
        type WithLifetime<'a> = Anon240876909159937813961889479930182217457<'a>;
        const KIND: &'static str = "{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | expression | fragment | keyword | loop | once | php_statement | props | script_element | section | stack | style_element | switch | text | verbatim}";
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
                "expression" => {
                    Ok(unsafe {
                        Self::Expression(
                            <Expression<
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
                "php_statement" => {
                    Ok(unsafe {
                        Self::PhpStatement(
                            <PhpStatement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "props" => {
                    Ok(unsafe {
                        Self::Props(
                            <Props<
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
                Self::Expression(x) => ::type_sitter::Node::raw(x),
                Self::Fragment(x) => ::type_sitter::Node::raw(x),
                Self::Keyword(x) => ::type_sitter::Node::raw(x),
                Self::Loop(x) => ::type_sitter::Node::raw(x),
                Self::Once(x) => ::type_sitter::Node::raw(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw(x),
                Self::Props(x) => ::type_sitter::Node::raw(x),
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
                Self::Expression(x) => ::type_sitter::Node::raw_mut(x),
                Self::Fragment(x) => ::type_sitter::Node::raw_mut(x),
                Self::Keyword(x) => ::type_sitter::Node::raw_mut(x),
                Self::Loop(x) => ::type_sitter::Node::raw_mut(x),
                Self::Once(x) => ::type_sitter::Node::raw_mut(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw_mut(x),
                Self::Props(x) => ::type_sitter::Node::raw_mut(x),
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
                Self::Expression(x) => x.into_raw(),
                Self::Fragment(x) => x.into_raw(),
                Self::Keyword(x) => x.into_raw(),
                Self::Loop(x) => x.into_raw(),
                Self::Once(x) => x.into_raw(),
                Self::PhpStatement(x) => x.into_raw(),
                Self::Props(x) => x.into_raw(),
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
    /**One of `{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | expression | fragment | keyword | livewire | loop | once | php_statement | props | script_element | stack | style_element | switch | text | verbatim}`:
- [`Comment`]
- [`Conditional`]
- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Element`]
- [`Entity`]
- [`ErroneousEndTag`]
- [`Expression`]
- [`Fragment`]
- [`Keyword`]
- [`Livewire`]
- [`Loop`]
- [`Once`]
- [`PhpStatement`]
- [`Props`]
- [`ScriptElement`]
- [`Stack`]
- [`StyleElement`]
- [`Switch`]
- [`Text`]
- [`Verbatim`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Anon245876123772096930279926916143829070986<'tree> {
        Comment(Comment<'tree>),
        Conditional(Conditional<'tree>),
        Directive(Directive<'tree>),
        DirectiveEnd(DirectiveEnd<'tree>),
        DirectiveStart(DirectiveStart<'tree>),
        Element(Element<'tree>),
        Entity(Entity<'tree>),
        ErroneousEndTag(ErroneousEndTag<'tree>),
        Expression(Expression<'tree>),
        Fragment(Fragment<'tree>),
        Keyword(Keyword<'tree>),
        Livewire(Livewire<'tree>),
        Loop(Loop<'tree>),
        Once(Once<'tree>),
        PhpStatement(PhpStatement<'tree>),
        Props(Props<'tree>),
        ScriptElement(ScriptElement<'tree>),
        Stack(Stack<'tree>),
        StyleElement(StyleElement<'tree>),
        Switch(Switch<'tree>),
        Text(Text<'tree>),
        Verbatim(Verbatim<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Anon245876123772096930279926916143829070986<'tree> {
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
        ///Returns the node if it is of type `expression` ([`Expression`]), otherwise returns `None`
        #[inline]
        pub fn as_expression(self) -> ::std::option::Option<Expression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Expression(x) = self {
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
        ///Returns the node if it is of type `props` ([`Props`]), otherwise returns `None`
        #[inline]
        pub fn as_props(self) -> ::std::option::Option<Props<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Props(x) = self {
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
    for Anon245876123772096930279926916143829070986<'tree> {
        type WithLifetime<'a> = Anon245876123772096930279926916143829070986<'a>;
        const KIND: &'static str = "{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | expression | fragment | keyword | livewire | loop | once | php_statement | props | script_element | stack | style_element | switch | text | verbatim}";
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
                "expression" => {
                    Ok(unsafe {
                        Self::Expression(
                            <Expression<
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
                "php_statement" => {
                    Ok(unsafe {
                        Self::PhpStatement(
                            <PhpStatement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "props" => {
                    Ok(unsafe {
                        Self::Props(
                            <Props<
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
                Self::Expression(x) => ::type_sitter::Node::raw(x),
                Self::Fragment(x) => ::type_sitter::Node::raw(x),
                Self::Keyword(x) => ::type_sitter::Node::raw(x),
                Self::Livewire(x) => ::type_sitter::Node::raw(x),
                Self::Loop(x) => ::type_sitter::Node::raw(x),
                Self::Once(x) => ::type_sitter::Node::raw(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw(x),
                Self::Props(x) => ::type_sitter::Node::raw(x),
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
                Self::Expression(x) => ::type_sitter::Node::raw_mut(x),
                Self::Fragment(x) => ::type_sitter::Node::raw_mut(x),
                Self::Keyword(x) => ::type_sitter::Node::raw_mut(x),
                Self::Livewire(x) => ::type_sitter::Node::raw_mut(x),
                Self::Loop(x) => ::type_sitter::Node::raw_mut(x),
                Self::Once(x) => ::type_sitter::Node::raw_mut(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw_mut(x),
                Self::Props(x) => ::type_sitter::Node::raw_mut(x),
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
                Self::Expression(x) => x.into_raw(),
                Self::Fragment(x) => x.into_raw(),
                Self::Keyword(x) => x.into_raw(),
                Self::Livewire(x) => x.into_raw(),
                Self::Loop(x) => x.into_raw(),
                Self::Once(x) => x.into_raw(),
                Self::PhpStatement(x) => x.into_raw(),
                Self::Props(x) => x.into_raw(),
                Self::ScriptElement(x) => x.into_raw(),
                Self::Stack(x) => x.into_raw(),
                Self::StyleElement(x) => x.into_raw(),
                Self::Switch(x) => x.into_raw(),
                Self::Text(x) => x.into_raw(),
                Self::Verbatim(x) => x.into_raw(),
            }
        }
    }
    /**One of `{assignment_expression | binary_expression | cast_expression | conditional_expression | primary_expression | unary_op_expression}`:
- [`AssignmentExpression`]
- [`BinaryExpression`]
- [`CastExpression`]
- [`ConditionalExpression`]
- [`PrimaryExpression`]
- [`UnaryOpExpression`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Anon249583382270925116125355034815407047852<'tree> {
        AssignmentExpression(AssignmentExpression<'tree>),
        BinaryExpression(BinaryExpression<'tree>),
        CastExpression(CastExpression<'tree>),
        ConditionalExpression(ConditionalExpression<'tree>),
        PrimaryExpression(PrimaryExpression<'tree>),
        UnaryOpExpression(UnaryOpExpression<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Anon249583382270925116125355034815407047852<'tree> {
        ///Returns the node if it is of type `assignment_expression` ([`AssignmentExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_assignment_expression(
            self,
        ) -> ::std::option::Option<AssignmentExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::AssignmentExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `binary_expression` ([`BinaryExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_binary_expression(
            self,
        ) -> ::std::option::Option<BinaryExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::BinaryExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `cast_expression` ([`CastExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_cast_expression(self) -> ::std::option::Option<CastExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::CastExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `conditional_expression` ([`ConditionalExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_conditional_expression(
            self,
        ) -> ::std::option::Option<ConditionalExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ConditionalExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `primary_expression` ([`PrimaryExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_primary_expression(
            self,
        ) -> ::std::option::Option<PrimaryExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::PrimaryExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `unary_op_expression` ([`UnaryOpExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_unary_op_expression(
            self,
        ) -> ::std::option::Option<UnaryOpExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::UnaryOpExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
    for Anon249583382270925116125355034815407047852<'tree> {
        type WithLifetime<'a> = Anon249583382270925116125355034815407047852<'a>;
        const KIND: &'static str = "{assignment_expression | binary_expression | cast_expression | conditional_expression | primary_expression | unary_op_expression}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "assignment_expression" => {
                    Ok(unsafe {
                        Self::AssignmentExpression(
                            <AssignmentExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "binary_expression" => {
                    Ok(unsafe {
                        Self::BinaryExpression(
                            <BinaryExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "cast_expression" => {
                    Ok(unsafe {
                        Self::CastExpression(
                            <CastExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "conditional_expression" => {
                    Ok(unsafe {
                        Self::ConditionalExpression(
                            <ConditionalExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "primary_expression" => {
                    Ok(unsafe {
                        Self::PrimaryExpression(
                            <PrimaryExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "unary_op_expression" => {
                    Ok(unsafe {
                        Self::UnaryOpExpression(
                            <UnaryOpExpression<
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
                Self::AssignmentExpression(x) => ::type_sitter::Node::raw(x),
                Self::BinaryExpression(x) => ::type_sitter::Node::raw(x),
                Self::CastExpression(x) => ::type_sitter::Node::raw(x),
                Self::ConditionalExpression(x) => ::type_sitter::Node::raw(x),
                Self::PrimaryExpression(x) => ::type_sitter::Node::raw(x),
                Self::UnaryOpExpression(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::AssignmentExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::BinaryExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::CastExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::ConditionalExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::PrimaryExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::UnaryOpExpression(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::AssignmentExpression(x) => x.into_raw(),
                Self::BinaryExpression(x) => x.into_raw(),
                Self::CastExpression(x) => x.into_raw(),
                Self::ConditionalExpression(x) => x.into_raw(),
                Self::PrimaryExpression(x) => x.into_raw(),
                Self::UnaryOpExpression(x) => x.into_raw(),
            }
        }
    }
    /**One of `{comment | directive | directive_end | directive_start | element | entity | erroneous_end_tag | expression | keyword | php_statement | props | script_element | style_element | switch | text}`:
- [`Comment`]
- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Element`]
- [`Entity`]
- [`ErroneousEndTag`]
- [`Expression`]
- [`Keyword`]
- [`PhpStatement`]
- [`Props`]
- [`ScriptElement`]
- [`StyleElement`]
- [`Switch`]
- [`Text`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Anon263080000079809860076926378912201300104<'tree> {
        Comment(Comment<'tree>),
        Directive(Directive<'tree>),
        DirectiveEnd(DirectiveEnd<'tree>),
        DirectiveStart(DirectiveStart<'tree>),
        Element(Element<'tree>),
        Entity(Entity<'tree>),
        ErroneousEndTag(ErroneousEndTag<'tree>),
        Expression(Expression<'tree>),
        Keyword(Keyword<'tree>),
        PhpStatement(PhpStatement<'tree>),
        Props(Props<'tree>),
        ScriptElement(ScriptElement<'tree>),
        StyleElement(StyleElement<'tree>),
        Switch(Switch<'tree>),
        Text(Text<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Anon263080000079809860076926378912201300104<'tree> {
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
        ///Returns the node if it is of type `expression` ([`Expression`]), otherwise returns `None`
        #[inline]
        pub fn as_expression(self) -> ::std::option::Option<Expression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Expression(x) = self {
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
        ///Returns the node if it is of type `props` ([`Props`]), otherwise returns `None`
        #[inline]
        pub fn as_props(self) -> ::std::option::Option<Props<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Props(x) = self {
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
    for Anon263080000079809860076926378912201300104<'tree> {
        type WithLifetime<'a> = Anon263080000079809860076926378912201300104<'a>;
        const KIND: &'static str = "{comment | directive | directive_end | directive_start | element | entity | erroneous_end_tag | expression | keyword | php_statement | props | script_element | style_element | switch | text}";
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
                "expression" => {
                    Ok(unsafe {
                        Self::Expression(
                            <Expression<
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
                "php_statement" => {
                    Ok(unsafe {
                        Self::PhpStatement(
                            <PhpStatement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "props" => {
                    Ok(unsafe {
                        Self::Props(
                            <Props<
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
                Self::Expression(x) => ::type_sitter::Node::raw(x),
                Self::Keyword(x) => ::type_sitter::Node::raw(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw(x),
                Self::Props(x) => ::type_sitter::Node::raw(x),
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
                Self::Expression(x) => ::type_sitter::Node::raw_mut(x),
                Self::Keyword(x) => ::type_sitter::Node::raw_mut(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw_mut(x),
                Self::Props(x) => ::type_sitter::Node::raw_mut(x),
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
                Self::Expression(x) => x.into_raw(),
                Self::Keyword(x) => x.into_raw(),
                Self::PhpStatement(x) => x.into_raw(),
                Self::Props(x) => x.into_raw(),
                Self::ScriptElement(x) => x.into_raw(),
                Self::StyleElement(x) => x.into_raw(),
                Self::Switch(x) => x.into_raw(),
                Self::Text(x) => x.into_raw(),
            }
        }
    }
    /**One of `{anonymous_function | array_creation_expression | arrow_function | class_constant_access_expression | function_call_expression | literal | member_access_expression | name | parenthesized_expression | qualified_name | relative_name | subscript_expression | update_expression | variable_name}`:
- [`AnonymousFunction`]
- [`ArrayCreationExpression`]
- [`ArrowFunction`]
- [`ClassConstantAccessExpression`]
- [`FunctionCallExpression`]
- [`Literal`]
- [`MemberAccessExpression`]
- [`Name`]
- [`ParenthesizedExpression`]
- [`QualifiedName`]
- [`RelativeName`]
- [`SubscriptExpression`]
- [`UpdateExpression`]
- [`VariableName`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Anon307706018955403244961867585570518755608<'tree> {
        AnonymousFunction(AnonymousFunction<'tree>),
        ArrayCreationExpression(ArrayCreationExpression<'tree>),
        ArrowFunction(ArrowFunction<'tree>),
        ClassConstantAccessExpression(ClassConstantAccessExpression<'tree>),
        FunctionCallExpression(FunctionCallExpression<'tree>),
        Literal(Literal<'tree>),
        MemberAccessExpression(MemberAccessExpression<'tree>),
        Name(Name<'tree>),
        ParenthesizedExpression(ParenthesizedExpression<'tree>),
        QualifiedName(QualifiedName<'tree>),
        RelativeName(RelativeName<'tree>),
        SubscriptExpression(SubscriptExpression<'tree>),
        UpdateExpression(UpdateExpression<'tree>),
        VariableName(VariableName<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Anon307706018955403244961867585570518755608<'tree> {
        ///Returns the node if it is of type `anonymous_function` ([`AnonymousFunction`]), otherwise returns `None`
        #[inline]
        pub fn as_anonymous_function(
            self,
        ) -> ::std::option::Option<AnonymousFunction<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::AnonymousFunction(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `array_creation_expression` ([`ArrayCreationExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_array_creation_expression(
            self,
        ) -> ::std::option::Option<ArrayCreationExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ArrayCreationExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `arrow_function` ([`ArrowFunction`]), otherwise returns `None`
        #[inline]
        pub fn as_arrow_function(self) -> ::std::option::Option<ArrowFunction<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ArrowFunction(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `class_constant_access_expression` ([`ClassConstantAccessExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_class_constant_access_expression(
            self,
        ) -> ::std::option::Option<ClassConstantAccessExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ClassConstantAccessExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `function_call_expression` ([`FunctionCallExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_function_call_expression(
            self,
        ) -> ::std::option::Option<FunctionCallExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::FunctionCallExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `literal` ([`Literal`]), otherwise returns `None`
        #[inline]
        pub fn as_literal(self) -> ::std::option::Option<Literal<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Literal(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `member_access_expression` ([`MemberAccessExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_member_access_expression(
            self,
        ) -> ::std::option::Option<MemberAccessExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::MemberAccessExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `name` ([`Name`]), otherwise returns `None`
        #[inline]
        pub fn as_name(self) -> ::std::option::Option<Name<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Name(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `parenthesized_expression` ([`ParenthesizedExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_parenthesized_expression(
            self,
        ) -> ::std::option::Option<ParenthesizedExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ParenthesizedExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `qualified_name` ([`QualifiedName`]), otherwise returns `None`
        #[inline]
        pub fn as_qualified_name(self) -> ::std::option::Option<QualifiedName<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::QualifiedName(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `relative_name` ([`RelativeName`]), otherwise returns `None`
        #[inline]
        pub fn as_relative_name(self) -> ::std::option::Option<RelativeName<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::RelativeName(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `subscript_expression` ([`SubscriptExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_subscript_expression(
            self,
        ) -> ::std::option::Option<SubscriptExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::SubscriptExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `update_expression` ([`UpdateExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_update_expression(
            self,
        ) -> ::std::option::Option<UpdateExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::UpdateExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `variable_name` ([`VariableName`]), otherwise returns `None`
        #[inline]
        pub fn as_variable_name(self) -> ::std::option::Option<VariableName<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::VariableName(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
    for Anon307706018955403244961867585570518755608<'tree> {
        type WithLifetime<'a> = Anon307706018955403244961867585570518755608<'a>;
        const KIND: &'static str = "{anonymous_function | array_creation_expression | arrow_function | class_constant_access_expression | function_call_expression | literal | member_access_expression | name | parenthesized_expression | qualified_name | relative_name | subscript_expression | update_expression | variable_name}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "anonymous_function" => {
                    Ok(unsafe {
                        Self::AnonymousFunction(
                            <AnonymousFunction<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "array_creation_expression" => {
                    Ok(unsafe {
                        Self::ArrayCreationExpression(
                            <ArrayCreationExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "arrow_function" => {
                    Ok(unsafe {
                        Self::ArrowFunction(
                            <ArrowFunction<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "class_constant_access_expression" => {
                    Ok(unsafe {
                        Self::ClassConstantAccessExpression(
                            <ClassConstantAccessExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "function_call_expression" => {
                    Ok(unsafe {
                        Self::FunctionCallExpression(
                            <FunctionCallExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "literal" => {
                    Ok(unsafe {
                        Self::Literal(
                            <Literal<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "member_access_expression" => {
                    Ok(unsafe {
                        Self::MemberAccessExpression(
                            <MemberAccessExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "name" => {
                    Ok(unsafe {
                        Self::Name(
                            <Name<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "parenthesized_expression" => {
                    Ok(unsafe {
                        Self::ParenthesizedExpression(
                            <ParenthesizedExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "qualified_name" => {
                    Ok(unsafe {
                        Self::QualifiedName(
                            <QualifiedName<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "relative_name" => {
                    Ok(unsafe {
                        Self::RelativeName(
                            <RelativeName<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "subscript_expression" => {
                    Ok(unsafe {
                        Self::SubscriptExpression(
                            <SubscriptExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "update_expression" => {
                    Ok(unsafe {
                        Self::UpdateExpression(
                            <UpdateExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "variable_name" => {
                    Ok(unsafe {
                        Self::VariableName(
                            <VariableName<
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
                Self::AnonymousFunction(x) => ::type_sitter::Node::raw(x),
                Self::ArrayCreationExpression(x) => ::type_sitter::Node::raw(x),
                Self::ArrowFunction(x) => ::type_sitter::Node::raw(x),
                Self::ClassConstantAccessExpression(x) => ::type_sitter::Node::raw(x),
                Self::FunctionCallExpression(x) => ::type_sitter::Node::raw(x),
                Self::Literal(x) => ::type_sitter::Node::raw(x),
                Self::MemberAccessExpression(x) => ::type_sitter::Node::raw(x),
                Self::Name(x) => ::type_sitter::Node::raw(x),
                Self::ParenthesizedExpression(x) => ::type_sitter::Node::raw(x),
                Self::QualifiedName(x) => ::type_sitter::Node::raw(x),
                Self::RelativeName(x) => ::type_sitter::Node::raw(x),
                Self::SubscriptExpression(x) => ::type_sitter::Node::raw(x),
                Self::UpdateExpression(x) => ::type_sitter::Node::raw(x),
                Self::VariableName(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::AnonymousFunction(x) => ::type_sitter::Node::raw_mut(x),
                Self::ArrayCreationExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::ArrowFunction(x) => ::type_sitter::Node::raw_mut(x),
                Self::ClassConstantAccessExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::FunctionCallExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::Literal(x) => ::type_sitter::Node::raw_mut(x),
                Self::MemberAccessExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::Name(x) => ::type_sitter::Node::raw_mut(x),
                Self::ParenthesizedExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::QualifiedName(x) => ::type_sitter::Node::raw_mut(x),
                Self::RelativeName(x) => ::type_sitter::Node::raw_mut(x),
                Self::SubscriptExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::UpdateExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::VariableName(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::AnonymousFunction(x) => x.into_raw(),
                Self::ArrayCreationExpression(x) => x.into_raw(),
                Self::ArrowFunction(x) => x.into_raw(),
                Self::ClassConstantAccessExpression(x) => x.into_raw(),
                Self::FunctionCallExpression(x) => x.into_raw(),
                Self::Literal(x) => x.into_raw(),
                Self::MemberAccessExpression(x) => x.into_raw(),
                Self::Name(x) => x.into_raw(),
                Self::ParenthesizedExpression(x) => x.into_raw(),
                Self::QualifiedName(x) => x.into_raw(),
                Self::RelativeName(x) => x.into_raw(),
                Self::SubscriptExpression(x) => x.into_raw(),
                Self::UpdateExpression(x) => x.into_raw(),
                Self::VariableName(x) => x.into_raw(),
            }
        }
    }
    /**One of `{dynamic_variable_name | expression | member_access_expression | name | nullsafe_member_access_expression | parenthesized_expression | qualified_name | scoped_property_access_expression | subscript_expression | variable_name}`:
- [`DynamicVariableName`]
- [`Expression`]
- [`MemberAccessExpression`]
- [`Name`]
- [`NullsafeMemberAccessExpression`]
- [`ParenthesizedExpression`]
- [`QualifiedName`]
- [`ScopedPropertyAccessExpression`]
- [`SubscriptExpression`]
- [`VariableName`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Anon308160872237808256834920196883287176613<'tree> {
        DynamicVariableName(DynamicVariableName<'tree>),
        Expression(Expression<'tree>),
        MemberAccessExpression(MemberAccessExpression<'tree>),
        Name(Name<'tree>),
        NullsafeMemberAccessExpression(NullsafeMemberAccessExpression<'tree>),
        ParenthesizedExpression(ParenthesizedExpression<'tree>),
        QualifiedName(QualifiedName<'tree>),
        ScopedPropertyAccessExpression(ScopedPropertyAccessExpression<'tree>),
        SubscriptExpression(SubscriptExpression<'tree>),
        VariableName(VariableName<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Anon308160872237808256834920196883287176613<'tree> {
        ///Returns the node if it is of type `dynamic_variable_name` ([`DynamicVariableName`]), otherwise returns `None`
        #[inline]
        pub fn as_dynamic_variable_name(
            self,
        ) -> ::std::option::Option<DynamicVariableName<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DynamicVariableName(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `expression` ([`Expression`]), otherwise returns `None`
        #[inline]
        pub fn as_expression(self) -> ::std::option::Option<Expression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Expression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `member_access_expression` ([`MemberAccessExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_member_access_expression(
            self,
        ) -> ::std::option::Option<MemberAccessExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::MemberAccessExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `name` ([`Name`]), otherwise returns `None`
        #[inline]
        pub fn as_name(self) -> ::std::option::Option<Name<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Name(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `nullsafe_member_access_expression` ([`NullsafeMemberAccessExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_nullsafe_member_access_expression(
            self,
        ) -> ::std::option::Option<NullsafeMemberAccessExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::NullsafeMemberAccessExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `parenthesized_expression` ([`ParenthesizedExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_parenthesized_expression(
            self,
        ) -> ::std::option::Option<ParenthesizedExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ParenthesizedExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `qualified_name` ([`QualifiedName`]), otherwise returns `None`
        #[inline]
        pub fn as_qualified_name(self) -> ::std::option::Option<QualifiedName<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::QualifiedName(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `scoped_property_access_expression` ([`ScopedPropertyAccessExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_scoped_property_access_expression(
            self,
        ) -> ::std::option::Option<ScopedPropertyAccessExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ScopedPropertyAccessExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `subscript_expression` ([`SubscriptExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_subscript_expression(
            self,
        ) -> ::std::option::Option<SubscriptExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::SubscriptExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `variable_name` ([`VariableName`]), otherwise returns `None`
        #[inline]
        pub fn as_variable_name(self) -> ::std::option::Option<VariableName<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::VariableName(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
    for Anon308160872237808256834920196883287176613<'tree> {
        type WithLifetime<'a> = Anon308160872237808256834920196883287176613<'a>;
        const KIND: &'static str = "{dynamic_variable_name | expression | member_access_expression | name | nullsafe_member_access_expression | parenthesized_expression | qualified_name | scoped_property_access_expression | subscript_expression | variable_name}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "dynamic_variable_name" => {
                    Ok(unsafe {
                        Self::DynamicVariableName(
                            <DynamicVariableName<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "expression" => {
                    Ok(unsafe {
                        Self::Expression(
                            <Expression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "member_access_expression" => {
                    Ok(unsafe {
                        Self::MemberAccessExpression(
                            <MemberAccessExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "name" => {
                    Ok(unsafe {
                        Self::Name(
                            <Name<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "nullsafe_member_access_expression" => {
                    Ok(unsafe {
                        Self::NullsafeMemberAccessExpression(
                            <NullsafeMemberAccessExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "parenthesized_expression" => {
                    Ok(unsafe {
                        Self::ParenthesizedExpression(
                            <ParenthesizedExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "qualified_name" => {
                    Ok(unsafe {
                        Self::QualifiedName(
                            <QualifiedName<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "scoped_property_access_expression" => {
                    Ok(unsafe {
                        Self::ScopedPropertyAccessExpression(
                            <ScopedPropertyAccessExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "subscript_expression" => {
                    Ok(unsafe {
                        Self::SubscriptExpression(
                            <SubscriptExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "variable_name" => {
                    Ok(unsafe {
                        Self::VariableName(
                            <VariableName<
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
                Self::DynamicVariableName(x) => ::type_sitter::Node::raw(x),
                Self::Expression(x) => ::type_sitter::Node::raw(x),
                Self::MemberAccessExpression(x) => ::type_sitter::Node::raw(x),
                Self::Name(x) => ::type_sitter::Node::raw(x),
                Self::NullsafeMemberAccessExpression(x) => ::type_sitter::Node::raw(x),
                Self::ParenthesizedExpression(x) => ::type_sitter::Node::raw(x),
                Self::QualifiedName(x) => ::type_sitter::Node::raw(x),
                Self::ScopedPropertyAccessExpression(x) => ::type_sitter::Node::raw(x),
                Self::SubscriptExpression(x) => ::type_sitter::Node::raw(x),
                Self::VariableName(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::DynamicVariableName(x) => ::type_sitter::Node::raw_mut(x),
                Self::Expression(x) => ::type_sitter::Node::raw_mut(x),
                Self::MemberAccessExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::Name(x) => ::type_sitter::Node::raw_mut(x),
                Self::NullsafeMemberAccessExpression(x) => {
                    ::type_sitter::Node::raw_mut(x)
                }
                Self::ParenthesizedExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::QualifiedName(x) => ::type_sitter::Node::raw_mut(x),
                Self::ScopedPropertyAccessExpression(x) => {
                    ::type_sitter::Node::raw_mut(x)
                }
                Self::SubscriptExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::VariableName(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::DynamicVariableName(x) => x.into_raw(),
                Self::Expression(x) => x.into_raw(),
                Self::MemberAccessExpression(x) => x.into_raw(),
                Self::Name(x) => x.into_raw(),
                Self::NullsafeMemberAccessExpression(x) => x.into_raw(),
                Self::ParenthesizedExpression(x) => x.into_raw(),
                Self::QualifiedName(x) => x.into_raw(),
                Self::ScopedPropertyAccessExpression(x) => x.into_raw(),
                Self::SubscriptExpression(x) => x.into_raw(),
                Self::VariableName(x) => x.into_raw(),
            }
        }
    }
    /**One of `{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | expression | keyword | loop | php_statement | props | script_element | style_element | switch | text}`:
- [`Comment`]
- [`Conditional`]
- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Element`]
- [`Entity`]
- [`ErroneousEndTag`]
- [`Expression`]
- [`Keyword`]
- [`Loop`]
- [`PhpStatement`]
- [`Props`]
- [`ScriptElement`]
- [`StyleElement`]
- [`Switch`]
- [`Text`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Anon310217771700970732338886093335230545339<'tree> {
        Comment(Comment<'tree>),
        Conditional(Conditional<'tree>),
        Directive(Directive<'tree>),
        DirectiveEnd(DirectiveEnd<'tree>),
        DirectiveStart(DirectiveStart<'tree>),
        Element(Element<'tree>),
        Entity(Entity<'tree>),
        ErroneousEndTag(ErroneousEndTag<'tree>),
        Expression(Expression<'tree>),
        Keyword(Keyword<'tree>),
        Loop(Loop<'tree>),
        PhpStatement(PhpStatement<'tree>),
        Props(Props<'tree>),
        ScriptElement(ScriptElement<'tree>),
        StyleElement(StyleElement<'tree>),
        Switch(Switch<'tree>),
        Text(Text<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Anon310217771700970732338886093335230545339<'tree> {
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
        ///Returns the node if it is of type `expression` ([`Expression`]), otherwise returns `None`
        #[inline]
        pub fn as_expression(self) -> ::std::option::Option<Expression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Expression(x) = self {
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
        ///Returns the node if it is of type `props` ([`Props`]), otherwise returns `None`
        #[inline]
        pub fn as_props(self) -> ::std::option::Option<Props<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Props(x) = self {
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
    for Anon310217771700970732338886093335230545339<'tree> {
        type WithLifetime<'a> = Anon310217771700970732338886093335230545339<'a>;
        const KIND: &'static str = "{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | expression | keyword | loop | php_statement | props | script_element | style_element | switch | text}";
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
                "expression" => {
                    Ok(unsafe {
                        Self::Expression(
                            <Expression<
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
                "php_statement" => {
                    Ok(unsafe {
                        Self::PhpStatement(
                            <PhpStatement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "props" => {
                    Ok(unsafe {
                        Self::Props(
                            <Props<
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
                Self::Expression(x) => ::type_sitter::Node::raw(x),
                Self::Keyword(x) => ::type_sitter::Node::raw(x),
                Self::Loop(x) => ::type_sitter::Node::raw(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw(x),
                Self::Props(x) => ::type_sitter::Node::raw(x),
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
                Self::Expression(x) => ::type_sitter::Node::raw_mut(x),
                Self::Keyword(x) => ::type_sitter::Node::raw_mut(x),
                Self::Loop(x) => ::type_sitter::Node::raw_mut(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw_mut(x),
                Self::Props(x) => ::type_sitter::Node::raw_mut(x),
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
                Self::Expression(x) => x.into_raw(),
                Self::Keyword(x) => x.into_raw(),
                Self::Loop(x) => x.into_raw(),
                Self::PhpStatement(x) => x.into_raw(),
                Self::Props(x) => x.into_raw(),
                Self::ScriptElement(x) => x.into_raw(),
                Self::StyleElement(x) => x.into_raw(),
                Self::Switch(x) => x.into_raw(),
                Self::Text(x) => x.into_raw(),
            }
        }
    }
    /**One of `{array_creation_expression | dynamic_variable_name | member_access_expression | nullsafe_member_access_expression | parenthesized_expression | scoped_property_access_expression | subscript_expression | variable_name}`:
- [`ArrayCreationExpression`]
- [`DynamicVariableName`]
- [`MemberAccessExpression`]
- [`NullsafeMemberAccessExpression`]
- [`ParenthesizedExpression`]
- [`ScopedPropertyAccessExpression`]
- [`SubscriptExpression`]
- [`VariableName`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Anon314770949262501397840657149534365951507<'tree> {
        ArrayCreationExpression(ArrayCreationExpression<'tree>),
        DynamicVariableName(DynamicVariableName<'tree>),
        MemberAccessExpression(MemberAccessExpression<'tree>),
        NullsafeMemberAccessExpression(NullsafeMemberAccessExpression<'tree>),
        ParenthesizedExpression(ParenthesizedExpression<'tree>),
        ScopedPropertyAccessExpression(ScopedPropertyAccessExpression<'tree>),
        SubscriptExpression(SubscriptExpression<'tree>),
        VariableName(VariableName<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Anon314770949262501397840657149534365951507<'tree> {
        ///Returns the node if it is of type `array_creation_expression` ([`ArrayCreationExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_array_creation_expression(
            self,
        ) -> ::std::option::Option<ArrayCreationExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ArrayCreationExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `dynamic_variable_name` ([`DynamicVariableName`]), otherwise returns `None`
        #[inline]
        pub fn as_dynamic_variable_name(
            self,
        ) -> ::std::option::Option<DynamicVariableName<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DynamicVariableName(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `member_access_expression` ([`MemberAccessExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_member_access_expression(
            self,
        ) -> ::std::option::Option<MemberAccessExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::MemberAccessExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `nullsafe_member_access_expression` ([`NullsafeMemberAccessExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_nullsafe_member_access_expression(
            self,
        ) -> ::std::option::Option<NullsafeMemberAccessExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::NullsafeMemberAccessExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `parenthesized_expression` ([`ParenthesizedExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_parenthesized_expression(
            self,
        ) -> ::std::option::Option<ParenthesizedExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ParenthesizedExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `scoped_property_access_expression` ([`ScopedPropertyAccessExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_scoped_property_access_expression(
            self,
        ) -> ::std::option::Option<ScopedPropertyAccessExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ScopedPropertyAccessExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `subscript_expression` ([`SubscriptExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_subscript_expression(
            self,
        ) -> ::std::option::Option<SubscriptExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::SubscriptExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `variable_name` ([`VariableName`]), otherwise returns `None`
        #[inline]
        pub fn as_variable_name(self) -> ::std::option::Option<VariableName<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::VariableName(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
    for Anon314770949262501397840657149534365951507<'tree> {
        type WithLifetime<'a> = Anon314770949262501397840657149534365951507<'a>;
        const KIND: &'static str = "{array_creation_expression | dynamic_variable_name | member_access_expression | nullsafe_member_access_expression | parenthesized_expression | scoped_property_access_expression | subscript_expression | variable_name}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "array_creation_expression" => {
                    Ok(unsafe {
                        Self::ArrayCreationExpression(
                            <ArrayCreationExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "dynamic_variable_name" => {
                    Ok(unsafe {
                        Self::DynamicVariableName(
                            <DynamicVariableName<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "member_access_expression" => {
                    Ok(unsafe {
                        Self::MemberAccessExpression(
                            <MemberAccessExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "nullsafe_member_access_expression" => {
                    Ok(unsafe {
                        Self::NullsafeMemberAccessExpression(
                            <NullsafeMemberAccessExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "parenthesized_expression" => {
                    Ok(unsafe {
                        Self::ParenthesizedExpression(
                            <ParenthesizedExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "scoped_property_access_expression" => {
                    Ok(unsafe {
                        Self::ScopedPropertyAccessExpression(
                            <ScopedPropertyAccessExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "subscript_expression" => {
                    Ok(unsafe {
                        Self::SubscriptExpression(
                            <SubscriptExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "variable_name" => {
                    Ok(unsafe {
                        Self::VariableName(
                            <VariableName<
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
                Self::ArrayCreationExpression(x) => ::type_sitter::Node::raw(x),
                Self::DynamicVariableName(x) => ::type_sitter::Node::raw(x),
                Self::MemberAccessExpression(x) => ::type_sitter::Node::raw(x),
                Self::NullsafeMemberAccessExpression(x) => ::type_sitter::Node::raw(x),
                Self::ParenthesizedExpression(x) => ::type_sitter::Node::raw(x),
                Self::ScopedPropertyAccessExpression(x) => ::type_sitter::Node::raw(x),
                Self::SubscriptExpression(x) => ::type_sitter::Node::raw(x),
                Self::VariableName(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::ArrayCreationExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::DynamicVariableName(x) => ::type_sitter::Node::raw_mut(x),
                Self::MemberAccessExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::NullsafeMemberAccessExpression(x) => {
                    ::type_sitter::Node::raw_mut(x)
                }
                Self::ParenthesizedExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::ScopedPropertyAccessExpression(x) => {
                    ::type_sitter::Node::raw_mut(x)
                }
                Self::SubscriptExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::VariableName(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::ArrayCreationExpression(x) => x.into_raw(),
                Self::DynamicVariableName(x) => x.into_raw(),
                Self::MemberAccessExpression(x) => x.into_raw(),
                Self::NullsafeMemberAccessExpression(x) => x.into_raw(),
                Self::ParenthesizedExpression(x) => x.into_raw(),
                Self::ScopedPropertyAccessExpression(x) => x.into_raw(),
                Self::SubscriptExpression(x) => x.into_raw(),
                Self::VariableName(x) => x.into_raw(),
            }
        }
    }
    /**One of `{dynamic_variable_name | expression | member_access_expression | nullsafe_member_access_expression | scoped_property_access_expression | subscript_expression | variable_name}`:
- [`DynamicVariableName`]
- [`Expression`]
- [`MemberAccessExpression`]
- [`NullsafeMemberAccessExpression`]
- [`ScopedPropertyAccessExpression`]
- [`SubscriptExpression`]
- [`VariableName`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Anon58897980112289426888429290682076362032<'tree> {
        DynamicVariableName(DynamicVariableName<'tree>),
        Expression(Expression<'tree>),
        MemberAccessExpression(MemberAccessExpression<'tree>),
        NullsafeMemberAccessExpression(NullsafeMemberAccessExpression<'tree>),
        ScopedPropertyAccessExpression(ScopedPropertyAccessExpression<'tree>),
        SubscriptExpression(SubscriptExpression<'tree>),
        VariableName(VariableName<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Anon58897980112289426888429290682076362032<'tree> {
        ///Returns the node if it is of type `dynamic_variable_name` ([`DynamicVariableName`]), otherwise returns `None`
        #[inline]
        pub fn as_dynamic_variable_name(
            self,
        ) -> ::std::option::Option<DynamicVariableName<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DynamicVariableName(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `expression` ([`Expression`]), otherwise returns `None`
        #[inline]
        pub fn as_expression(self) -> ::std::option::Option<Expression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Expression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `member_access_expression` ([`MemberAccessExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_member_access_expression(
            self,
        ) -> ::std::option::Option<MemberAccessExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::MemberAccessExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `nullsafe_member_access_expression` ([`NullsafeMemberAccessExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_nullsafe_member_access_expression(
            self,
        ) -> ::std::option::Option<NullsafeMemberAccessExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::NullsafeMemberAccessExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `scoped_property_access_expression` ([`ScopedPropertyAccessExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_scoped_property_access_expression(
            self,
        ) -> ::std::option::Option<ScopedPropertyAccessExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ScopedPropertyAccessExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `subscript_expression` ([`SubscriptExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_subscript_expression(
            self,
        ) -> ::std::option::Option<SubscriptExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::SubscriptExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `variable_name` ([`VariableName`]), otherwise returns `None`
        #[inline]
        pub fn as_variable_name(self) -> ::std::option::Option<VariableName<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::VariableName(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
    for Anon58897980112289426888429290682076362032<'tree> {
        type WithLifetime<'a> = Anon58897980112289426888429290682076362032<'a>;
        const KIND: &'static str = "{dynamic_variable_name | expression | member_access_expression | nullsafe_member_access_expression | scoped_property_access_expression | subscript_expression | variable_name}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "dynamic_variable_name" => {
                    Ok(unsafe {
                        Self::DynamicVariableName(
                            <DynamicVariableName<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "expression" => {
                    Ok(unsafe {
                        Self::Expression(
                            <Expression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "member_access_expression" => {
                    Ok(unsafe {
                        Self::MemberAccessExpression(
                            <MemberAccessExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "nullsafe_member_access_expression" => {
                    Ok(unsafe {
                        Self::NullsafeMemberAccessExpression(
                            <NullsafeMemberAccessExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "scoped_property_access_expression" => {
                    Ok(unsafe {
                        Self::ScopedPropertyAccessExpression(
                            <ScopedPropertyAccessExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "subscript_expression" => {
                    Ok(unsafe {
                        Self::SubscriptExpression(
                            <SubscriptExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "variable_name" => {
                    Ok(unsafe {
                        Self::VariableName(
                            <VariableName<
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
                Self::DynamicVariableName(x) => ::type_sitter::Node::raw(x),
                Self::Expression(x) => ::type_sitter::Node::raw(x),
                Self::MemberAccessExpression(x) => ::type_sitter::Node::raw(x),
                Self::NullsafeMemberAccessExpression(x) => ::type_sitter::Node::raw(x),
                Self::ScopedPropertyAccessExpression(x) => ::type_sitter::Node::raw(x),
                Self::SubscriptExpression(x) => ::type_sitter::Node::raw(x),
                Self::VariableName(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::DynamicVariableName(x) => ::type_sitter::Node::raw_mut(x),
                Self::Expression(x) => ::type_sitter::Node::raw_mut(x),
                Self::MemberAccessExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::NullsafeMemberAccessExpression(x) => {
                    ::type_sitter::Node::raw_mut(x)
                }
                Self::ScopedPropertyAccessExpression(x) => {
                    ::type_sitter::Node::raw_mut(x)
                }
                Self::SubscriptExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::VariableName(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::DynamicVariableName(x) => x.into_raw(),
                Self::Expression(x) => x.into_raw(),
                Self::MemberAccessExpression(x) => x.into_raw(),
                Self::NullsafeMemberAccessExpression(x) => x.into_raw(),
                Self::ScopedPropertyAccessExpression(x) => x.into_raw(),
                Self::SubscriptExpression(x) => x.into_raw(),
                Self::VariableName(x) => x.into_raw(),
            }
        }
    }
    /**One of `{comment | conditional | conditional_keyword | directive | directive_end | directive_start | doctype | element | entity | envoy | erroneous_end_tag | expression | fragment | keyword | livewire | loop | once | php_statement | props | script_element | section | stack | style_element | switch | text | verbatim}`:
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
- [`Expression`]
- [`Fragment`]
- [`Keyword`]
- [`Livewire`]
- [`Loop`]
- [`Once`]
- [`PhpStatement`]
- [`Props`]
- [`ScriptElement`]
- [`Section`]
- [`Stack`]
- [`StyleElement`]
- [`Switch`]
- [`Text`]
- [`Verbatim`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Anon71931883397758457417774301653197094678<'tree> {
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
        Expression(Expression<'tree>),
        Fragment(Fragment<'tree>),
        Keyword(Keyword<'tree>),
        Livewire(Livewire<'tree>),
        Loop(Loop<'tree>),
        Once(Once<'tree>),
        PhpStatement(PhpStatement<'tree>),
        Props(Props<'tree>),
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
    impl<'tree> Anon71931883397758457417774301653197094678<'tree> {
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
        ///Returns the node if it is of type `expression` ([`Expression`]), otherwise returns `None`
        #[inline]
        pub fn as_expression(self) -> ::std::option::Option<Expression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Expression(x) = self {
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
        ///Returns the node if it is of type `props` ([`Props`]), otherwise returns `None`
        #[inline]
        pub fn as_props(self) -> ::std::option::Option<Props<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Props(x) = self {
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
    for Anon71931883397758457417774301653197094678<'tree> {
        type WithLifetime<'a> = Anon71931883397758457417774301653197094678<'a>;
        const KIND: &'static str = "{comment | conditional | conditional_keyword | directive | directive_end | directive_start | doctype | element | entity | envoy | erroneous_end_tag | expression | fragment | keyword | livewire | loop | once | php_statement | props | script_element | section | stack | style_element | switch | text | verbatim}";
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
                "expression" => {
                    Ok(unsafe {
                        Self::Expression(
                            <Expression<
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
                "php_statement" => {
                    Ok(unsafe {
                        Self::PhpStatement(
                            <PhpStatement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "props" => {
                    Ok(unsafe {
                        Self::Props(
                            <Props<
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
                Self::Expression(x) => ::type_sitter::Node::raw(x),
                Self::Fragment(x) => ::type_sitter::Node::raw(x),
                Self::Keyword(x) => ::type_sitter::Node::raw(x),
                Self::Livewire(x) => ::type_sitter::Node::raw(x),
                Self::Loop(x) => ::type_sitter::Node::raw(x),
                Self::Once(x) => ::type_sitter::Node::raw(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw(x),
                Self::Props(x) => ::type_sitter::Node::raw(x),
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
                Self::Expression(x) => ::type_sitter::Node::raw_mut(x),
                Self::Fragment(x) => ::type_sitter::Node::raw_mut(x),
                Self::Keyword(x) => ::type_sitter::Node::raw_mut(x),
                Self::Livewire(x) => ::type_sitter::Node::raw_mut(x),
                Self::Loop(x) => ::type_sitter::Node::raw_mut(x),
                Self::Once(x) => ::type_sitter::Node::raw_mut(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw_mut(x),
                Self::Props(x) => ::type_sitter::Node::raw_mut(x),
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
                Self::Expression(x) => x.into_raw(),
                Self::Fragment(x) => x.into_raw(),
                Self::Keyword(x) => x.into_raw(),
                Self::Livewire(x) => x.into_raw(),
                Self::Loop(x) => x.into_raw(),
                Self::Once(x) => x.into_raw(),
                Self::PhpStatement(x) => x.into_raw(),
                Self::Props(x) => x.into_raw(),
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
    /**One of `{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | expression | keyword | loop | php_statement | props | script_element | style_element | switch | text | verbatim}`:
- [`Comment`]
- [`Conditional`]
- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Element`]
- [`Entity`]
- [`ErroneousEndTag`]
- [`Expression`]
- [`Keyword`]
- [`Loop`]
- [`PhpStatement`]
- [`Props`]
- [`ScriptElement`]
- [`StyleElement`]
- [`Switch`]
- [`Text`]
- [`Verbatim`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Anon94390688163068428215780624704411407183<'tree> {
        Comment(Comment<'tree>),
        Conditional(Conditional<'tree>),
        Directive(Directive<'tree>),
        DirectiveEnd(DirectiveEnd<'tree>),
        DirectiveStart(DirectiveStart<'tree>),
        Element(Element<'tree>),
        Entity(Entity<'tree>),
        ErroneousEndTag(ErroneousEndTag<'tree>),
        Expression(Expression<'tree>),
        Keyword(Keyword<'tree>),
        Loop(Loop<'tree>),
        PhpStatement(PhpStatement<'tree>),
        Props(Props<'tree>),
        ScriptElement(ScriptElement<'tree>),
        StyleElement(StyleElement<'tree>),
        Switch(Switch<'tree>),
        Text(Text<'tree>),
        Verbatim(Verbatim<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Anon94390688163068428215780624704411407183<'tree> {
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
        ///Returns the node if it is of type `expression` ([`Expression`]), otherwise returns `None`
        #[inline]
        pub fn as_expression(self) -> ::std::option::Option<Expression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Expression(x) = self {
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
        ///Returns the node if it is of type `props` ([`Props`]), otherwise returns `None`
        #[inline]
        pub fn as_props(self) -> ::std::option::Option<Props<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Props(x) = self {
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
    for Anon94390688163068428215780624704411407183<'tree> {
        type WithLifetime<'a> = Anon94390688163068428215780624704411407183<'a>;
        const KIND: &'static str = "{comment | conditional | directive | directive_end | directive_start | element | entity | erroneous_end_tag | expression | keyword | loop | php_statement | props | script_element | style_element | switch | text | verbatim}";
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
                "expression" => {
                    Ok(unsafe {
                        Self::Expression(
                            <Expression<
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
                "php_statement" => {
                    Ok(unsafe {
                        Self::PhpStatement(
                            <PhpStatement<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "props" => {
                    Ok(unsafe {
                        Self::Props(
                            <Props<
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
                Self::Expression(x) => ::type_sitter::Node::raw(x),
                Self::Keyword(x) => ::type_sitter::Node::raw(x),
                Self::Loop(x) => ::type_sitter::Node::raw(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw(x),
                Self::Props(x) => ::type_sitter::Node::raw(x),
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
                Self::Expression(x) => ::type_sitter::Node::raw_mut(x),
                Self::Keyword(x) => ::type_sitter::Node::raw_mut(x),
                Self::Loop(x) => ::type_sitter::Node::raw_mut(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw_mut(x),
                Self::Props(x) => ::type_sitter::Node::raw_mut(x),
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
                Self::Expression(x) => x.into_raw(),
                Self::Keyword(x) => x.into_raw(),
                Self::Loop(x) => x.into_raw(),
                Self::PhpStatement(x) => x.into_raw(),
                Self::Props(x) => x.into_raw(),
                Self::ScriptElement(x) => x.into_raw(),
                Self::StyleElement(x) => x.into_raw(),
                Self::Switch(x) => x.into_raw(),
                Self::Text(x) => x.into_raw(),
                Self::Verbatim(x) => x.into_raw(),
            }
        }
    }
    /**One of `{array_element_key_value_initializer | array_element_spreading_initializer | array_element_value_initializer}`:
- [`ArrayElementKeyValueInitializer`]
- [`ArrayElementSpreadingInitializer`]
- [`ArrayElementValueInitializer`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ArrayElementKeyValueInitializer_ArrayElementSpreadingInitializer_ArrayElementValueInitializer<
        'tree,
    > {
        ArrayElementKeyValueInitializer(ArrayElementKeyValueInitializer<'tree>),
        ArrayElementSpreadingInitializer(ArrayElementSpreadingInitializer<'tree>),
        ArrayElementValueInitializer(ArrayElementValueInitializer<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<
        'tree,
    > ArrayElementKeyValueInitializer_ArrayElementSpreadingInitializer_ArrayElementValueInitializer<
        'tree,
    > {
        ///Returns the node if it is of type `array_element_key_value_initializer` ([`ArrayElementKeyValueInitializer`]), otherwise returns `None`
        #[inline]
        pub fn as_array_element_key_value_initializer(
            self,
        ) -> ::std::option::Option<ArrayElementKeyValueInitializer<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ArrayElementKeyValueInitializer(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `array_element_spreading_initializer` ([`ArrayElementSpreadingInitializer`]), otherwise returns `None`
        #[inline]
        pub fn as_array_element_spreading_initializer(
            self,
        ) -> ::std::option::Option<ArrayElementSpreadingInitializer<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ArrayElementSpreadingInitializer(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `array_element_value_initializer` ([`ArrayElementValueInitializer`]), otherwise returns `None`
        #[inline]
        pub fn as_array_element_value_initializer(
            self,
        ) -> ::std::option::Option<ArrayElementValueInitializer<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ArrayElementValueInitializer(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
    for ArrayElementKeyValueInitializer_ArrayElementSpreadingInitializer_ArrayElementValueInitializer<
        'tree,
    > {
        type WithLifetime<'a> = ArrayElementKeyValueInitializer_ArrayElementSpreadingInitializer_ArrayElementValueInitializer<
            'a,
        >;
        const KIND: &'static str = "{array_element_key_value_initializer | array_element_spreading_initializer | array_element_value_initializer}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "array_element_key_value_initializer" => {
                    Ok(unsafe {
                        Self::ArrayElementKeyValueInitializer(
                            <ArrayElementKeyValueInitializer<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "array_element_spreading_initializer" => {
                    Ok(unsafe {
                        Self::ArrayElementSpreadingInitializer(
                            <ArrayElementSpreadingInitializer<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "array_element_value_initializer" => {
                    Ok(unsafe {
                        Self::ArrayElementValueInitializer(
                            <ArrayElementValueInitializer<
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
                Self::ArrayElementKeyValueInitializer(x) => ::type_sitter::Node::raw(x),
                Self::ArrayElementSpreadingInitializer(x) => ::type_sitter::Node::raw(x),
                Self::ArrayElementValueInitializer(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::ArrayElementKeyValueInitializer(x) => {
                    ::type_sitter::Node::raw_mut(x)
                }
                Self::ArrayElementSpreadingInitializer(x) => {
                    ::type_sitter::Node::raw_mut(x)
                }
                Self::ArrayElementValueInitializer(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::ArrayElementKeyValueInitializer(x) => x.into_raw(),
                Self::ArrayElementSpreadingInitializer(x) => x.into_raw(),
                Self::ArrayElementValueInitializer(x) => x.into_raw(),
            }
        }
    }
    /**One of `{attribute_name | attribute_value | directive | expression | php_statement | quoted_attribute_value}`:
- [`AttributeName`]
- [`AttributeValue`]
- [`Directive`]
- [`Expression`]
- [`PhpStatement`]
- [`QuotedAttributeValue`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum AttributeName_AttributeValue_Directive_Expression_PhpStatement_QuotedAttributeValue<
        'tree,
    > {
        AttributeName(AttributeName<'tree>),
        AttributeValue(AttributeValue<'tree>),
        Directive(Directive<'tree>),
        Expression(Expression<'tree>),
        PhpStatement(PhpStatement<'tree>),
        QuotedAttributeValue(QuotedAttributeValue<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<
        'tree,
    > AttributeName_AttributeValue_Directive_Expression_PhpStatement_QuotedAttributeValue<
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
        ///Returns the node if it is of type `expression` ([`Expression`]), otherwise returns `None`
        #[inline]
        pub fn as_expression(self) -> ::std::option::Option<Expression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Expression(x) = self {
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
    for AttributeName_AttributeValue_Directive_Expression_PhpStatement_QuotedAttributeValue<
        'tree,
    > {
        type WithLifetime<'a> = AttributeName_AttributeValue_Directive_Expression_PhpStatement_QuotedAttributeValue<
            'a,
        >;
        const KIND: &'static str = "{attribute_name | attribute_value | directive | expression | php_statement | quoted_attribute_value}";
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
                "expression" => {
                    Ok(unsafe {
                        Self::Expression(
                            <Expression<
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
                Self::Expression(x) => ::type_sitter::Node::raw(x),
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
                Self::Expression(x) => ::type_sitter::Node::raw_mut(x),
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
                Self::Expression(x) => x.into_raw(),
                Self::PhpStatement(x) => x.into_raw(),
                Self::QuotedAttributeValue(x) => x.into_raw(),
            }
        }
    }
    /**One of `{attribute_value | comment | conditional | directive | expression | php_statement | props}`:
- [`AttributeValue`]
- [`Comment`]
- [`Conditional`]
- [`Directive`]
- [`Expression`]
- [`PhpStatement`]
- [`Props`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum AttributeValue_Comment_Conditional_Directive_Expression_PhpStatement_Props<
        'tree,
    > {
        AttributeValue(AttributeValue<'tree>),
        Comment(Comment<'tree>),
        Conditional(Conditional<'tree>),
        Directive(Directive<'tree>),
        Expression(Expression<'tree>),
        PhpStatement(PhpStatement<'tree>),
        Props(Props<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<
        'tree,
    > AttributeValue_Comment_Conditional_Directive_Expression_PhpStatement_Props<'tree> {
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
        ///Returns the node if it is of type `expression` ([`Expression`]), otherwise returns `None`
        #[inline]
        pub fn as_expression(self) -> ::std::option::Option<Expression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Expression(x) = self {
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
        ///Returns the node if it is of type `props` ([`Props`]), otherwise returns `None`
        #[inline]
        pub fn as_props(self) -> ::std::option::Option<Props<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Props(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
    for AttributeValue_Comment_Conditional_Directive_Expression_PhpStatement_Props<
        'tree,
    > {
        type WithLifetime<'a> = AttributeValue_Comment_Conditional_Directive_Expression_PhpStatement_Props<
            'a,
        >;
        const KIND: &'static str = "{attribute_value | comment | conditional | directive | expression | php_statement | props}";
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
                "expression" => {
                    Ok(unsafe {
                        Self::Expression(
                            <Expression<
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
                "props" => {
                    Ok(unsafe {
                        Self::Props(
                            <Props<
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
                Self::Expression(x) => ::type_sitter::Node::raw(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw(x),
                Self::Props(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::AttributeValue(x) => ::type_sitter::Node::raw_mut(x),
                Self::Comment(x) => ::type_sitter::Node::raw_mut(x),
                Self::Conditional(x) => ::type_sitter::Node::raw_mut(x),
                Self::Directive(x) => ::type_sitter::Node::raw_mut(x),
                Self::Expression(x) => ::type_sitter::Node::raw_mut(x),
                Self::PhpStatement(x) => ::type_sitter::Node::raw_mut(x),
                Self::Props(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::AttributeValue(x) => x.into_raw(),
                Self::Comment(x) => x.into_raw(),
                Self::Conditional(x) => x.into_raw(),
                Self::Directive(x) => x.into_raw(),
                Self::Expression(x) => x.into_raw(),
                Self::PhpStatement(x) => x.into_raw(),
                Self::Props(x) => x.into_raw(),
            }
        }
    }
    /**One of `{\ | namespace_name}`:
- [`symbols::Backslash`]
- [`NamespaceName`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Backslash_NamespaceName<'tree> {
        Backslash(symbols::Backslash<'tree>),
        NamespaceName(NamespaceName<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Backslash_NamespaceName<'tree> {
        ///Returns the node if it is of type `\` ([`symbols::Backslash`]), otherwise returns `None`
        #[inline]
        pub fn as_backslash(self) -> ::std::option::Option<symbols::Backslash<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Backslash(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `namespace_name` ([`NamespaceName`]), otherwise returns `None`
        #[inline]
        pub fn as_namespace_name(self) -> ::std::option::Option<NamespaceName<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::NamespaceName(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Backslash_NamespaceName<'tree> {
        type WithLifetime<'a> = Backslash_NamespaceName<'a>;
        const KIND: &'static str = "{\\ | namespace_name}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "\\" => {
                    Ok(unsafe {
                        Self::Backslash(
                            <symbols::Backslash<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "namespace_name" => {
                    Ok(unsafe {
                        Self::NamespaceName(
                            <NamespaceName<
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
                Self::Backslash(x) => ::type_sitter::Node::raw(x),
                Self::NamespaceName(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Backslash(x) => ::type_sitter::Node::raw_mut(x),
                Self::NamespaceName(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Backslash(x) => x.into_raw(),
                Self::NamespaceName(x) => x.into_raw(),
            }
        }
    }
    /**One of `{\ | namespace | namespace_name}`:
- [`symbols::Backslash`]
- [`unnamed::Namespace`]
- [`NamespaceName`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Backslash_Namespace_NamespaceName<'tree> {
        Backslash(symbols::Backslash<'tree>),
        Namespace(unnamed::Namespace<'tree>),
        NamespaceName(NamespaceName<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Backslash_Namespace_NamespaceName<'tree> {
        ///Returns the node if it is of type `\` ([`symbols::Backslash`]), otherwise returns `None`
        #[inline]
        pub fn as_backslash(self) -> ::std::option::Option<symbols::Backslash<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Backslash(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `namespace` ([`unnamed::Namespace`]), otherwise returns `None`
        #[inline]
        pub fn as_namespace(self) -> ::std::option::Option<unnamed::Namespace<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Namespace(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `namespace_name` ([`NamespaceName`]), otherwise returns `None`
        #[inline]
        pub fn as_namespace_name(self) -> ::std::option::Option<NamespaceName<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::NamespaceName(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Backslash_Namespace_NamespaceName<'tree> {
        type WithLifetime<'a> = Backslash_Namespace_NamespaceName<'a>;
        const KIND: &'static str = "{\\ | namespace | namespace_name}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "\\" => {
                    Ok(unsafe {
                        Self::Backslash(
                            <symbols::Backslash<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "namespace" => {
                    Ok(unsafe {
                        Self::Namespace(
                            <unnamed::Namespace<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "namespace_name" => {
                    Ok(unsafe {
                        Self::NamespaceName(
                            <NamespaceName<
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
                Self::Backslash(x) => ::type_sitter::Node::raw(x),
                Self::Namespace(x) => ::type_sitter::Node::raw(x),
                Self::NamespaceName(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Backslash(x) => ::type_sitter::Node::raw_mut(x),
                Self::Namespace(x) => ::type_sitter::Node::raw_mut(x),
                Self::NamespaceName(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Backslash(x) => x.into_raw(),
                Self::Namespace(x) => x.into_raw(),
                Self::NamespaceName(x) => x.into_raw(),
            }
        }
    }
    /**One of `{boolean | encapsed_string | float | integer | null | string}`:
- [`Boolean`]
- [`EncapsedString`]
- [`Float`]
- [`Integer`]
- [`Null`]
- [`String`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Boolean_EncapsedString_Float_Integer_Null_String<'tree> {
        Boolean(Boolean<'tree>),
        EncapsedString(EncapsedString<'tree>),
        Float(Float<'tree>),
        Integer(Integer<'tree>),
        Null(Null<'tree>),
        String(String<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Boolean_EncapsedString_Float_Integer_Null_String<'tree> {
        ///Returns the node if it is of type `boolean` ([`Boolean`]), otherwise returns `None`
        #[inline]
        pub fn as_boolean(self) -> ::std::option::Option<Boolean<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Boolean(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `encapsed_string` ([`EncapsedString`]), otherwise returns `None`
        #[inline]
        pub fn as_encapsed_string(self) -> ::std::option::Option<EncapsedString<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::EncapsedString(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `float` ([`Float`]), otherwise returns `None`
        #[inline]
        pub fn as_float_(self) -> ::std::option::Option<Float<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Float(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `integer` ([`Integer`]), otherwise returns `None`
        #[inline]
        pub fn as_integer(self) -> ::std::option::Option<Integer<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Integer(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `null` ([`Null`]), otherwise returns `None`
        #[inline]
        pub fn as_null(self) -> ::std::option::Option<Null<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Null(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `string` ([`String`]), otherwise returns `None`
        #[inline]
        pub fn as_string(self) -> ::std::option::Option<String<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::String(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
    for Boolean_EncapsedString_Float_Integer_Null_String<'tree> {
        type WithLifetime<'a> = Boolean_EncapsedString_Float_Integer_Null_String<'a>;
        const KIND: &'static str = "{boolean | encapsed_string | float | integer | null | string}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "boolean" => {
                    Ok(unsafe {
                        Self::Boolean(
                            <Boolean<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "encapsed_string" => {
                    Ok(unsafe {
                        Self::EncapsedString(
                            <EncapsedString<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "float" => {
                    Ok(unsafe {
                        Self::Float(
                            <Float<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "integer" => {
                    Ok(unsafe {
                        Self::Integer(
                            <Integer<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "null" => {
                    Ok(unsafe {
                        Self::Null(
                            <Null<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "string" => {
                    Ok(unsafe {
                        Self::String(
                            <String<
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
                Self::Boolean(x) => ::type_sitter::Node::raw(x),
                Self::EncapsedString(x) => ::type_sitter::Node::raw(x),
                Self::Float(x) => ::type_sitter::Node::raw(x),
                Self::Integer(x) => ::type_sitter::Node::raw(x),
                Self::Null(x) => ::type_sitter::Node::raw(x),
                Self::String(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Boolean(x) => ::type_sitter::Node::raw_mut(x),
                Self::EncapsedString(x) => ::type_sitter::Node::raw_mut(x),
                Self::Float(x) => ::type_sitter::Node::raw_mut(x),
                Self::Integer(x) => ::type_sitter::Node::raw_mut(x),
                Self::Null(x) => ::type_sitter::Node::raw_mut(x),
                Self::String(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Boolean(x) => x.into_raw(),
                Self::EncapsedString(x) => x.into_raw(),
                Self::Float(x) => x.into_raw(),
                Self::Integer(x) => x.into_raw(),
                Self::Null(x) => x.into_raw(),
                Self::String(x) => x.into_raw(),
            }
        }
    }
    /**One of `{bottom_type | type}`:
- [`BottomType`]
- [`Type`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum BottomType_Type<'tree> {
        BottomType(BottomType<'tree>),
        Type(Type<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> BottomType_Type<'tree> {
        ///Returns the node if it is of type `bottom_type` ([`BottomType`]), otherwise returns `None`
        #[inline]
        pub fn as_bottom_type(self) -> ::std::option::Option<BottomType<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::BottomType(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `type` ([`Type`]), otherwise returns `None`
        #[inline]
        pub fn as_type(self) -> ::std::option::Option<Type<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Type(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for BottomType_Type<'tree> {
        type WithLifetime<'a> = BottomType_Type<'a>;
        const KIND: &'static str = "{bottom_type | type}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "bottom_type" => {
                    Ok(unsafe {
                        Self::BottomType(
                            <BottomType<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "type" => {
                    Ok(unsafe {
                        Self::Type(
                            <Type<
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
                Self::BottomType(x) => ::type_sitter::Node::raw(x),
                Self::Type(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::BottomType(x) => ::type_sitter::Node::raw_mut(x),
                Self::Type(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::BottomType(x) => x.into_raw(),
                Self::Type(x) => x.into_raw(),
            }
        }
    }
    /**One of `{by_ref | variable_name}`:
- [`ByRef`]
- [`VariableName`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ByRef_VariableName<'tree> {
        ByRef(ByRef<'tree>),
        VariableName(VariableName<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> ByRef_VariableName<'tree> {
        ///Returns the node if it is of type `by_ref` ([`ByRef`]), otherwise returns `None`
        #[inline]
        pub fn as_by_ref(self) -> ::std::option::Option<ByRef<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ByRef(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `variable_name` ([`VariableName`]), otherwise returns `None`
        #[inline]
        pub fn as_variable_name(self) -> ::std::option::Option<VariableName<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::VariableName(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for ByRef_VariableName<'tree> {
        type WithLifetime<'a> = ByRef_VariableName<'a>;
        const KIND: &'static str = "{by_ref | variable_name}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "by_ref" => {
                    Ok(unsafe {
                        Self::ByRef(
                            <ByRef<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "variable_name" => {
                    Ok(unsafe {
                        Self::VariableName(
                            <VariableName<
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
                Self::ByRef(x) => ::type_sitter::Node::raw(x),
                Self::VariableName(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::ByRef(x) => ::type_sitter::Node::raw_mut(x),
                Self::VariableName(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::ByRef(x) => x.into_raw(),
                Self::VariableName(x) => x.into_raw(),
            }
        }
    }
    /**One of `{cast_expression | expression | primary_expression | unary_op_expression}`:
- [`CastExpression`]
- [`Expression`]
- [`PrimaryExpression`]
- [`UnaryOpExpression`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum CastExpression_Expression_PrimaryExpression_UnaryOpExpression<'tree> {
        CastExpression(CastExpression<'tree>),
        Expression(Expression<'tree>),
        PrimaryExpression(PrimaryExpression<'tree>),
        UnaryOpExpression(UnaryOpExpression<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> CastExpression_Expression_PrimaryExpression_UnaryOpExpression<'tree> {
        ///Returns the node if it is of type `cast_expression` ([`CastExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_cast_expression(self) -> ::std::option::Option<CastExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::CastExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `expression` ([`Expression`]), otherwise returns `None`
        #[inline]
        pub fn as_expression(self) -> ::std::option::Option<Expression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Expression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `primary_expression` ([`PrimaryExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_primary_expression(
            self,
        ) -> ::std::option::Option<PrimaryExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::PrimaryExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `unary_op_expression` ([`UnaryOpExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_unary_op_expression(
            self,
        ) -> ::std::option::Option<UnaryOpExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::UnaryOpExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
    for CastExpression_Expression_PrimaryExpression_UnaryOpExpression<'tree> {
        type WithLifetime<'a> = CastExpression_Expression_PrimaryExpression_UnaryOpExpression<
            'a,
        >;
        const KIND: &'static str = "{cast_expression | expression | primary_expression | unary_op_expression}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "cast_expression" => {
                    Ok(unsafe {
                        Self::CastExpression(
                            <CastExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "expression" => {
                    Ok(unsafe {
                        Self::Expression(
                            <Expression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "primary_expression" => {
                    Ok(unsafe {
                        Self::PrimaryExpression(
                            <PrimaryExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "unary_op_expression" => {
                    Ok(unsafe {
                        Self::UnaryOpExpression(
                            <UnaryOpExpression<
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
                Self::CastExpression(x) => ::type_sitter::Node::raw(x),
                Self::Expression(x) => ::type_sitter::Node::raw(x),
                Self::PrimaryExpression(x) => ::type_sitter::Node::raw(x),
                Self::UnaryOpExpression(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::CastExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::Expression(x) => ::type_sitter::Node::raw_mut(x),
                Self::PrimaryExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::UnaryOpExpression(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::CastExpression(x) => x.into_raw(),
                Self::Expression(x) => x.into_raw(),
                Self::PrimaryExpression(x) => x.into_raw(),
                Self::UnaryOpExpression(x) => x.into_raw(),
            }
        }
    }
    /**One of `{cast_expression | primary_expression | unary_op_expression}`:
- [`CastExpression`]
- [`PrimaryExpression`]
- [`UnaryOpExpression`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum CastExpression_PrimaryExpression_UnaryOpExpression<'tree> {
        CastExpression(CastExpression<'tree>),
        PrimaryExpression(PrimaryExpression<'tree>),
        UnaryOpExpression(UnaryOpExpression<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> CastExpression_PrimaryExpression_UnaryOpExpression<'tree> {
        ///Returns the node if it is of type `cast_expression` ([`CastExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_cast_expression(self) -> ::std::option::Option<CastExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::CastExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `primary_expression` ([`PrimaryExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_primary_expression(
            self,
        ) -> ::std::option::Option<PrimaryExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::PrimaryExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `unary_op_expression` ([`UnaryOpExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_unary_op_expression(
            self,
        ) -> ::std::option::Option<UnaryOpExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::UnaryOpExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
    for CastExpression_PrimaryExpression_UnaryOpExpression<'tree> {
        type WithLifetime<'a> = CastExpression_PrimaryExpression_UnaryOpExpression<'a>;
        const KIND: &'static str = "{cast_expression | primary_expression | unary_op_expression}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "cast_expression" => {
                    Ok(unsafe {
                        Self::CastExpression(
                            <CastExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "primary_expression" => {
                    Ok(unsafe {
                        Self::PrimaryExpression(
                            <PrimaryExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "unary_op_expression" => {
                    Ok(unsafe {
                        Self::UnaryOpExpression(
                            <UnaryOpExpression<
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
                Self::CastExpression(x) => ::type_sitter::Node::raw(x),
                Self::PrimaryExpression(x) => ::type_sitter::Node::raw(x),
                Self::UnaryOpExpression(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::CastExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::PrimaryExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::UnaryOpExpression(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::CastExpression(x) => x.into_raw(),
                Self::PrimaryExpression(x) => x.into_raw(),
                Self::UnaryOpExpression(x) => x.into_raw(),
            }
        }
    }
    /**One of `{conditional_keyword | directive | directive_end | directive_start | expression | php_only | text}`:
- [`ConditionalKeyword`]
- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Expression`]
- [`PhpOnly`]
- [`Text`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ConditionalKeyword_Directive_DirectiveEnd_DirectiveStart_Expression_PhpOnly_Text<
        'tree,
    > {
        ConditionalKeyword(ConditionalKeyword<'tree>),
        Directive(Directive<'tree>),
        DirectiveEnd(DirectiveEnd<'tree>),
        DirectiveStart(DirectiveStart<'tree>),
        Expression(Expression<'tree>),
        PhpOnly(PhpOnly<'tree>),
        Text(Text<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<
        'tree,
    > ConditionalKeyword_Directive_DirectiveEnd_DirectiveStart_Expression_PhpOnly_Text<
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
        ///Returns the node if it is of type `expression` ([`Expression`]), otherwise returns `None`
        #[inline]
        pub fn as_expression(self) -> ::std::option::Option<Expression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Expression(x) = self {
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
    for ConditionalKeyword_Directive_DirectiveEnd_DirectiveStart_Expression_PhpOnly_Text<
        'tree,
    > {
        type WithLifetime<'a> = ConditionalKeyword_Directive_DirectiveEnd_DirectiveStart_Expression_PhpOnly_Text<
            'a,
        >;
        const KIND: &'static str = "{conditional_keyword | directive | directive_end | directive_start | expression | php_only | text}";
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
                "expression" => {
                    Ok(unsafe {
                        Self::Expression(
                            <Expression<
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
                Self::Expression(x) => ::type_sitter::Node::raw(x),
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
                Self::Expression(x) => ::type_sitter::Node::raw_mut(x),
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
                Self::Expression(x) => x.into_raw(),
                Self::PhpOnly(x) => x.into_raw(),
                Self::Text(x) => x.into_raw(),
            }
        }
    }
    /**One of `{directive | directive_end | directive_start | expression | php_end_tag | php_only | php_tag}`:
- [`Directive`]
- [`DirectiveEnd`]
- [`DirectiveStart`]
- [`Expression`]
- [`PhpEndTag`]
- [`PhpOnly`]
- [`PhpTag`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Directive_DirectiveEnd_DirectiveStart_Expression_PhpEndTag_PhpOnly_PhpTag<
        'tree,
    > {
        Directive(Directive<'tree>),
        DirectiveEnd(DirectiveEnd<'tree>),
        DirectiveStart(DirectiveStart<'tree>),
        Expression(Expression<'tree>),
        PhpEndTag(PhpEndTag<'tree>),
        PhpOnly(PhpOnly<'tree>),
        PhpTag(PhpTag<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<
        'tree,
    > Directive_DirectiveEnd_DirectiveStart_Expression_PhpEndTag_PhpOnly_PhpTag<'tree> {
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
        ///Returns the node if it is of type `expression` ([`Expression`]), otherwise returns `None`
        #[inline]
        pub fn as_expression(self) -> ::std::option::Option<Expression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Expression(x) = self {
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
    for Directive_DirectiveEnd_DirectiveStart_Expression_PhpEndTag_PhpOnly_PhpTag<
        'tree,
    > {
        type WithLifetime<'a> = Directive_DirectiveEnd_DirectiveStart_Expression_PhpEndTag_PhpOnly_PhpTag<
            'a,
        >;
        const KIND: &'static str = "{directive | directive_end | directive_start | expression | php_end_tag | php_only | php_tag}";
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
                "expression" => {
                    Ok(unsafe {
                        Self::Expression(
                            <Expression<
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
                Self::Expression(x) => ::type_sitter::Node::raw(x),
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
                Self::Expression(x) => ::type_sitter::Node::raw_mut(x),
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
                Self::Expression(x) => x.into_raw(),
                Self::PhpEndTag(x) => x.into_raw(),
                Self::PhpOnly(x) => x.into_raw(),
                Self::PhpTag(x) => x.into_raw(),
            }
        }
    }
    /**One of `{directive | expression}`:
- [`Directive`]
- [`Expression`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Directive_Expression<'tree> {
        Directive(Directive<'tree>),
        Expression(Expression<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Directive_Expression<'tree> {
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
        ///Returns the node if it is of type `expression` ([`Expression`]), otherwise returns `None`
        #[inline]
        pub fn as_expression(self) -> ::std::option::Option<Expression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Expression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Directive_Expression<'tree> {
        type WithLifetime<'a> = Directive_Expression<'a>;
        const KIND: &'static str = "{directive | expression}";
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
                "expression" => {
                    Ok(unsafe {
                        Self::Expression(
                            <Expression<
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
                Self::Expression(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Directive(x) => ::type_sitter::Node::raw_mut(x),
                Self::Expression(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Directive(x) => x.into_raw(),
                Self::Expression(x) => x.into_raw(),
            }
        }
    }
    /**One of `{disjunctive_normal_form_type | intersection_type | named_type | optional_type | primitive_type | union_type}`:
- [`DisjunctiveNormalFormType`]
- [`IntersectionType`]
- [`NamedType`]
- [`OptionalType`]
- [`PrimitiveType`]
- [`UnionType`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum DisjunctiveNormalFormType_IntersectionType_NamedType_OptionalType_PrimitiveType_UnionType<
        'tree,
    > {
        DisjunctiveNormalFormType(DisjunctiveNormalFormType<'tree>),
        IntersectionType(IntersectionType<'tree>),
        NamedType(NamedType<'tree>),
        OptionalType(OptionalType<'tree>),
        PrimitiveType(PrimitiveType<'tree>),
        UnionType(UnionType<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<
        'tree,
    > DisjunctiveNormalFormType_IntersectionType_NamedType_OptionalType_PrimitiveType_UnionType<
        'tree,
    > {
        ///Returns the node if it is of type `disjunctive_normal_form_type` ([`DisjunctiveNormalFormType`]), otherwise returns `None`
        #[inline]
        pub fn as_disjunctive_normal_form_type(
            self,
        ) -> ::std::option::Option<DisjunctiveNormalFormType<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DisjunctiveNormalFormType(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `intersection_type` ([`IntersectionType`]), otherwise returns `None`
        #[inline]
        pub fn as_intersection_type(
            self,
        ) -> ::std::option::Option<IntersectionType<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::IntersectionType(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `named_type` ([`NamedType`]), otherwise returns `None`
        #[inline]
        pub fn as_named_type(self) -> ::std::option::Option<NamedType<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::NamedType(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `optional_type` ([`OptionalType`]), otherwise returns `None`
        #[inline]
        pub fn as_optional_type(self) -> ::std::option::Option<OptionalType<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::OptionalType(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `primitive_type` ([`PrimitiveType`]), otherwise returns `None`
        #[inline]
        pub fn as_primitive_type(self) -> ::std::option::Option<PrimitiveType<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::PrimitiveType(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `union_type` ([`UnionType`]), otherwise returns `None`
        #[inline]
        pub fn as_union_type(self) -> ::std::option::Option<UnionType<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::UnionType(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
    for DisjunctiveNormalFormType_IntersectionType_NamedType_OptionalType_PrimitiveType_UnionType<
        'tree,
    > {
        type WithLifetime<'a> = DisjunctiveNormalFormType_IntersectionType_NamedType_OptionalType_PrimitiveType_UnionType<
            'a,
        >;
        const KIND: &'static str = "{disjunctive_normal_form_type | intersection_type | named_type | optional_type | primitive_type | union_type}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "disjunctive_normal_form_type" => {
                    Ok(unsafe {
                        Self::DisjunctiveNormalFormType(
                            <DisjunctiveNormalFormType<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "intersection_type" => {
                    Ok(unsafe {
                        Self::IntersectionType(
                            <IntersectionType<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "named_type" => {
                    Ok(unsafe {
                        Self::NamedType(
                            <NamedType<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "optional_type" => {
                    Ok(unsafe {
                        Self::OptionalType(
                            <OptionalType<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "primitive_type" => {
                    Ok(unsafe {
                        Self::PrimitiveType(
                            <PrimitiveType<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "union_type" => {
                    Ok(unsafe {
                        Self::UnionType(
                            <UnionType<
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
                Self::DisjunctiveNormalFormType(x) => ::type_sitter::Node::raw(x),
                Self::IntersectionType(x) => ::type_sitter::Node::raw(x),
                Self::NamedType(x) => ::type_sitter::Node::raw(x),
                Self::OptionalType(x) => ::type_sitter::Node::raw(x),
                Self::PrimitiveType(x) => ::type_sitter::Node::raw(x),
                Self::UnionType(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::DisjunctiveNormalFormType(x) => ::type_sitter::Node::raw_mut(x),
                Self::IntersectionType(x) => ::type_sitter::Node::raw_mut(x),
                Self::NamedType(x) => ::type_sitter::Node::raw_mut(x),
                Self::OptionalType(x) => ::type_sitter::Node::raw_mut(x),
                Self::PrimitiveType(x) => ::type_sitter::Node::raw_mut(x),
                Self::UnionType(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::DisjunctiveNormalFormType(x) => x.into_raw(),
                Self::IntersectionType(x) => x.into_raw(),
                Self::NamedType(x) => x.into_raw(),
                Self::OptionalType(x) => x.into_raw(),
                Self::PrimitiveType(x) => x.into_raw(),
                Self::UnionType(x) => x.into_raw(),
            }
        }
    }
    /**One of `{dynamic_variable_name | expression | name | variable_name}`:
- [`DynamicVariableName`]
- [`Expression`]
- [`Name`]
- [`VariableName`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum DynamicVariableName_Expression_Name_VariableName<'tree> {
        DynamicVariableName(DynamicVariableName<'tree>),
        Expression(Expression<'tree>),
        Name(Name<'tree>),
        VariableName(VariableName<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> DynamicVariableName_Expression_Name_VariableName<'tree> {
        ///Returns the node if it is of type `dynamic_variable_name` ([`DynamicVariableName`]), otherwise returns `None`
        #[inline]
        pub fn as_dynamic_variable_name(
            self,
        ) -> ::std::option::Option<DynamicVariableName<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DynamicVariableName(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `expression` ([`Expression`]), otherwise returns `None`
        #[inline]
        pub fn as_expression(self) -> ::std::option::Option<Expression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Expression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `name` ([`Name`]), otherwise returns `None`
        #[inline]
        pub fn as_name(self) -> ::std::option::Option<Name<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Name(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `variable_name` ([`VariableName`]), otherwise returns `None`
        #[inline]
        pub fn as_variable_name(self) -> ::std::option::Option<VariableName<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::VariableName(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
    for DynamicVariableName_Expression_Name_VariableName<'tree> {
        type WithLifetime<'a> = DynamicVariableName_Expression_Name_VariableName<'a>;
        const KIND: &'static str = "{dynamic_variable_name | expression | name | variable_name}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "dynamic_variable_name" => {
                    Ok(unsafe {
                        Self::DynamicVariableName(
                            <DynamicVariableName<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "expression" => {
                    Ok(unsafe {
                        Self::Expression(
                            <Expression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "name" => {
                    Ok(unsafe {
                        Self::Name(
                            <Name<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "variable_name" => {
                    Ok(unsafe {
                        Self::VariableName(
                            <VariableName<
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
                Self::DynamicVariableName(x) => ::type_sitter::Node::raw(x),
                Self::Expression(x) => ::type_sitter::Node::raw(x),
                Self::Name(x) => ::type_sitter::Node::raw(x),
                Self::VariableName(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::DynamicVariableName(x) => ::type_sitter::Node::raw_mut(x),
                Self::Expression(x) => ::type_sitter::Node::raw_mut(x),
                Self::Name(x) => ::type_sitter::Node::raw_mut(x),
                Self::VariableName(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::DynamicVariableName(x) => x.into_raw(),
                Self::Expression(x) => x.into_raw(),
                Self::Name(x) => x.into_raw(),
                Self::VariableName(x) => x.into_raw(),
            }
        }
    }
    /**One of `{dynamic_variable_name | expression | variable_name}`:
- [`DynamicVariableName`]
- [`Expression`]
- [`VariableName`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum DynamicVariableName_Expression_VariableName<'tree> {
        DynamicVariableName(DynamicVariableName<'tree>),
        Expression(Expression<'tree>),
        VariableName(VariableName<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> DynamicVariableName_Expression_VariableName<'tree> {
        ///Returns the node if it is of type `dynamic_variable_name` ([`DynamicVariableName`]), otherwise returns `None`
        #[inline]
        pub fn as_dynamic_variable_name(
            self,
        ) -> ::std::option::Option<DynamicVariableName<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DynamicVariableName(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `expression` ([`Expression`]), otherwise returns `None`
        #[inline]
        pub fn as_expression(self) -> ::std::option::Option<Expression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Expression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `variable_name` ([`VariableName`]), otherwise returns `None`
        #[inline]
        pub fn as_variable_name(self) -> ::std::option::Option<VariableName<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::VariableName(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
    for DynamicVariableName_Expression_VariableName<'tree> {
        type WithLifetime<'a> = DynamicVariableName_Expression_VariableName<'a>;
        const KIND: &'static str = "{dynamic_variable_name | expression | variable_name}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "dynamic_variable_name" => {
                    Ok(unsafe {
                        Self::DynamicVariableName(
                            <DynamicVariableName<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "expression" => {
                    Ok(unsafe {
                        Self::Expression(
                            <Expression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "variable_name" => {
                    Ok(unsafe {
                        Self::VariableName(
                            <VariableName<
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
                Self::DynamicVariableName(x) => ::type_sitter::Node::raw(x),
                Self::Expression(x) => ::type_sitter::Node::raw(x),
                Self::VariableName(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::DynamicVariableName(x) => ::type_sitter::Node::raw_mut(x),
                Self::Expression(x) => ::type_sitter::Node::raw_mut(x),
                Self::VariableName(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::DynamicVariableName(x) => x.into_raw(),
                Self::Expression(x) => x.into_raw(),
                Self::VariableName(x) => x.into_raw(),
            }
        }
    }
    /**One of `{dynamic_variable_name | variable_name}`:
- [`DynamicVariableName`]
- [`VariableName`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum DynamicVariableName_VariableName<'tree> {
        DynamicVariableName(DynamicVariableName<'tree>),
        VariableName(VariableName<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> DynamicVariableName_VariableName<'tree> {
        ///Returns the node if it is of type `dynamic_variable_name` ([`DynamicVariableName`]), otherwise returns `None`
        #[inline]
        pub fn as_dynamic_variable_name(
            self,
        ) -> ::std::option::Option<DynamicVariableName<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DynamicVariableName(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `variable_name` ([`VariableName`]), otherwise returns `None`
        #[inline]
        pub fn as_variable_name(self) -> ::std::option::Option<VariableName<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::VariableName(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for DynamicVariableName_VariableName<'tree> {
        type WithLifetime<'a> = DynamicVariableName_VariableName<'a>;
        const KIND: &'static str = "{dynamic_variable_name | variable_name}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "dynamic_variable_name" => {
                    Ok(unsafe {
                        Self::DynamicVariableName(
                            <DynamicVariableName<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "variable_name" => {
                    Ok(unsafe {
                        Self::VariableName(
                            <VariableName<
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
                Self::DynamicVariableName(x) => ::type_sitter::Node::raw(x),
                Self::VariableName(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::DynamicVariableName(x) => ::type_sitter::Node::raw_mut(x),
                Self::VariableName(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::DynamicVariableName(x) => x.into_raw(),
                Self::VariableName(x) => x.into_raw(),
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
    /**One of `{escape_sequence | variable_name}`:
- [`EscapeSequence`]
- [`VariableName`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum EscapeSequence_VariableName<'tree> {
        EscapeSequence(EscapeSequence<'tree>),
        VariableName(VariableName<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> EscapeSequence_VariableName<'tree> {
        ///Returns the node if it is of type `escape_sequence` ([`EscapeSequence`]), otherwise returns `None`
        #[inline]
        pub fn as_escape_sequence(self) -> ::std::option::Option<EscapeSequence<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::EscapeSequence(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `variable_name` ([`VariableName`]), otherwise returns `None`
        #[inline]
        pub fn as_variable_name(self) -> ::std::option::Option<VariableName<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::VariableName(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for EscapeSequence_VariableName<'tree> {
        type WithLifetime<'a> = EscapeSequence_VariableName<'a>;
        const KIND: &'static str = "{escape_sequence | variable_name}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "escape_sequence" => {
                    Ok(unsafe {
                        Self::EscapeSequence(
                            <EscapeSequence<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "variable_name" => {
                    Ok(unsafe {
                        Self::VariableName(
                            <VariableName<
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
                Self::EscapeSequence(x) => ::type_sitter::Node::raw(x),
                Self::VariableName(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::EscapeSequence(x) => ::type_sitter::Node::raw_mut(x),
                Self::VariableName(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::EscapeSequence(x) => x.into_raw(),
                Self::VariableName(x) => x.into_raw(),
            }
        }
    }
    /**One of `{expression | variadic_unpacking}`:
- [`Expression`]
- [`VariadicUnpacking`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Expression_VariadicUnpacking<'tree> {
        Expression(Expression<'tree>),
        VariadicUnpacking(VariadicUnpacking<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Expression_VariadicUnpacking<'tree> {
        ///Returns the node if it is of type `expression` ([`Expression`]), otherwise returns `None`
        #[inline]
        pub fn as_expression(self) -> ::std::option::Option<Expression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Expression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `variadic_unpacking` ([`VariadicUnpacking`]), otherwise returns `None`
        #[inline]
        pub fn as_variadic_unpacking(
            self,
        ) -> ::std::option::Option<VariadicUnpacking<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::VariadicUnpacking(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Expression_VariadicUnpacking<'tree> {
        type WithLifetime<'a> = Expression_VariadicUnpacking<'a>;
        const KIND: &'static str = "{expression | variadic_unpacking}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "expression" => {
                    Ok(unsafe {
                        Self::Expression(
                            <Expression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "variadic_unpacking" => {
                    Ok(unsafe {
                        Self::VariadicUnpacking(
                            <VariadicUnpacking<
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
                Self::Expression(x) => ::type_sitter::Node::raw(x),
                Self::VariadicUnpacking(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Expression(x) => ::type_sitter::Node::raw_mut(x),
                Self::VariadicUnpacking(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Expression(x) => x.into_raw(),
                Self::VariadicUnpacking(x) => x.into_raw(),
            }
        }
    }
    /**One of `{intersection_type | named_type | optional_type | primitive_type}`:
- [`IntersectionType`]
- [`NamedType`]
- [`OptionalType`]
- [`PrimitiveType`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum IntersectionType_NamedType_OptionalType_PrimitiveType<'tree> {
        IntersectionType(IntersectionType<'tree>),
        NamedType(NamedType<'tree>),
        OptionalType(OptionalType<'tree>),
        PrimitiveType(PrimitiveType<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> IntersectionType_NamedType_OptionalType_PrimitiveType<'tree> {
        ///Returns the node if it is of type `intersection_type` ([`IntersectionType`]), otherwise returns `None`
        #[inline]
        pub fn as_intersection_type(
            self,
        ) -> ::std::option::Option<IntersectionType<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::IntersectionType(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `named_type` ([`NamedType`]), otherwise returns `None`
        #[inline]
        pub fn as_named_type(self) -> ::std::option::Option<NamedType<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::NamedType(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `optional_type` ([`OptionalType`]), otherwise returns `None`
        #[inline]
        pub fn as_optional_type(self) -> ::std::option::Option<OptionalType<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::OptionalType(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `primitive_type` ([`PrimitiveType`]), otherwise returns `None`
        #[inline]
        pub fn as_primitive_type(self) -> ::std::option::Option<PrimitiveType<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::PrimitiveType(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
    for IntersectionType_NamedType_OptionalType_PrimitiveType<'tree> {
        type WithLifetime<'a> = IntersectionType_NamedType_OptionalType_PrimitiveType<
            'a,
        >;
        const KIND: &'static str = "{intersection_type | named_type | optional_type | primitive_type}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "intersection_type" => {
                    Ok(unsafe {
                        Self::IntersectionType(
                            <IntersectionType<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "named_type" => {
                    Ok(unsafe {
                        Self::NamedType(
                            <NamedType<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "optional_type" => {
                    Ok(unsafe {
                        Self::OptionalType(
                            <OptionalType<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "primitive_type" => {
                    Ok(unsafe {
                        Self::PrimitiveType(
                            <PrimitiveType<
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
                Self::IntersectionType(x) => ::type_sitter::Node::raw(x),
                Self::NamedType(x) => ::type_sitter::Node::raw(x),
                Self::OptionalType(x) => ::type_sitter::Node::raw(x),
                Self::PrimitiveType(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::IntersectionType(x) => ::type_sitter::Node::raw_mut(x),
                Self::NamedType(x) => ::type_sitter::Node::raw_mut(x),
                Self::OptionalType(x) => ::type_sitter::Node::raw_mut(x),
                Self::PrimitiveType(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::IntersectionType(x) => x.into_raw(),
                Self::NamedType(x) => x.into_raw(),
                Self::OptionalType(x) => x.into_raw(),
                Self::PrimitiveType(x) => x.into_raw(),
            }
        }
    }
    /**One of `{member_access_expression | subscript_expression | variable_name}`:
- [`MemberAccessExpression`]
- [`SubscriptExpression`]
- [`VariableName`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum MemberAccessExpression_SubscriptExpression_VariableName<'tree> {
        MemberAccessExpression(MemberAccessExpression<'tree>),
        SubscriptExpression(SubscriptExpression<'tree>),
        VariableName(VariableName<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> MemberAccessExpression_SubscriptExpression_VariableName<'tree> {
        ///Returns the node if it is of type `member_access_expression` ([`MemberAccessExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_member_access_expression(
            self,
        ) -> ::std::option::Option<MemberAccessExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::MemberAccessExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `subscript_expression` ([`SubscriptExpression`]), otherwise returns `None`
        #[inline]
        pub fn as_subscript_expression(
            self,
        ) -> ::std::option::Option<SubscriptExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::SubscriptExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `variable_name` ([`VariableName`]), otherwise returns `None`
        #[inline]
        pub fn as_variable_name(self) -> ::std::option::Option<VariableName<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::VariableName(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
    for MemberAccessExpression_SubscriptExpression_VariableName<'tree> {
        type WithLifetime<'a> = MemberAccessExpression_SubscriptExpression_VariableName<
            'a,
        >;
        const KIND: &'static str = "{member_access_expression | subscript_expression | variable_name}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "member_access_expression" => {
                    Ok(unsafe {
                        Self::MemberAccessExpression(
                            <MemberAccessExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "subscript_expression" => {
                    Ok(unsafe {
                        Self::SubscriptExpression(
                            <SubscriptExpression<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "variable_name" => {
                    Ok(unsafe {
                        Self::VariableName(
                            <VariableName<
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
                Self::MemberAccessExpression(x) => ::type_sitter::Node::raw(x),
                Self::SubscriptExpression(x) => ::type_sitter::Node::raw(x),
                Self::VariableName(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::MemberAccessExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::SubscriptExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::VariableName(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::MemberAccessExpression(x) => x.into_raw(),
                Self::SubscriptExpression(x) => x.into_raw(),
                Self::VariableName(x) => x.into_raw(),
            }
        }
    }
    /**One of `{name | qualified_name | relative_name}`:
- [`Name`]
- [`QualifiedName`]
- [`RelativeName`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Name_QualifiedName_RelativeName<'tree> {
        Name(Name<'tree>),
        QualifiedName(QualifiedName<'tree>),
        RelativeName(RelativeName<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Name_QualifiedName_RelativeName<'tree> {
        ///Returns the node if it is of type `name` ([`Name`]), otherwise returns `None`
        #[inline]
        pub fn as_name(self) -> ::std::option::Option<Name<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Name(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `qualified_name` ([`QualifiedName`]), otherwise returns `None`
        #[inline]
        pub fn as_qualified_name(self) -> ::std::option::Option<QualifiedName<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::QualifiedName(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `relative_name` ([`RelativeName`]), otherwise returns `None`
        #[inline]
        pub fn as_relative_name(self) -> ::std::option::Option<RelativeName<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::RelativeName(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Name_QualifiedName_RelativeName<'tree> {
        type WithLifetime<'a> = Name_QualifiedName_RelativeName<'a>;
        const KIND: &'static str = "{name | qualified_name | relative_name}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "name" => {
                    Ok(unsafe {
                        Self::Name(
                            <Name<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "qualified_name" => {
                    Ok(unsafe {
                        Self::QualifiedName(
                            <QualifiedName<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "relative_name" => {
                    Ok(unsafe {
                        Self::RelativeName(
                            <RelativeName<
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
                Self::Name(x) => ::type_sitter::Node::raw(x),
                Self::QualifiedName(x) => ::type_sitter::Node::raw(x),
                Self::RelativeName(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Name(x) => ::type_sitter::Node::raw_mut(x),
                Self::QualifiedName(x) => ::type_sitter::Node::raw_mut(x),
                Self::RelativeName(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Name(x) => x.into_raw(),
                Self::QualifiedName(x) => x.into_raw(),
                Self::RelativeName(x) => x.into_raw(),
            }
        }
    }
    /**One of `{name | qualified_name | relative_scope}`:
- [`Name`]
- [`QualifiedName`]
- [`RelativeScope`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Name_QualifiedName_RelativeScope<'tree> {
        Name(Name<'tree>),
        QualifiedName(QualifiedName<'tree>),
        RelativeScope(RelativeScope<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Name_QualifiedName_RelativeScope<'tree> {
        ///Returns the node if it is of type `name` ([`Name`]), otherwise returns `None`
        #[inline]
        pub fn as_name(self) -> ::std::option::Option<Name<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Name(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `qualified_name` ([`QualifiedName`]), otherwise returns `None`
        #[inline]
        pub fn as_qualified_name(self) -> ::std::option::Option<QualifiedName<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::QualifiedName(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `relative_scope` ([`RelativeScope`]), otherwise returns `None`
        #[inline]
        pub fn as_relative_scope(self) -> ::std::option::Option<RelativeScope<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::RelativeScope(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Name_QualifiedName_RelativeScope<'tree> {
        type WithLifetime<'a> = Name_QualifiedName_RelativeScope<'a>;
        const KIND: &'static str = "{name | qualified_name | relative_scope}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "name" => {
                    Ok(unsafe {
                        Self::Name(
                            <Name<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "qualified_name" => {
                    Ok(unsafe {
                        Self::QualifiedName(
                            <QualifiedName<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "relative_scope" => {
                    Ok(unsafe {
                        Self::RelativeScope(
                            <RelativeScope<
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
                Self::Name(x) => ::type_sitter::Node::raw(x),
                Self::QualifiedName(x) => ::type_sitter::Node::raw(x),
                Self::RelativeScope(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Name(x) => ::type_sitter::Node::raw_mut(x),
                Self::QualifiedName(x) => ::type_sitter::Node::raw_mut(x),
                Self::RelativeScope(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Name(x) => x.into_raw(),
                Self::QualifiedName(x) => x.into_raw(),
                Self::RelativeScope(x) => x.into_raw(),
            }
        }
    }
    /**One of `{named_type | optional_type | primitive_type}`:
- [`NamedType`]
- [`OptionalType`]
- [`PrimitiveType`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum NamedType_OptionalType_PrimitiveType<'tree> {
        NamedType(NamedType<'tree>),
        OptionalType(OptionalType<'tree>),
        PrimitiveType(PrimitiveType<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> NamedType_OptionalType_PrimitiveType<'tree> {
        ///Returns the node if it is of type `named_type` ([`NamedType`]), otherwise returns `None`
        #[inline]
        pub fn as_named_type(self) -> ::std::option::Option<NamedType<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::NamedType(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `optional_type` ([`OptionalType`]), otherwise returns `None`
        #[inline]
        pub fn as_optional_type(self) -> ::std::option::Option<OptionalType<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::OptionalType(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `primitive_type` ([`PrimitiveType`]), otherwise returns `None`
        #[inline]
        pub fn as_primitive_type(self) -> ::std::option::Option<PrimitiveType<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::PrimitiveType(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
    for NamedType_OptionalType_PrimitiveType<'tree> {
        type WithLifetime<'a> = NamedType_OptionalType_PrimitiveType<'a>;
        const KIND: &'static str = "{named_type | optional_type | primitive_type}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "named_type" => {
                    Ok(unsafe {
                        Self::NamedType(
                            <NamedType<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "optional_type" => {
                    Ok(unsafe {
                        Self::OptionalType(
                            <OptionalType<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "primitive_type" => {
                    Ok(unsafe {
                        Self::PrimitiveType(
                            <PrimitiveType<
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
                Self::NamedType(x) => ::type_sitter::Node::raw(x),
                Self::OptionalType(x) => ::type_sitter::Node::raw(x),
                Self::PrimitiveType(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::NamedType(x) => ::type_sitter::Node::raw_mut(x),
                Self::OptionalType(x) => ::type_sitter::Node::raw_mut(x),
                Self::PrimitiveType(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::NamedType(x) => x.into_raw(),
                Self::OptionalType(x) => x.into_raw(),
                Self::PrimitiveType(x) => x.into_raw(),
            }
        }
    }
    /**One of `{named_type | primitive_type}`:
- [`NamedType`]
- [`PrimitiveType`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum NamedType_PrimitiveType<'tree> {
        NamedType(NamedType<'tree>),
        PrimitiveType(PrimitiveType<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> NamedType_PrimitiveType<'tree> {
        ///Returns the node if it is of type `named_type` ([`NamedType`]), otherwise returns `None`
        #[inline]
        pub fn as_named_type(self) -> ::std::option::Option<NamedType<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::NamedType(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `primitive_type` ([`PrimitiveType`]), otherwise returns `None`
        #[inline]
        pub fn as_primitive_type(self) -> ::std::option::Option<PrimitiveType<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::PrimitiveType(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for NamedType_PrimitiveType<'tree> {
        type WithLifetime<'a> = NamedType_PrimitiveType<'a>;
        const KIND: &'static str = "{named_type | primitive_type}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "named_type" => {
                    Ok(unsafe {
                        Self::NamedType(
                            <NamedType<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "primitive_type" => {
                    Ok(unsafe {
                        Self::PrimitiveType(
                            <PrimitiveType<
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
                Self::NamedType(x) => ::type_sitter::Node::raw(x),
                Self::PrimitiveType(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::NamedType(x) => ::type_sitter::Node::raw_mut(x),
                Self::PrimitiveType(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::NamedType(x) => x.into_raw(),
                Self::PrimitiveType(x) => x.into_raw(),
            }
        }
    }
    /**One of `{! | + | - | ~}`:
- [`symbols::Not`]
- [`symbols::Add`]
- [`symbols::Sub`]
- [`symbols::BitNot`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Not_Add_Sub_BitNot<'tree> {
        Not(symbols::Not<'tree>),
        Add(symbols::Add<'tree>),
        Sub(symbols::Sub<'tree>),
        BitNot(symbols::BitNot<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Not_Add_Sub_BitNot<'tree> {
        ///Returns the node if it is of type `!` ([`symbols::Not`]), otherwise returns `None`
        #[inline]
        pub fn as_not(self) -> ::std::option::Option<symbols::Not<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Not(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `+` ([`symbols::Add`]), otherwise returns `None`
        #[inline]
        pub fn as_add(self) -> ::std::option::Option<symbols::Add<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Add(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `-` ([`symbols::Sub`]), otherwise returns `None`
        #[inline]
        pub fn as_sub(self) -> ::std::option::Option<symbols::Sub<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Sub(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `~` ([`symbols::BitNot`]), otherwise returns `None`
        #[inline]
        pub fn as_bit_not(self) -> ::std::option::Option<symbols::BitNot<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::BitNot(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Not_Add_Sub_BitNot<'tree> {
        type WithLifetime<'a> = Not_Add_Sub_BitNot<'a>;
        const KIND: &'static str = "{! | + | - | ~}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "!" => {
                    Ok(unsafe {
                        Self::Not(
                            <symbols::Not<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "+" => {
                    Ok(unsafe {
                        Self::Add(
                            <symbols::Add<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "-" => {
                    Ok(unsafe {
                        Self::Sub(
                            <symbols::Sub<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "~" => {
                    Ok(unsafe {
                        Self::BitNot(
                            <symbols::BitNot<
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
                Self::Not(x) => ::type_sitter::Node::raw(x),
                Self::Add(x) => ::type_sitter::Node::raw(x),
                Self::Sub(x) => ::type_sitter::Node::raw(x),
                Self::BitNot(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Not(x) => ::type_sitter::Node::raw_mut(x),
                Self::Add(x) => ::type_sitter::Node::raw_mut(x),
                Self::Sub(x) => ::type_sitter::Node::raw_mut(x),
                Self::BitNot(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Not(x) => x.into_raw(),
                Self::Add(x) => x.into_raw(),
                Self::Sub(x) => x.into_raw(),
                Self::BitNot(x) => x.into_raw(),
            }
        }
    }
    /**One of `{php_only | { | }}`:
- [`PhpOnly`]
- [`symbols::LBrace`]
- [`symbols::RBrace`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum PhpOnly_LBrace_RBrace<'tree> {
        PhpOnly(PhpOnly<'tree>),
        LBrace(symbols::LBrace<'tree>),
        RBrace(symbols::RBrace<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> PhpOnly_LBrace_RBrace<'tree> {
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
        ///Returns the node if it is of type `{` ([`symbols::LBrace`]), otherwise returns `None`
        #[inline]
        pub fn as_l_brace(self) -> ::std::option::Option<symbols::LBrace<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::LBrace(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `}` ([`symbols::RBrace`]), otherwise returns `None`
        #[inline]
        pub fn as_r_brace(self) -> ::std::option::Option<symbols::RBrace<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::RBrace(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for PhpOnly_LBrace_RBrace<'tree> {
        type WithLifetime<'a> = PhpOnly_LBrace_RBrace<'a>;
        const KIND: &'static str = "{php_only | { | }}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "php_only" => {
                    Ok(unsafe {
                        Self::PhpOnly(
                            <PhpOnly<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "{" => {
                    Ok(unsafe {
                        Self::LBrace(
                            <symbols::LBrace<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "}" => {
                    Ok(unsafe {
                        Self::RBrace(
                            <symbols::RBrace<
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
                Self::PhpOnly(x) => ::type_sitter::Node::raw(x),
                Self::LBrace(x) => ::type_sitter::Node::raw(x),
                Self::RBrace(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::PhpOnly(x) => ::type_sitter::Node::raw_mut(x),
                Self::LBrace(x) => ::type_sitter::Node::raw_mut(x),
                Self::RBrace(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::PhpOnly(x) => x.into_raw(),
                Self::LBrace(x) => x.into_raw(),
                Self::RBrace(x) => x.into_raw(),
            }
        }
    }
    /**One of `{property_promotion_parameter | simple_parameter | variadic_parameter}`:
- [`PropertyPromotionParameter`]
- [`SimpleParameter`]
- [`VariadicParameter`]*/
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum PropertyPromotionParameter_SimpleParameter_VariadicParameter<'tree> {
        PropertyPromotionParameter(PropertyPromotionParameter<'tree>),
        SimpleParameter(SimpleParameter<'tree>),
        VariadicParameter(VariadicParameter<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> PropertyPromotionParameter_SimpleParameter_VariadicParameter<'tree> {
        ///Returns the node if it is of type `property_promotion_parameter` ([`PropertyPromotionParameter`]), otherwise returns `None`
        #[inline]
        pub fn as_property_promotion_parameter(
            self,
        ) -> ::std::option::Option<PropertyPromotionParameter<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::PropertyPromotionParameter(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `simple_parameter` ([`SimpleParameter`]), otherwise returns `None`
        #[inline]
        pub fn as_simple_parameter(
            self,
        ) -> ::std::option::Option<SimpleParameter<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::SimpleParameter(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        ///Returns the node if it is of type `variadic_parameter` ([`VariadicParameter`]), otherwise returns `None`
        #[inline]
        pub fn as_variadic_parameter(
            self,
        ) -> ::std::option::Option<VariadicParameter<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::VariadicParameter(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
        /**Get the optional field `attributes`.

This child has type `attribute_list?` ([`AttributeList`])*/
        #[inline]
        pub fn attributes(
            &self,
        ) -> ::std::option::Option<
            ::type_sitter::NodeResult<'tree, AttributeList<'tree>>,
        > {
            ::type_sitter::Node::raw(self)
                .child_by_field_name("attributes")
                .map(<AttributeList<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
        }
        /**Get the field `name`.

This child has type `{by_ref | variable_name}`:

- [`ByRef`]
- [`VariableName`]
*/
        #[inline]
        pub fn name(
            &self,
        ) -> ::type_sitter::NodeResult<'tree, anon_unions::ByRef_VariableName<'tree>> {
            ::type_sitter::Node::raw(self)
                .child_by_field_name("name")
                .map(
                    <anon_unions::ByRef_VariableName<
                        'tree,
                    > as ::type_sitter::Node<'tree>>::try_from_raw,
                )
                .expect(
                    "required child not present, there should at least be a MISSING node in its place",
                )
        }
        /**Get the optional field `type`.

This child has type `type?` ([`Type`])*/
        #[inline]
        pub fn r#type(
            &self,
        ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, Type<'tree>>> {
            ::type_sitter::Node::raw(self)
                .child_by_field_name("type")
                .map(<Type<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
    for PropertyPromotionParameter_SimpleParameter_VariadicParameter<'tree> {
        type WithLifetime<'a> = PropertyPromotionParameter_SimpleParameter_VariadicParameter<
            'a,
        >;
        const KIND: &'static str = "{property_promotion_parameter | simple_parameter | variadic_parameter}";
        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "property_promotion_parameter" => {
                    Ok(unsafe {
                        Self::PropertyPromotionParameter(
                            <PropertyPromotionParameter<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "simple_parameter" => {
                    Ok(unsafe {
                        Self::SimpleParameter(
                            <SimpleParameter<
                                'tree,
                            > as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                        )
                    })
                }
                "variadic_parameter" => {
                    Ok(unsafe {
                        Self::VariadicParameter(
                            <VariadicParameter<
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
                Self::PropertyPromotionParameter(x) => ::type_sitter::Node::raw(x),
                Self::SimpleParameter(x) => ::type_sitter::Node::raw(x),
                Self::VariadicParameter(x) => ::type_sitter::Node::raw(x),
            }
        }
        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::PropertyPromotionParameter(x) => ::type_sitter::Node::raw_mut(x),
                Self::SimpleParameter(x) => ::type_sitter::Node::raw_mut(x),
                Self::VariadicParameter(x) => ::type_sitter::Node::raw_mut(x),
            }
        }
        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::PropertyPromotionParameter(x) => x.into_raw(),
                Self::SimpleParameter(x) => x.into_raw(),
                Self::VariadicParameter(x) => x.into_raw(),
            }
        }
    }
}
