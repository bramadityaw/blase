use tree_sitter::Tree;

pub struct DocumentData {
    pub version: i32,
    pub data: Vec<u8>,
    pub tree: Tree,
}
