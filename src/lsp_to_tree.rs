use tower_lsp::lsp_types;

pub fn point(lsp_pos: lsp_types::Position) -> tree_sitter::Point {
    let lsp_types::Position { line, character } = lsp_pos;
    tree_sitter::Point {
        row: line as usize,
        column: character as usize,
    }
}
