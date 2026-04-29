//! Converts **into** lsp_types
use std::ops::Not;

use crate::{
    analysis::{
        self, Cancellable,
        completions::{self, CompletionItemKind, CompletionRelevance},
        signature_help, workspace_symbols,
    },
    config::Config,
    db::{self, FileRange, text_edit::InsertDelete},
    line_index::{LineEndings, LineIndex, PositionEncoding},
    server::ServerStateSnapshot,
};
use async_lsp::lsp_types::{self, CompletionResponse, Position, Range, Url};
use camino::Utf8Path;
use line_index::{TextRange, TextSize};
use tree_sitter::Point;

pub fn symbol_info(
    snap: &ServerStateSnapshot,
    info: workspace_symbols::SymbolInformation,
) -> Cancellable<Option<lsp_types::SymbolInformation>> {
    let workspace_symbols::SymbolInformation { name, range, kind } = info;
    let Some(location) = location(snap, range)? else {
        return Ok(None);
    };
    #[allow(deprecated)]
    let info = lsp_types::SymbolInformation {
        name,
        kind: symbol_kind(kind),
        location,
        tags: None,
        deprecated: None,
        container_name: None,
    };
    Ok(Some(info))
}

pub fn symbol_kind(kind: workspace_symbols::SymbolKind) -> lsp_types::SymbolKind {
    match kind {
        workspace_symbols::SymbolKind::View => lsp_types::SymbolKind::FILE,
        workspace_symbols::SymbolKind::Component => lsp_types::SymbolKind::FUNCTION,
        workspace_symbols::SymbolKind::Layout => lsp_types::SymbolKind::FUNCTION,
    }
}

pub fn diagnostic(line_index: &LineIndex, d: analysis::Diagnostic) -> lsp_types::Diagnostic {
    lsp_types::Diagnostic {
        range: range(line_index, d.range.range),
        severity: Some(severity(d.severity)),
        message: d.message,
        code: None,
        code_description: None,
        source: Some("blase".to_owned()),
        related_information: None,
        tags: None,
        data: None,
    }
}

pub fn severity(s: db::Severity) -> lsp_types::DiagnosticSeverity {
    match s {
        db::Severity::Error => lsp_types::DiagnosticSeverity::ERROR,
        db::Severity::Warning => lsp_types::DiagnosticSeverity::WARNING,
        db::Severity::WeakWarning => lsp_types::DiagnosticSeverity::HINT,
        db::Severity::Allow => lsp_types::DiagnosticSeverity::INFORMATION,
    }
}

pub fn completion_response(
    config: &Config,
    line_index: &LineIndex,
    tdpp: &lsp_types::TextDocumentPositionParams,
    cmps: Vec<completions::CompletionItem>,
) -> CompletionResponse {
    let max_relevance = cmps
        .iter()
        .map(|it| it.relevance.score())
        .max()
        .unwrap_or_default();

    CompletionResponse::Array(
        cmps.into_iter()
            .map(|item| completion_item(config, item, max_relevance, line_index, tdpp))
            .collect(),
    )
}

fn text_edit(line_index: &LineIndex, indel: InsertDelete) -> lsp_types::TextEdit {
    let range = range(line_index, indel.delete);
    let new_text = match line_index.endings {
        LineEndings::Unix => indel.insert,
        LineEndings::Dos => indel.insert.replace('\n', "\r\n"),
    };
    lsp_types::TextEdit { range, new_text }
}

fn completion_text_edit(
    line_index: &LineIndex,
    insert_replace_support: Option<lsp_types::Position>,
    indel: InsertDelete,
) -> lsp_types::CompletionTextEdit {
    let text_edit = text_edit(line_index, indel);
    match insert_replace_support {
        Some(cursor_pos) => lsp_types::InsertReplaceEdit {
            new_text: text_edit.new_text,
            insert: lsp_types::Range {
                start: text_edit.range.start,
                end: cursor_pos,
            },
            replace: text_edit.range,
        }
        .into(),
        None => text_edit.into(),
    }
}

fn completion_item(
    config: &Config,
    item: completions::CompletionItem,
    max_relevance: u32,
    line_index: &LineIndex,
    tdpp: &lsp_types::TextDocumentPositionParams,
) -> lsp_types::CompletionItem {
    let insert_replace_support = config.insert_replace_support().then_some(tdpp.position);

    let mut additional_text_edits = Vec::new();
    // LSP does not allow arbitrary edits in completion, so we have to do a
    // non-trivial mapping here.
    let mut text_edit = None;
    let source_range = item.source_range;
    for indel in &item.edit {
        if indel.delete.contains_range(source_range) {
            // Extract this indel as the main edit
            text_edit = Some(if indel.delete == source_range {
                self::completion_text_edit(line_index, insert_replace_support, indel.clone())
            } else {
                assert!(source_range.end() == indel.delete.end());
                let range1 = TextRange::new(indel.delete.start(), source_range.start());
                let range2 = source_range;
                let indel1 = InsertDelete::delete(range1);
                let indel2 = InsertDelete::replace(range2, indel.insert.clone());
                additional_text_edits.push(self::text_edit(line_index, indel1));
                self::completion_text_edit(line_index, insert_replace_support, indel2)
            })
        } else {
            assert!(source_range.intersect(indel.delete).is_none());
            let text_edit = self::text_edit(line_index, indel.clone());
            additional_text_edits.push(text_edit);
        }
    }

    let insert_text_format = matches!(item.kind, CompletionItemKind::Snippet)
        .then_some(lsp_types::InsertTextFormat::SNIPPET);
    let additional_text_edits = additional_text_edits
        .is_empty()
        .not()
        .then_some(additional_text_edits);

    let mut res = lsp_types::CompletionItem {
        label: item.label.clone(),
        text_edit,
        additional_text_edits,
        filter_text: Some(item.lookup().to_owned()),
        kind: Some(completion_item_kind(item.kind)),
        insert_text_format,
        ..Default::default()
    };

    set_score(&mut res, max_relevance, item.relevance);

    fn set_score(
        res: &mut lsp_types::CompletionItem,
        max_relevance: u32,
        relevance: CompletionRelevance,
    ) {
        if relevance.is_relevant() && relevance.score() == max_relevance {
            res.preselect = Some(true);
        }
        // The relevance needs to be inverted to come up with a sort score
        // because the client will sort ascending.
        let sort_score = relevance.score() ^ 0xFF_FF_FF_FF;
        // Zero pad the string to ensure values can be properly sorted
        // by the client. Hex format is used because it is easier to
        // visually compare very large values, which the sort text
        // tends to be since it is the opposite of the score.
        res.sort_text = Some(format!("{sort_score:08x}"));
    }

    res
}

fn completion_item_kind(kind: CompletionItemKind) -> lsp_types::CompletionItemKind {
    match kind {
        CompletionItemKind::Snippet => lsp_types::CompletionItemKind::SNIPPET,
    }
}

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

pub(crate) fn location(
    snap: &ServerStateSnapshot,
    frange: FileRange,
) -> Cancellable<Option<lsp_types::Location>> {
    let url = url(&frange.path);
    let Some(line_index) = snap.file_line_index(&frange.path)? else {
        return Ok(None);
    };
    let range = range(&line_index, frange.range);
    let loc = lsp_types::Location::new(url, range);
    Ok(Some(loc))
}

pub fn url(path: &Utf8Path) -> Url {
    Url::from_file_path(dbg!(path.as_std_path())).unwrap()
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
