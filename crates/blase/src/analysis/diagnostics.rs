use ast::NodeExt;
use camino::Utf8Path;
use line_index::{TextRange, TextSize};
use tree_sitter::{Query, QueryCursor, StreamingIterator};
use type_sitter::{HasChildren, Node, UntypedNode};

use crate::{
    config::Config,
    db::{
        DocumentDatabase, FileRange, ParsedDocument, RootDatabase, Severity,
        def::{ComponentName, LayoutName},
    },
    resolve_path,
    util::FileType,
};

#[cfg(test)]
mod tests {
    use std::{collections::BTreeMap, sync::LazyLock};

    use itertools::Itertools;
    use line_index::{TextRange, TextSize};

    use crate::{
        analysis::fixture::PositionFixture,
        config::Config,
        db::{RootDatabase, Severity, SourceDatabase},
    };

    /// Extracts `//^^^ some text` annotations.
    ///
    /// A run of `^^^` can be arbitrary long and points to the corresponding range
    /// in the line above.
    ///
    /// The `// ^file text` syntax can be used to attach `text` to the entirety of
    /// the file.
    ///
    /// Multiline string values are supported:
    ///
    /// // ^^^ first line
    /// //   | second line
    ///
    /// Trailing whitespace is sometimes desired but usually stripped by the editor
    /// if at the end of a line, or incorrectly sized if followed by another
    /// annotation. In those cases the annotation can be explicitly ended with the
    /// `$` character.
    ///
    /// // ^^^ trailing-ws-wanted  $
    ///
    /// Annotations point to the last line that actually was long enough for the
    /// range, not counting annotations themselves. So overlapping annotations are
    /// possible:
    /// ```text
    /// // stuff        other stuff
    /// // ^^ 'st'
    /// // ^^^^^ 'stuff'
    /// //              ^^^^^^^^^^^ 'other stuff'
    /// ```
    pub fn extract_annotations(text: &str) -> Vec<(TextRange, String)> {
        let mut res = Vec::new();
        // map from line length to beginning of last line that had that length
        let mut line_start_map = BTreeMap::new();
        let mut line_start: TextSize = 0.into();
        let mut prev_line_annotations: Vec<(TextSize, usize)> = Vec::new();
        for line in text.split_inclusive('\n') {
            let mut this_line_annotations = Vec::new();
            let line_length = if let Some((prefix, suffix)) = line.split_once("//") {
                let ss_len = TextSize::of("//");
                let annotation_offset = TextSize::of(prefix) + ss_len;
                for annotation in extract_line_annotations(suffix.trim_end_matches('\n')) {
                    match annotation {
                        LineAnnotation::Annotation {
                            mut range,
                            content,
                            file,
                        } => {
                            range += annotation_offset;
                            this_line_annotations.push((range.end(), res.len()));
                            let range = if file {
                                TextRange::up_to(TextSize::of(text))
                            } else {
                                let line_start =
                                    line_start_map.range(range.end()..).next().unwrap();

                                range + line_start.1
                            };
                            res.push((range, content));
                        }
                        LineAnnotation::Continuation {
                            mut offset,
                            content,
                        } => {
                            offset += annotation_offset;
                            let &(_, idx) = prev_line_annotations
                                .iter()
                                .find(|&&(off, _idx)| off == offset)
                                .unwrap();
                            res[idx].1.push('\n');
                            res[idx].1.push_str(&content);
                            res[idx].1.push('\n');
                        }
                    }
                }
                annotation_offset
            } else {
                TextSize::of(line)
            };

            line_start_map = line_start_map.split_off(&line_length);
            line_start_map.insert(line_length, line_start);

            line_start += TextSize::of(line);

            prev_line_annotations = this_line_annotations;
        }

        res
    }

    enum LineAnnotation {
        Annotation {
            range: TextRange,
            content: String,
            file: bool,
        },
        Continuation {
            offset: TextSize,
            content: String,
        },
    }

