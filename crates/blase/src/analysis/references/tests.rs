use std::sync::LazyLock;

use expect_test::{Expect, expect};
use itertools::Itertools;

use crate::{analysis::fixture, config::Config};

const TEST_CONFIG: LazyLock<Config> = LazyLock::new(|| Config {
    capabilities: async_lsp::lsp_types::ClientCapabilities::default(),
    workspace_folder: camino::Utf8PathBuf::from_path_buf(std::path::PathBuf::from("/")).unwrap(),
    client_info: None,
});

fn check_no_usages(fixture: &str) {
    let (analysis, position) = fixture::position(fixture);
    let refs = analysis.references(&TEST_CONFIG, position).unwrap();
    assert!(refs.is_none());
}

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

        let refs = refs.references;

        for (path, ranges) in refs.into_iter().sorted_by_key(|(path, _)| path.to_owned()) {
            let contents = analysis.contents(&path).unwrap();
            for range in ranges {
                macros::format_to!(actual, "{:?} {:?} {}\n", path, range, &contents[range]);
            }
        }
    }

    expect.assert_eq(actual.trim_start())
}

#[test]
fn no_usage() {
    check_no_usages(
        r#"
<im$0g/>
"#,
    );

    check_no_usages(
        r#"
He$0llo
"#,
    );
}

#[test]
fn find_layout_usage() {
    check(
        r#"
//- /resources/views/layout/hello.blade.php
Hello World
//- /resources/views/one.blade.php
Welcome to Laravel!
<x-h$0ello-layout/>

<x-hello-layout/>
It's so nice to see you
//- /resources/views/two.blade.php
<x-hello-layout/>
Welcome to Laravel!
"#,
        expect![[r#"
            "/resources/views/one.blade.php" 39..56 <x-hello-layout/>
            "/resources/views/two.blade.php" 0..17 <x-hello-layout/>
        "#]],
    )
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

                "/resources/views/one.blade.php" 32..42 <x-hello/>
                "/resources/views/two.blade.php" 0..10 <x-hello/>
            "#]],
    )
}
