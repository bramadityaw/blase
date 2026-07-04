use std::sync::LazyLock;

use expect_test::{Expect, expect};

use crate::{
    analysis::fixture::{self, PositionFixture},
    config::Config,
    db::Severity,
};

const TEST_CONFIG: LazyLock<Config> = LazyLock::new(|| Config {
    capabilities: async_lsp::lsp_types::ClientCapabilities::default(),
    workspace_folder: camino::Utf8PathBuf::from_path_buf(std::path::PathBuf::from("/")).unwrap(),
    client_info: None,
});

fn check_diagnostic(fixture: &str, expect: Expect) {
    let (analysis, position) = fixture::optional_position(fixture);
    let fixture = PositionFixture::parse(fixture);
    assert!(position.is_none());

    let files = fixture
        .fixture
        .into_iter()
        .map(|f| f.path)
        .collect::<Vec<_>>();
    let annotations = files
        .clone()
        .into_iter()
        .flat_map(|path| {
            analysis
                .full_diagnostics(&TEST_CONFIG, &path)
                .unwrap()
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
        .collect::<Vec<_>>();

    let mut actual = String::new();
    actual.push_str("[\n");
    for (path, (range, annotation)) in annotations {
        macros::format_to!(actual, "  {} ({:?}): {},\n", path, range, annotation);
    }
    actual.push(']');

    expect.assert_eq(&actual);
}

fn check_no_diagnostic(fixture: &str) {
    let (analysis, position) = fixture::optional_position(fixture);
    let fixture = PositionFixture::parse(fixture);
    assert!(position.is_none());

    let paths = fixture
        .fixture
        .into_iter()
        .map(|f| f.path)
        .collect::<Vec<_>>();

    for path in paths.iter() {
        let ds = analysis.full_diagnostics(&TEST_CONFIG, path).unwrap();
        dbg!(path, &ds);
        assert!(ds.is_empty());
    }
}

#[test]
fn slots() {
    check_no_diagnostic(
        r#"
//- /resources/views/components/greeting.blade.php
Hi {{ $name }}

//- /resources/views/foo.blade.php
<x-greeting>
    <x-slot name="name">Brandon</x-slot>
</x-greeting>
        "#,
    );
}

#[test]
fn not_a_view() {
    check_diagnostic(
        r#"
//- /resources/views/components/greeting.blade.php
Hi {{ $name }}

//- /resources/views/index.blade.php
@include('components.greeting')
        "#,
        expect![[r#"
            [
              /resources/views/index.blade.php (9..30): warning: `components.greeting` is not a view, but a component,
            ]"#]],
    );
}

#[test]
fn include_path_traversal() {
    check_diagnostic(
        "@include('../../../etc.passwd')",
        expect![[r#"
        [
          /index.blade.php (9..30): error: view name `../../../etc.passwd` contains `..`, which is not allowed,
        ]"#]],
    );
}

#[test]
fn has_such_view() {
    check_no_diagnostic(
        r#"
//- /resources/views/view/name.blade.php
Foo

//- /resources/views/custom/admin.blade.php
Bar

//- /resources/views/admin.blade.php
Baz

//- /resources/views/index.blade.php
@include('view.name', ['status' => 'complete'])
@includeIf('view.name', ['status' => 'complete'])
@includeWhen($boolean, 'view.name', ['status' => 'complete'])

@includeUnless($boolean, 'view.name', ['status' => 'complete'])
@includeFirst(['custom.admin', 'admin'], ['status' => 'complete'])
@includeIsolated('view.name', ['user' => $user])
        "#,
    );
}

#[test]
fn no_such_view() {
    check_diagnostic(
        r#"
@include('view.name', ['status' => 'complete'])
@includeIf('view.name', ['status' => 'complete'])
@includeWhen($boolean, 'view.name', ['status' => 'complete'])

@includeUnless($boolean, 'view.name', ['status' => 'complete'])
@includeFirst(['custom.admin', 'admin'], ['status' => 'complete'])
@includeIsolated('view.name', ['user' => $user])
        "#,
        expect![[r#"
            [
              /index.blade.php (9..20): error: cannot find view `view.name` in the current workspace,
              /index.blade.php (59..70): error: cannot find view `view.name` in the current workspace,
              /index.blade.php (121..132): error: cannot find view `view.name` in the current workspace,
              /index.blade.php (186..197): error: cannot find view `view.name` in the current workspace,
              /index.blade.php (240..254): error: cannot find view `custom.admin` in the current workspace,
              /index.blade.php (256..263): error: cannot find view `admin` in the current workspace,
              /index.blade.php (309..320): error: cannot find view `view.name` in the current workspace,
            ]"#]],
    );
}

#[test]
fn no_such_component() {
    check_diagnostic(
        r#"
   <x-foo/>
"#,
        expect![[r#"
            [
              /index.blade.php (1..6): error: cannot find component `x-foo` in the current workspace,
            ]"#]],
    );
}

#[test]
fn no_such_layout() {
    check_diagnostic(
        r#"
   <x-foo-layout/>
"#,
        expect![[r#"
            [
              /index.blade.php (1..13): error: cannot find layout `x-foo-layout` in the current workspace,
            ]"#]],
    );
}

#[test]
fn syntax_error() {
    check_diagnostic(
        r#"
  @if(match) @endif
"#,
        expect![[r#"
            [
              /index.blade.php (0..9): error: Unexpected token(s): '@if (',
            ]"#]],
    );
}