    fn extract_line_annotations(mut line: &str) -> Vec<LineAnnotation> {
        let mut res = Vec::new();
        let mut offset: TextSize = 0.into();
        let marker: fn(char) -> bool = if line.contains('^') {
            |c| c == '^'
        } else {
            |c| c == '|'
        };
        while let Some(idx) = line.find(marker) {
            offset += TextSize::try_from(idx).unwrap();
            line = &line[idx..];

            let mut len = line.chars().take_while(|&it| it == '^').count();
            let mut continuation = false;
            if len == 0 {
                assert!(line.starts_with('|'));
                continuation = true;
                len = 1;
            }
            let range = TextRange::at(offset, len.try_into().unwrap());
            let line_no_caret = &line[len..];
            let end_marker = line_no_caret.find('$');
            let next = line_no_caret.find(marker).map_or(line.len(), |it| it + len);

            let cond = |end_marker| {
                end_marker < next
                    && (line_no_caret[end_marker + 1..].is_empty()
                        || line_no_caret[end_marker + 1..]
                            .strip_prefix(|c: char| c.is_whitespace() || c == '^')
                            .is_some())
            };
            let mut content = match end_marker {
                Some(end_marker) if cond(end_marker) => &line_no_caret[..end_marker],
                _ => line_no_caret[..next - len].trim_end(),
            };

            let mut file = false;
            if !continuation && content.starts_with("file") {
                file = true;
                content = &content["file".len()..];
            }

            let content = content.trim_start().to_owned();

            let annotation = if continuation {
                LineAnnotation::Continuation {
                    offset: range.end(),
                    content,
                }
            } else {
                LineAnnotation::Annotation {
                    range,
                    content,
                    file,
                }
            };
            res.push(annotation);

            line = &line[next..];
            offset += TextSize::try_from(next).unwrap();
        }

        res
    }

    #[test]
    fn test_extract_annotations_1() {
        let text = text_procs::trim_indent(
            r#"
fn main() {
    let (x,     y) = (9, 2);
       //^ def  ^ def
    zoo + 1
} //^^^ type:
  //  | i32

// ^file
    "#,
        );
        let res = extract_annotations(&text)
            .into_iter()
            .map(|(range, ann)| (&text[range], ann))
            .collect::<Vec<_>>();

        assert_eq!(
            res[..3],
            [
                ("x", "def".into()),
                ("y", "def".into()),
                ("zoo", "type:\ni32\n".into())
            ]
        );
        assert_eq!(res[3].0.len(), 115);
    }

    #[test]
    fn test_extract_annotations_2() {
        let text = text_procs::trim_indent(
            r#"
fn main() {
    (x,   y);
   //^ a
      //  ^ b
  //^^^^^^^^ c
}"#,
        );
        let res = extract_annotations(&text)
            .into_iter()
            .map(|(range, ann)| (&text[range], ann))
            .collect::<Vec<_>>();

        assert_eq!(
            res,
            [
                ("x", "a".into()),
                ("y", "b".into()),
                ("(x,   y)", "c".into())
            ]
        );
    }

    const TEST_CONFIG: LazyLock<Config> = LazyLock::new(|| Config {
        capabilities: async_lsp::lsp_types::ClientCapabilities::default(),
        workspace_folder: camino::Utf8PathBuf::from_path_buf(std::path::PathBuf::from("/"))
            .unwrap(),
        client_info: None,
    });

    fn check_diagnostic(fixture: &str) {
        let mut db = RootDatabase::default();
        let fixture = PositionFixture::parse(fixture);
        assert!(fixture.file_position.is_none());
        db.set_from_fixtures(fixture.fixture.clone());

        let files = fixture
            .fixture
            .into_iter()
            .map(|f| f.path)
            .collect::<Vec<_>>();
        let mut annotations = files
            .clone()
            .into_iter()
            .flat_map(|path| {
                super::full_diagnostics(&db, &TEST_CONFIG, &path)
                    .into_iter()
                    .map(|d| {
                        let mut annotation = String::new();
                        annotation += match d.severity {
                            Severity::Error => "error",
                            Severity::Warning => "warning",
                            Severity::WeakWarning => "weak",
                            Severity::Allow => "allow",
                        };
                        annotation += &format!(": {}", d.message);
                        (d.range, annotation)
                    })
            })
            .map(|(file_range, annotation)| (file_range.path, (file_range.range, annotation)))
            .into_group_map();

        for file_path in files {
            let line_index = db.line_index(&file_path).unwrap();
            let mut actual = annotations.remove(&file_path).unwrap_or_default();
            let mut expected = extract_annotations(&db.contents(&file_path).unwrap());
            actual.sort_by_key(|(range, s)| (range.start(), s.clone()));
            expected.sort_by_key(|(range, s)| (range.start(), s.clone()));

            if &expected != dbg!(&actual) {
                let fneg = expected
                    .iter()
                    .filter(|x| !actual.contains(x))
                    .map(|(range, s)| (line_index.line_col(range.start()), range, s))
                    .collect::<Vec<_>>();
                let fpos = actual
                    .iter()
                    .filter(|x| !expected.contains(x))
                    .map(|(range, s)| (line_index.line_col(range.start()), range, s))
                    .collect::<Vec<_>>();

                panic!(
                    "Diagnostic test failed.\nFalse negatives: {fneg:?}\nFalse positives: {fpos:?}"
                );
            }
        }
    }
    #[test]
    fn no_such_component() {
        check_diagnostic(
            r#"
 <x-foo/>
//^^^^^error: cannot find component `x-foo` in the current workspace
"#,
        );
    }
}

