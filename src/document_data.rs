use tree_sitter::Tree;

#[derive(Debug, Clone)]
pub struct DocumentData {
    pub version: i32,
    pub data: Vec<u8>,
    pub tree: Tree,
}
