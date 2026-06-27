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

    for path in paths {
        let ds = analysis.full_diagnostics(&TEST_CONFIG, &path).unwrap();
        assert!(ds.is_empty(), "{}", path);
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
