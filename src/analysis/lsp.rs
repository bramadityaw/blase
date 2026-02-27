use std::sync::Arc;

use async_lsp::lsp_types::{GotoDefinitionResponse, Position, Url};
use line_index::{LineCol, LineIndex};

impl super::Analysis {
    pub fn goto_def(&self, url: Url, pos: Position) -> GotoDefinitionResponse {
        let text = Arc::clone(&self.file_contents(&url));
        let line_index = LineIndex::new(&text);
        let offset = line_index.offset(LineCol {
            line: pos.line,
            col: pos.character,
        });
        todo!()
    }
}
