use std::sync::LazyLock;

use crate::{analysis::fixture, config::Config};

const TEST_CONFIG: LazyLock<Config> = LazyLock::new(|| Config {
    capabilities: async_lsp::lsp_types::ClientCapabilities::default(),
    workspace_folder: camino::Utf8PathBuf::from_path_buf(std::path::PathBuf::from("/")).unwrap(),
    client_info: None,
});

fn fixture(blase_fixture: &str) -> Vec<crate::db::FileRange> {
    let (analysis, position) = fixture::position(blase_fixture);
    let ranges = analysis
        .goto_def(&TEST_CONFIG, position)
        .expect("salsa cancelled");
    ranges
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
