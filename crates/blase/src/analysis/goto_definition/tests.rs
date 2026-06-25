use std::sync::LazyLock;

use expect_test::{Expect, expect};

use crate::{analysis::fixture, config::Config};

use super::FileRange;

const TEST_CONFIG: LazyLock<Config> = LazyLock::new(|| Config {
    capabilities: async_lsp::lsp_types::ClientCapabilities::default(),
    workspace_folder: camino::Utf8PathBuf::from_path_buf(std::path::PathBuf::from("/")).unwrap(),
    client_info: None,
});

fn check(blase_fixture: &str, expect: Expect) {
    let (analysis, position) = fixture::position(blase_fixture);
    let ranges = analysis
        .goto_def(&TEST_CONFIG, position)
        .expect("salsa cancelled");
    let mut actual = String::new();
    for FileRange { path, range } in ranges {
        macros::format_to!(actual, "{}", path);
        if !range.is_empty() {
            macros::format_to!(actual, " {:?}", range);
        }
        actual.push('\n');
    }
    expect.assert_eq(&actual);
}

fn fixture(blase_fixture: &str) -> Vec<crate::db::FileRange> {
    let (analysis, position) = fixture::position(blase_fixture);
    let ranges = analysis
        .goto_def(&TEST_CONFIG, position)
        .expect("salsa cancelled");
    ranges
}

#[test]
fn component_in_nested_directory() {
    check(
        r#"
//- /resources/views/components/icon/arrow-left.blade.php
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 640" {{ $attributes }}>
    <!--!Font Awesome Free v7.1.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2025 Fonticons, Inc.-->
    <path d="M73.4 297.4C60.9 309.9 60.9 330.2 73.4 342.7L233.4 502.7C245.9 515.2 266.2 515.2 278.7 502.7C291.2 490.2 291.2 469.9 278.7 457.4L173.3 352L544 352C561.7 352 576 337.7 576 320C576 302.3 561.7 288 544 288L173.3 288L278.7 182.6C291.2 170.1 291.2 149.8 278.7 137.3C266.2 124.8 245.9 124.8 233.4 137.3L73.4 297.3z"/>
</svg>
//- /resources/views/index.blade.php
<x-icon.arr$0ow-left/>
        "#,
        expect![[r#"
            /resources/views\components\icon\arrow-left.blade.php
        "#]],
    );
}

#[test]
fn class_component() {
    let ranges = fixture(
        r#"
//- /app/View/Components/Foo.php
<?php

class Foo extends Component
{
    public function __construct(
        $world.
    ){}
}
//- /resources/views/components/foo.blade.php
Hello {{ $world }}

//- /resources/views/index.blade.php
<x-f$0oo/>
"#,
    );
    insta::assert_json_snapshot!(ranges, @r#"
    [
      {
        "path": "/app/View/Components\\Foo.php",
        "range": "0..0"
      },
      {
        "path": "/resources/views\\components\\foo.blade.php",
        "range": "0..0"
      }
    ]
    "#);
}

#[test]
fn anon_component() {
    let ranges = fixture(
        r#"
//- /resources/views/components/foo.blade.php
@props(['x', 'y' => []])
{{--
--  This is a comment
--}}
$x

//- /resources/views/index.blade.php
<x-foo $0/>
"#,
    );
    insta::assert_json_snapshot!(ranges, @r#"
    [
      {
        "path": "/resources/views\\components\\foo.blade.php",
        "range": "0..0"
      }
    ]
    "#);
}

#[test]
fn layout_component() {
    let ranges = fixture(
        r#"
//- /resources/views/components/layout.blade.php
{{--
--  This is a comment
--}}

//- /resources/views/index.blade.php
<x-la$0yout>
        "#,
    );
    expect_test::expect![[
        r#"[FileRange { path: "/resources/views\\components\\layout.blade.php", range: 0..0 }]"#
    ]]
    .assert_eq(&format!("{:?}", ranges));

    let ranges = fixture(
        r#"
//- /resources/views/components/layout.blade.php
{{--
--  This is a comment
--}}

//- /resources/views/index.blade.php
<x-layout $0>
</x-layout>
        "#,
    );
    expect_test::expect![[
        r#"[FileRange { path: "/resources/views\\components\\layout.blade.php", range: 0..0 }]"#
    ]]
    .assert_eq(&format!("{:?}", ranges));

    let ranges = fixture(
        r#"
//- /resources/views/components/layout.blade.php
{{--
--  This is a comment
--}}

//- /resources/views/index.blade.php
<x-layout>
</x-layout $0>
            "#,
    );
    expect_test::expect![[
        r#"[FileRange { path: "/resources/views\\components\\layout.blade.php", range: 0..0 }]"#
    ]]
    .assert_eq(&format!("{:?}", ranges));
}

#[test]
fn no_response() {
    let ranges = fixture(
        r#"
//- /resources/views/components/layout.blade.php
{{--
--  This is a comment
--}}

//- /resources/views/index.blade.php
<x-layout>
$0
</x-layout>
            "#,
    );
    expect_test::expect![[r#"[]"#]].assert_eq(&format!("{:?}", ranges));
}
