use std::collections::HashMap;

use camino::Utf8PathBuf;
use line_index::{TextRange, TextSize};
use tree_sitter::{Query, QueryCursor, StreamingIterator};
use type_sitter::{Node, UntypedNode};

use crate::{
    config::Config,
    db::{
        DocumentDatabase, FilePosition, ParsedDocument, RootDatabase,
        def::{ComponentName, LayoutName, queries},
    },
    resolve_path,
    util::FileType,
};

pub struct ReferenceSearchResult {
    /// If Some, it stores the file paths of the document that defines the component/layout
    /// If None, it is a builtin directive
    pub defined_files: Option<Box<[Utf8PathBuf]>>,

    /// All references found, grouped by file
    pub references: HashMap<Utf8PathBuf, Vec<TextRange>>,
}

pub fn references(
    db: &RootDatabase,
    config: &Config,
    position: FilePosition,
) -> Option<Vec<ReferenceSearchResult>> {
    let path = &position.path;
    let offset = position.offset;
    let document = db.parsed_document(path)?;
    let current_node = document.get_node_at(offset)?;
    ast::match_node!(current_node, {
        ast::blade::TagName(tag_name) => handle_component_or_layout_references(db, config, tag_name, &document),
        _ => None,
    })
}

pub fn handle_component_or_layout_references(
    db: &RootDatabase,
    config: &Config,
    tag_name: ast::blade::TagName<'_>,
    document: &ParsedDocument,
) -> Option<Vec<ReferenceSearchResult>> {
    let name = document.text_for_node(db, tag_name)?;
    let (name, (class_path, resources_path)) = match ComponentName::new(name) {
        Some(name) => (
            &name.tag_name(),
            resolve_path::component_paths(&name, config),
        ),
        None => {
            let name = LayoutName::new(name)?;
            (&name.tag_name(), resolve_path::layout_paths(&name, config))
        }
    };
    let doc_paths = [class_path, resources_path]
        .into_iter()
        .filter(|path| db.parsed_document(path).is_some())
        .collect::<Box<_>>();
    let tag_name = tag_name.raw();
    let query = Query::new(
        &tag_name.language(),
        &queries::component_and_layout_named(name),
    )
    .unwrap();
    let references = db
        .all_documents()
        .into_iter()
        .filter(|doc| doc.filetype == FileType::Blade)
        .flat_map(|doc| {
            let node = doc.root_node();
            let contents = doc.contents(db);
            let mut cursor = QueryCursor::new();
            let mut matches = cursor.matches(&query, *node.raw(), contents.as_bytes());

            let mut captures = Vec::new();

            while let Some(m) = matches.next() {
                captures.push(m.captures.to_owned());
            }

            captures
                .into_iter()
                .map(|captures| (captures, doc.source.path(db)))
        })
        .map(|(captures, path)| {
            let ranges = captures
                .iter()
                .filter_map(|capture| {
                    let node = UntypedNode::new(capture.node)
                        .downcast::<ast::blade::Element>()
                        .ok()?;
                    let range = TextRange::new(
                        TextSize::new(node.byte_range().start as u32),
                        TextSize::new(node.byte_range().end as u32),
                    );
                    Some(range)
                })
                .collect::<Vec<_>>();
            (path.to_owned(), ranges)
        })
        .collect::<HashMap<_, _>>();
    Some(vec![ReferenceSearchResult {
        defined_files: Some(doc_paths),
        references,
    }])
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use expect_test::{Expect, expect};

    use crate::{analysis::fixture, config::Config};

    const TEST_CONFIG: LazyLock<Config> = LazyLock::new(|| Config {
        capabilities: async_lsp::lsp_types::ClientCapabilities::default(),
        workspace_folder: camino::Utf8PathBuf::from_path_buf(std::path::PathBuf::from("/"))
            .unwrap(),
        client_info: None,
    });

    fn check(fixture: &str, expect: Expect) {
        let (analysis, position) = fixture::position(fixture);
        let mut refs = analysis
            .references(&TEST_CONFIG, position)
            .unwrap()
            .unwrap();

        refs.sort_by_key(|refs| {
            refs.defined_files
                .clone()
                .and_then(|path| path.first().cloned())
        });

        let mut actual = String::new();
        for refs in refs {
            actual += "\n\n";

            if refs.references.is_empty() {
                actual += "(no references)\n";
                continue;
            }

            if let Some(files) = refs.defined_files {
                for file in files {
                    actual += file.as_str();
                }
                actual += "\n\n";
            }

            for (path, mut ranges) in refs.references {
                let contents = analysis.contents(&path).unwrap();
                ranges.sort_by_key(|range| range.start());
                for range in ranges {
                    macros::format_to!(actual, "{:?} {:?} {}\n", path, range, &contents[range]);
                }
            }
        }

        expect.assert_eq(actual.trim_start())
    }

    #[test]
    fn find_component_usage() {
        check(
            r#"
//- /resources/views/components/hello.blade.php
Hello World
//- /resources/views/one.blade.php
Welcome to Laravel!
<x-h$0ello/>

<x-hello/>
It's so nice to see you
//- /resources/views/two.blade.php
<x-hello/>
Welcome to Laravel!
"#,
            //FIXME: Sometimes, the test outputs change every run. However, the only
            //thing changing is the ordering of the references list (sometimes ascending or descending).
            //
            // I've sorted the references list by the defined_files field, but I'll leave the comment above
            // just in case this comes up again.
            expect![[r#"
                /resources/views\components\hello.blade.php

                "/resources/views/two.blade.php" 0..10 <x-hello/>
                "/resources/views/one.blade.php" 32..42 <x-hello/>
            "#]],
        )
    }
}
