use std::sync::LazyLock;

use expect_test::{Expect, expect};

use crate::{analysis::fixture, config::Config};

const TEST_CONFIG: LazyLock<Config> = LazyLock::new(|| Config {
    capabilities: async_lsp::lsp_types::ClientCapabilities::default(),
    workspace_folder: camino::Utf8PathBuf::from_path_buf(std::path::PathBuf::from("/")).unwrap(),
    client_info: None,
});

fn check_no_hover(blade_fixture: &str) {
    let (analysis, position) = fixture::position(blade_fixture);
    let hover = analysis
        .hover(&TEST_CONFIG, position)
        .expect("Query cancelled");
    assert!(hover.is_none(), "Expected no hover response");
}

fn check(blade_fixture: &str, expect: Expect) {
    let (analysis, position) = fixture::position(blade_fixture);
    let contents = analysis.contents(&position.path).unwrap();
    let hover = analysis
        .hover(&TEST_CONFIG, position)
        .expect("Query cancelled")
        .expect("Expected hover response");
    let hovered_element = &contents[hover.range];
    let actual = format!("*{hovered_element}*\n{}\n", hover.markup);
    expect.assert_eq(&actual);
}

#[test]
fn no_hover_elsewhere() {
    check_no_hover(
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
<h1>Hi$0</h1>
"#,
    );
}

#[test]
fn test_hover_on_class_layout() {
    check(
        r#"
//- /app/View/Components/FooLayout.php
<?php

class FooLayout extends Component
{
    /**
     * Get the view / contents that represents the component.
     */
    public function render(): View
    {
        return view('layouts.foo');
    }
}
//- /resources/views/components/layouts/foo.blade.php
Hello!

//- /resources/views/index.blade.php
<x-f$0oo-layout>
"#,
        expect![[r#"
            *x-foo-layout*
            *Project Path*: app\View\Components\FooLayout.php
            ___
            ```blade
            <x-foo-layout>
              {{ $slot }}
            </x-foo-layout>
            ```
        "#]],
    );
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
            *Project Path*: app\View\Components\Foo.php
            ___
            ```blade
            <x-foo world="">
            ```
        "#]],
    );
}

#[test]
fn test_hover_on_anon_component_with_no_documentation() {
    check(
        r#"
//- /resources/views/components/foo.blade.php
@props(['x', 'y' => []])
{{--
--  This is a comment
--}}
$x

//- /resources/views/index.blade.php
<x-f$0oo/>
"#,
        expect![[r#"
            *x-foo*
            *Project Path*: resources\views\components\foo.blade.php
            ___
            ```blade
            <x-foo x="" y="[]">
            ```
        "#]],
    );
}

#[test]
fn test_hover_on_anon_component_with_documentation() {
    check(
        r#"
//- /resources/views/components/foo.blade.php
{{--
--  This is a comment
--}}
@props(['x', 'y' => []])
$x

//- /resources/views/index.blade.php
<x-f$0oo/>
"#,
        expect![[r#"
            *x-foo*
            *Project Path*: resources\views\components\foo.blade.php
            ___
            ```blade
            <x-foo x="" y="[]">
            ```
            This is a comment
        "#]],
    );
}

#[test]
fn test_hover_on_anon_layout_with_documentation() {
    check(
        r#"
//- /resources/views/layouts/foo.blade.php
{{--
--  This is a comment
--}}
Hi

//- /resources/views/index.blade.php
<x-f$0oo-layout/>
"#,
        expect![[r#"
            *x-foo-layout*
            *Project Path*: resources\views\layouts\foo.blade.php
            ___
            ```blade
            <x-foo-layout>
              {{ $slot }}
            </x-foo-layout>
            ```
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
            *Project Path*: resources\views\components\foo.blade.php
            ___
            ```blade
            <x-foo x="" y="[]">
            ```
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
            *Project Path*: resources\views\components\foo.blade.php
            ___
            ```blade
            <x-foo>
            ```
        "#]],
    );
}

#[test]
fn test_hover_on_anon_layout() {
    check(
        r#"
//- /resources/views/layouts/foo.blade.php
test

//- /resources/views/index.blade.php
<x-f$0oo-layout>
"#,
        expect![[r#"
            *x-foo-layout*
            *Project Path*: resources\views\layouts\foo.blade.php
            ___
            ```blade
            <x-foo-layout>
              {{ $slot }}
            </x-foo-layout>
            ```
        "#]],
    );
}
