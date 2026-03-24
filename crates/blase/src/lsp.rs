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
    use async_lsp::lsp_types::{Position, Range, Url};
    use camino::Utf8PathBuf;
    use line_index::LineCol;
    use tree_sitter::Point;

    #[tracing::instrument]
    pub fn line_col(position: Position) -> LineCol {
        LineCol {
            line: position.line,
            col: position.character,
        }
    }

    pub fn utf8_path(url: &Url) -> Utf8PathBuf {
        assert_eq!(url.scheme(), "file");
        let path = url.to_file_path().unwrap();
        // Since the protocol uses a UTF-8 encoding, unwrapping is fine
        Utf8PathBuf::from_path_buf(path).unwrap()
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

    pub fn range(
        tree_sitter::Range {
            start_byte: _,
            end_byte: _,
            start_point,
            end_point,
        }: tree_sitter::Range,
    ) -> Range {
        super::into::range(start_point, end_point)
    }
}

pub mod into {
    //! Converts **into** lsp_types
    use crate::analysis::signature_help;
    use async_lsp::lsp_types::{self, Position, Range, Url};
    use camino::Utf8Path;
    use tree_sitter::Point;

    pub fn signature_help(
        help: signature_help::SignatureHelp,
        label_offsets: bool,
    ) -> lsp_types::SignatureHelp {
        let (label, params) = if label_offsets {
            let params = help
                .parameter_ranges()
                .iter()
                .map(|it| {
                    let start = help.signature[..it.start().into()]
                        .chars()
                        .map(|c| c.len_utf16())
                        .sum::<usize>() as u32;
                    let end = start
                        + help.signature[it.start().into()..it.end().into()]
                            .chars()
                            .map(|c| c.len_utf16())
                            .sum::<usize>() as u32;
                    [start, end]
                })
                .map(|label_offsets| lsp_types::ParameterInformation {
                    label: lsp_types::ParameterLabel::LabelOffsets(label_offsets),
                    documentation: None,
                })
                .collect::<Vec<_>>();
            (help.signature, params)
        } else {
            let params = help
                .parameter_labels()
                .map(|label| lsp_types::ParameterInformation {
                    label: lsp_types::ParameterLabel::Simple(label.to_owned()),
                    documentation: None,
                })
                .collect::<Vec<_>>();
            (help.signature, params)
        };
        let active_parameter = help.active_parameter.map(|it| it as u32);
        let signature = lsp_types::SignatureInformation {
            label,
            documentation: None,
            parameters: Some(params),
            active_parameter,
        };
        lsp_types::SignatureHelp {
            signatures: vec![signature],
            active_signature: Some(0),
            active_parameter,
        }
    }

    pub fn url(path: &Utf8Path) -> Url {
        Url::from_file_path(path.as_std_path()).unwrap()
    }

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
