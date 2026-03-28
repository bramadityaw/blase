//! Converts **into** lsp_types
use crate::{
    analysis::{Cancellable, signature_help},
    line_index::{LineIndex, PositionEncoding},
};
use async_lsp::lsp_types::{self, Position, Range, Url};
use camino::Utf8Path;
use line_index::{TextRange, TextSize};
use tree_sitter::Point;

pub fn cancellable<R>(result: Cancellable<R>) -> Result<R, async_lsp::ResponseError> {
    result.map_err(|err| {
            let message = "Salsa query cancelled: ".to_string();
            let message = message
                + match err {
                    salsa::Cancelled::Local => {
                        "The query was operating but the local database execution has been cancelled."
                    }
                    salsa::Cancelled::PendingWrite => "The query was operating on revision R, but there is a pending write to move to revision R+1.",
                    salsa::Cancelled::PropagatedPanic => "The query was blocked on another thread, and that thread panicked.",
                    _ => "unknown",
                };
            async_lsp::ResponseError::new(async_lsp::ErrorCode::INTERNAL_ERROR, message)
        })
}

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

pub(crate) fn position(line_index: &LineIndex, offset: TextSize) -> lsp_types::Position {
    let line_col = line_index.index.line_col(offset);
    match line_index.encoding {
        PositionEncoding::Utf8 => lsp_types::Position::new(line_col.line, line_col.col),
        PositionEncoding::Wide(enc) => {
            let line_col = line_index.index.to_wide(enc, line_col).unwrap();
            lsp_types::Position::new(line_col.line, line_col.col)
        }
    }
}

pub(crate) fn range(line_index: &LineIndex, range: TextRange) -> lsp_types::Range {
    let start = position(line_index, range.start());
    let end = position(line_index, range.end());
    lsp_types::Range::new(start, end)
}

pub fn range_points(start: Point, end: Point) -> Range {
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
