use type_sitter::{HasChildren, Node, TreeCursor, UntypedNode};

pub mod blade;
pub mod php;

pub trait NodeExt<'tree>: Node<'tree> {
    fn next_sibling(&self) -> Option<UntypedNode<'tree>> {
        self.raw().next_sibling().map(UntypedNode::from)
    }

    fn prev_sibling(&self) -> Option<UntypedNode<'tree>> {
        self.raw().prev_sibling().map(UntypedNode::from)
    }

    fn untyped_children(
        &self,
        cursor: &mut TreeCursor<'tree>,
    ) -> impl Iterator<Item = UntypedNode<'tree>> {
        self.raw().children(&mut cursor.0).map(UntypedNode::from)
    }

    /// Iterate over the current node and its ancestors up until the root node
    fn ancestors(&self) -> impl Iterator<Item = UntypedNode<'tree>> {
        std::iter::successors(Some(self.upcast()), Node::parent)
    }

    /// Checks if the node is a certain implementor of Node
    ///
    /// This is perhaps only useful for UntypedNode, and
    /// should be a method on that type instead of this trait
    fn is<N: Node<'tree>>(&self) -> bool {
        self.upcast().downcast::<N>().is_ok()
    }
}

impl<'tree> NodeExt<'tree> for UntypedNode<'tree> {}

#[macro_export]
macro_rules! match_node {
    ($node:expr, {
        $( $( $path:ident )::+ ($it:pat) => $res:expr, )*
        _ => $catch_all:expr $(,)?
    }) => {{
        use ::type_sitter::Node as _;
        let __node = $node.upcast();
        $( if let Ok($it) = ::type_sitter::UntypedNode::downcast::<$($path)::+>(&__node) { $res } else )*
        { $catch_all }
    }};
}

/// Checks if the UntypedNode matches either one of the patterns
#[macro_export]
macro_rules! node_is {
    ($node:expr, $($( $type:ident )::+)|+ ) => {{
        let __node = $node;
        $( $crate::NodeExt::is::<$($type)::+>(&__node) )||+
    }};
}

impl<'tree> blade::Element<'tree> {
    pub fn tag(self) -> Option<UntypedNode<'tree>> {
        let element = self;
        let mut cursor = element.walk();
        let children = element.children(&mut cursor);
        for child in children.filter_map(Result::ok) {
            let tag = match child {
                blade::anon_unions::Anon261678758207218650704219939059354909483::SelfClosingTag(
                    self_closing_tag,
                ) => self_closing_tag.upcast(),
                blade::anon_unions::Anon261678758207218650704219939059354909483::StartTag(
                    start_tag,
                ) => start_tag.upcast(),
                _ => continue,
            };
            return Some(tag);
        }
        None
    }

    pub fn tag_name(self) -> Option<blade::TagName<'tree>> {
        let tag = self.tag()?;
        let tag_name = match_node!(tag, {
            blade::SelfClosingTag(self_tag) => {
                self_tag.tag_name()
            },
            blade::StartTag(start_tag) => {
                start_tag.tag_name()
            },
            blade::EndTag(end_tag) => {
                end_tag.tag_name()
            },
            _ => return None,
        });
        tag_name.ok()
    }
}
