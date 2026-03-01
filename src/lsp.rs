use async_lsp::{
    LanguageClient,
    lsp_types::{Diagnostic, PublishDiagnosticsParams, Url},
};

use crate::server::ServerState;

impl ServerState {
    pub fn publish_diagnostics(
        &mut self,
        uri: Url,
        diagnostics: Vec<Diagnostic>,
        version: Option<i32>,
    ) {
        if let Err(e) = self.client.publish_diagnostics(PublishDiagnosticsParams {
            uri,
            diagnostics,
            version,
        }) {
            tracing::error!("Failed to publish diagnostics: {e}");
        };
    }
}

pub mod from {
    //! Converts **from** lsp_types
    use async_lsp::lsp_types::{Position, Range};
    use line_index::LineCol;
    use tree_sitter::Point;

    #[tracing::instrument]
    pub fn line_col(position: Position) -> LineCol {
        LineCol {
            line: position.line,
            col: position.character,
        }
    }

    pub fn points(Range { start, end }: Range) -> (Point, Point) {
        let start = Point {
            row: start.line as usize,
            column: start.character as usize,
        };
        let end = Point {
            row: end.line as usize,
            column: end.character as usize,
        };
        (start, end)
    }
}

pub mod into {
    //! Converts **into** lsp_types
    use async_lsp::lsp_types::{Position, Range};
    use tree_sitter::Point;

    pub fn range(start: Point, end: Point) -> Range {
        let start = Position {
            line: start.row as u32,
            character: start.column as u32,
        };
        let end = Position {
            line: end.row as u32,
            character: end.column as u32,
        };
        Range { start, end }
    }
}
