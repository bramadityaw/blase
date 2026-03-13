pub mod blade;
pub mod php;

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
