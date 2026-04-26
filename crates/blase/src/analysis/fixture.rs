//! Utilities for creating `Analysis` instances for tests.

use camino::Utf8PathBuf;
use line_index::TextSize;

use crate::{
    analysis::{Analysis, AnalysisHost},
    db::FilePosition,
};

#[derive(Clone, Debug)]
pub struct Fixture {
    pub path: Utf8PathBuf,
    pub text: String,
}

impl AnalysisHost {
    pub fn set_from_fixtures(&mut self, fixture: Vec<Fixture>) {
        for Fixture { path, text } in fixture {
            self.set_source_file(path, &text);
        }
    }
}

const CURSOR_MARKER: &str = "$0";
const ESCAPED_CURSOR_MARKER: &str = "\\$0";

impl Fixture {
    fn parse_meta_line(meta: &str) -> Fixture {
        let meta = meta.trim();

        let mut components = meta.split_ascii_whitespace();

        let path = components
            .next()
            .expect("fixture meta must start with a path")
            .to_owned();
        assert!(
            path.starts_with('/'),
            "fixture path does not start with `/`: {path:?}"
        );

        Self {
            path: Utf8PathBuf::from_path_buf(std::path::PathBuf::from(path)).unwrap(),
            text: String::new(),
        }
    }

    pub fn parse(blase_fixture: &str) -> Vec<Fixture> {
        let fixture = text_procs::trim_indent(blase_fixture);

        let default = if fixture.contains("//- /") {
            None
        } else {
            Some("//- /index.blade.php")
        };

        let mut res = Vec::new();
        for (ix, line) in default
            .into_iter()
            .chain(fixture.split_inclusive('\n'))
            .enumerate()
        {
            if line.contains("//-") {
                assert!(
                    line.starts_with("//-"),
                    "Metadata line {ix} has invalid indentation. \
                     All metadata lines need to have the same indentation.\n\
                     The offending line: {line:?}"
                );
            }

            if let Some(line) = line.strip_prefix("//-") {
                let meta = Self::parse_meta_line(line);
                res.push(meta);
            } else {
                if matches!(line.strip_prefix("// "), Some(l) if l.trim().starts_with('/')) {
                    panic!("looks like invalid metadata line: {line:?}");
                }

                if let Some(entry) = res.last_mut() {
                    entry.text.push_str(line);
                }
            }
        }
        res
    }
}

pub struct PositionFixture {
    pub fixture: Vec<Fixture>,
    pub file_position: Option<FilePosition>,
}

/// Returns the offset of the first occurrence of `$0` marker and the copy of `text`
/// without the marker.
fn try_extract_offset(text: &str) -> Option<(TextSize, String)> {
    let cursor_pos = text.find(CURSOR_MARKER)?;
    let mut new_text = String::with_capacity(text.len() - CURSOR_MARKER.len());
    new_text.push_str(&text[..cursor_pos]);
    new_text.push_str(&text[cursor_pos + CURSOR_MARKER.len()..]);
    let cursor_pos = TextSize::from(cursor_pos as u32);
    Some((cursor_pos, new_text))
}

impl PositionFixture {
    pub fn parse(fixture: &str) -> Self {
        let mut fixture = Fixture::parse(fixture);
        let mut file_position = None;

        for entry in fixture.iter_mut() {
            let text = entry.text.as_str();
            if text.contains(CURSOR_MARKER) {
                if text.contains(ESCAPED_CURSOR_MARKER) {
                    entry.text = text.replace(ESCAPED_CURSOR_MARKER, CURSOR_MARKER);
                } else {
                    try_extract_offset(text).map(|(offset, text)| {
                        file_position = Some(FilePosition {
                            path: entry.path.clone(),
                            offset,
                        });
                        entry.text = text;
                    });
                }
            };
        }

        PositionFixture {
            fixture,
            file_position,
        }
    }
}

pub fn position(fixture: &str) -> (Analysis, FilePosition) {
    let mut host = AnalysisHost::default();
    let PositionFixture {
        fixture,
        file_position,
    } = PositionFixture::parse(fixture);
    host.set_from_fixtures(fixture);

    (
        host.analysis(),
        file_position.expect("text should contain cursor marker"),
    )
}
