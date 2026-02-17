use std::{ops::Range, sync::Arc};

use crate::line_index::{LineEndings, LineIndex, PositionEncoding};
use async_lsp::lsp_types::{self, Position, TextDocumentContentChangeEvent};
use line_index::{LineCol, TextRange, TextSize, WideLineCol};

fn offset(line_index: &LineIndex, position: Position) -> anyhow::Result<TextSize> {
    let line_col = match line_index.encoding {
        PositionEncoding::Utf8 => LineCol {
            line: position.line,
            col: position.character,
        },
        PositionEncoding::Wide(enc) => {
            let line_col = WideLineCol {
                line: position.line,
                col: position.character,
            };
            line_index
                .index
                .to_utf8(enc, line_col)
                .ok_or_else(|| anyhow::format_err!("Invalid wide col offset"))?
        }
    };
    let line_range = line_index.index.line(line_col.line).ok_or_else(|| {
        anyhow::format_err!(
            "Invalid offset {line_col:?} (line index length: {:?})",
            line_index.index.len()
        )
    })?;
    let col = TextSize::from(line_col.col);
    let clamped_len = col.min(line_range.len());
    Ok(line_range.start() + clamped_len)
}

fn text_range(line_index: &LineIndex, range: lsp_types::Range) -> anyhow::Result<TextRange> {
    let start = offset(line_index, range.start)?;
    let end = offset(line_index, range.end)?;
    match end < start {
        true => Err(anyhow::format_err!("Invalid Range")),
        false => Ok(TextRange::new(start, end)),
    }
}

pub fn apply_document_changes(
    encoding: PositionEncoding,
    file_contents: &str,
    mut content_changes: Vec<TextDocumentContentChangeEvent>,
) -> String {
    use line_index as idx;
    use std::mem;

    // If at least one of the changes is a full document change, use the last
    // of them as the starting point and ignore all previous changes.
    let (mut text, content_changes) = match content_changes
        .iter()
        .rposition(|change| change.range.is_none())
    {
        Some(idx) => {
            let text = mem::take(&mut content_changes[idx].text);
            (text, &content_changes[idx + 1..])
        }
        None => (file_contents.to_owned(), &content_changes[..]),
    };

    if content_changes.is_empty() {
        return text;
    }

    let mut line_index = LineIndex {
        // the index will be overwritten in the bottom loop's first iteration
        index: Arc::new(idx::LineIndex::new(&text)),
        // We don't care about line endings here.
        endings: LineEndings::Unix,
        encoding,
    };

    // The changes we got must be applied sequentially, but can cross lines so we
    // have to keep our line index updated.
    // Some clients (e.g. Code) sort the ranges in reverse. As an optimization, we
    // remember the last valid line in the index and only rebuild it if needed.
    // The VFS will normalize the end of lines to `\n`.
    let mut index_valid = !0u32;
    for change in content_changes {
        // The None case can't happen as we have handled it above already
        if let Some(range) = change.range {
            if index_valid <= range.end.line {
                *Arc::make_mut(&mut line_index.index) = idx::LineIndex::new(&text);
            }
            index_valid = range.start.line;
            if let Ok(range) = text_range(&line_index, range) {
                text.replace_range(Range::<usize>::from(range), &change.text);
            }
        }
    }

    text
}

#[cfg(test)]
mod tests {
    use line_index::WideEncoding;

    use super::*;

    #[test]
    fn test_apply_document_changes() {
        macro_rules! c {
            [$($sl:expr, $sc:expr; $el:expr, $ec:expr => $text:expr),+] => {
                vec![$(TextDocumentContentChangeEvent {
                    range: Some(lsp_types::Range {
                        start: Position { line: $sl, character: $sc },
                        end: Position { line: $el, character: $ec },
                    }),
                    range_length: None,
                    text: String::from($text),
                }),+]
            };
        }

        let encoding = PositionEncoding::Wide(WideEncoding::Utf16);
        let text = apply_document_changes(encoding, "", vec![]);
        assert_eq!(text, "");
        let text = apply_document_changes(
            encoding,
            &text,
            vec![TextDocumentContentChangeEvent {
                range: None,
                range_length: None,
                text: String::from("the"),
            }],
        );
        assert_eq!(text, "the");
        let text = apply_document_changes(encoding, &text, c![0, 3; 0, 3 => " quick"]);
        assert_eq!(text, "the quick");
        let text = apply_document_changes(
            encoding,
            &text,
            c![0, 0; 0, 4 => "", 0, 5; 0, 5 => " foxes"],
        );
        assert_eq!(text, "quick foxes");
        let text = apply_document_changes(encoding, &text, c![0, 11; 0, 11 => "\ndream"]);
        assert_eq!(text, "quick foxes\ndream");
        let text = apply_document_changes(encoding, &text, c![1, 0; 1, 0 => "have "]);
        assert_eq!(text, "quick foxes\nhave dream");
        let text = apply_document_changes(
            encoding,
            &text,
            c![0, 0; 0, 0 => "the ", 1, 4; 1, 4 => " quiet", 1, 16; 1, 16 => "s\n"],
        );
        assert_eq!(text, "the quick foxes\nhave quiet dreams\n");
        let text = apply_document_changes(
            encoding,
            &text,
            c![0, 15; 0, 15 => "\n", 2, 17; 2, 17 => "\n"],
        );
        assert_eq!(text, "the quick foxes\n\nhave quiet dreams\n\n");
        let text = apply_document_changes(
            encoding,
            &text,
            c![1, 0; 1, 0 => "DREAM", 2, 0; 2, 0 => "they ", 3, 0; 3, 0 => "DON'T THEY?"],
        );
        assert_eq!(
            text,
            "the quick foxes\nDREAM\nthey have quiet dreams\nDON'T THEY?\n"
        );
        let text =
            apply_document_changes(encoding, &text, c![0, 10; 1, 5 => "", 2, 0; 2, 12 => ""]);
        assert_eq!(text, "the quick \nthey have quiet dreams\n");

        let text = String::from("❤️");
        let text = apply_document_changes(encoding, &text, c![0, 0; 0, 0 => "a"]);
        assert_eq!(text, "a❤️");

        let text = String::from("a\nb");
        let text =
            apply_document_changes(encoding, &text, c![0, 1; 1, 0 => "\nțc", 0, 1; 1, 1 => "d"]);
        assert_eq!(text, "adcb");

        let text = String::from("a\nb");
        let text =
            apply_document_changes(encoding, &text, c![0, 1; 1, 0 => "ț\nc", 0, 2; 0, 2 => "c"]);
        assert_eq!(text, "ațc\ncb");
    }
}
