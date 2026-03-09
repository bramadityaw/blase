mod nodes;

pub use nodes::*;

#[macro_export]
macro_rules! match_node {
    ($node:expr, {
        $( $( $path:ident )::+ ($it:pat) => $res:expr, )*
        _ => $catch_all:expr $(,)?
    }) => {
        $( if let Ok($it) = ::type_sitter::UntypedNode::downcast::<$($path)::+>(&$node) { $res } else )*
        { $catch_all }
    };
}
