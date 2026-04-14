use std::sync::LazyLock;

use expect_test::{Expect, expect};

use crate::{analysis::fixture, config::Config};

const TEST_CONFIG: LazyLock<Config> = LazyLock::new(|| Config {
    capabilities: async_lsp::lsp_types::ClientCapabilities::default(),
    workspace_folder: camino::Utf8PathBuf::from_path_buf(std::path::PathBuf::from("/")).unwrap(),
});

fn check(blade_fixture: &str, expect: Expect) {
    let (analysis, position) = fixture::position(blade_fixture);
    let contents = analysis.contents(&position.path).unwrap().unwrap();
    let hover = analysis
        .hover(&TEST_CONFIG, position)
        .expect("Query cancelled")
        .expect("Expected hover response");
    let hovered_element = &contents[hover.range];
    let actual = format!("*{hovered_element}*\n{}\n", hover.markup);
    expect.assert_eq(&actual);
}

#[test]
fn test_hover_on_class_component() {
    check(
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
<x-f$0oo>
"#,
        expect![[r#"
            *x-foo*
            ```blade
            <x-foo world="">
            ```
            ---
            *Project Path*: app\View\Components\Foo.php

        "#]],
    );
}

#[test]
fn test_hover_on_anon_component_with_props() {
    check(
        r#"
//- /resources/views/components/foo.blade.php
@props(['x', 'y' => []])
$x

//- /resources/views/index.blade.php
<x-f$0oo>
"#,
        expect![[r#"
            *x-foo*
            ```blade
            <x-foo x="" y="[]">
            ```
            ---
            *Project Path*: resources\views\components\foo.blade.php

        "#]],
    );
}

#[test]
fn test_hover_on_anon_component() {
    check(
        r#"
//- /resources/views/components/foo.blade.php
test

//- /resources/views/index.blade.php
<x-f$0oo>
"#,
        expect![[r#"
            *x-foo*
            ```blade
            <x-foo>
            ```
            ---
            *Project Path*: resources\views\components\foo.blade.php

        "#]],
    );
}
