//! Converts **from** lsp_types
use async_lsp::lsp_types::{Position, Range, Url};
use camino::Utf8PathBuf;
use line_index::LineCol;
use tree_sitter::Point;

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
    super::into_proto::range_points(start_point, end_point)
}
