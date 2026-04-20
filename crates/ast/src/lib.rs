use type_sitter::{Node, TreeCursor, UntypedNode};

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
        $( ::type_sitter::UntypedNode::downcast::<$($type)::+>(&__node).is_ok() )||+
    }};
}