pub struct Diagnostic {
    pub message: String,
    pub range: FileRange,
    pub severity: Severity,
}

pub fn syntax_errors(db: &RootDatabase, path: &Utf8Path) -> Vec<Diagnostic> {
    let errors = db.parse_errors(path);
    errors
        .into_iter()
        .map(|e| {
            let (range, message) = match e {
                crate::db::ParseError::Missing {
                    range,
                    error,
                    grammar_name: _,
                } => (range, error),
                crate::db::ParseError::Syntax {
                    range,
                    error,
                    affected: _,
                } => (range, error),
            };
            let text_range = TextRange::new(
                TextSize::new(range.start_byte as u32),
                TextSize::new(range.end_byte as u32),
            );
            Diagnostic {
                message,
                range: FileRange {
                    path: path.to_owned(),
                    range: text_range,
                },
                severity: Severity::Error,
            }
        })
        .collect()
}

/// Request both syntax and semantic diagnostics for the given [`Utf8Path`].
pub fn full_diagnostics(db: &RootDatabase, config: &Config, path: &Utf8Path) -> Vec<Diagnostic> {
    let mut syntax_errors = syntax_errors(db, path);
    let semantic_errors = semantic_diagnostics(db, config, path);
    syntax_errors.extend(semantic_errors);
    syntax_errors
}

pub fn semantic_diagnostics(
    db: &RootDatabase,
    config: &Config,
    path: &Utf8Path,
) -> Vec<Diagnostic> {
    let mut acc = Vec::new();
    let Some(document) = db.parsed_document(path) else {
        return acc;
    };
    if document.filetype == FileType::Blade {
        no_such_component_or_layout(db, &document, config, &mut acc);
    }
    acc
}

pub struct Element<'tree>(pub ast::blade::Element<'tree>);

impl<'tree> Element<'tree> {
    pub fn tag_name(&self) -> Option<ast::blade::TagName<'tree>> {
        let mut cursor = self.0.walk();
        let children = self.0.children(&mut cursor);
        children
            .filter_map(|child| {
                let child = child.ok()?;
                match child {
                ast::blade::anon_unions::Anon213946333235361205431304157586062365302::SelfClosingTag(
                    self_closing,
                ) => self_closing.tag_name().ok(),
                ast::blade::anon_unions::Anon213946333235361205431304157586062365302::StartTag(
                    start,
                ) => start.tag_name().ok(),
                _ => None,
            }
            })
            .next()
    }
}

fn no_such_component_or_layout<'tree>(
    db: &RootDatabase,
    document: &ParsedDocument,
    config: &Config,
    acc: &mut Vec<Diagnostic>,
) {
    const COMPONENTS_AND_LAYOUTS: &'static str = r#"
((element [(start_tag (tag_name) @tag) (self_closing_tag (tag_name) @tag)]) (#match? @tag "^x-[a-z\-]+")) @component
    "#;
    let contents = document.contents(db);
    let root = document.root_node();
    let root = root.raw();
    let mut cursor = QueryCursor::new();
    let query = Query::new(&root.language(), COMPONENTS_AND_LAYOUTS).unwrap();
    let mut matches = cursor.matches(&query, *root, contents.as_bytes());
    while let Some(m) = matches.next() {
        for m in m.captures {
            let node = UntypedNode::new(m.node);
            let Ok(element) = node.downcast::<ast::blade::Element>() else {
                continue;
            };
            let Some(tag_name) = Element(element).tag_name() else {
                continue;
            };
            let name = contents.get(tag_name.byte_range());

            let range = FileRange {
                path: document.source.path(db).to_owned(),
                range: TextRange::new(
                    TextSize::new(tag_name.byte_range().start as u32),
                    TextSize::new(tag_name.byte_range().end as u32),
                ),
            };
            let severity = Severity::Error;
            if let Some(component_name) = name.and_then(ComponentName::new) {
                let message = format!(
                    "cannot find component `{}` in the current workspace",
                    component_name.tag_name()
                );

                let (class_path, resources_path) =
                    resolve_path::component_paths(component_name, config);

                if ![class_path, resources_path]
                    .iter()
                    .any(|path| db.parsed_document(path).is_some())
                {
                    acc.push(Diagnostic {
                        message,
                        range,
                        severity,
                    });
                }
                continue;
            }
            if let Some(layout_name) = name.and_then(LayoutName::new) {
                let message = format!(
                    "cannot find layout `{}` in the current workspace",
                    layout_name.tag_name()
                );

                let (class_path, resources_path) = resolve_path::layout_paths(layout_name, config);
                if ![class_path, resources_path]
                    .iter()
                    .any(|path| db.parsed_document(path).is_some())
                {
                    acc.push(Diagnostic {
                        message,
                        range,
                        severity,
                    });
                }
                continue;
            }
        }
    }
}
