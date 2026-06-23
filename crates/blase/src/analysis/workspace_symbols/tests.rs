use std::sync::LazyLock;

use expect_test::{Expect, expect};

use super::*;
use crate::{analysis::fixture, config::Config};

const TEST_CONFIG: LazyLock<Config> = LazyLock::new(|| Config {
    capabilities: async_lsp::lsp_types::ClientCapabilities::default(),
    workspace_folder: camino::Utf8PathBuf::from_path_buf(std::path::PathBuf::from("/")).unwrap(),
    client_info: None,
});

fn check(fixture: &str, query: &str, expect: Expect) {
    let analysis = fixture::analysis(fixture);
    let symbols = analysis
        .workspace_symbols(&TEST_CONFIG, query.into())
        .unwrap();
    let actual = match symbols {
        Some(infos) => {
            let mut infos = infos;
            infos.sort_by_key(|info| info.name.clone());
            let mut buf = String::new();
            for info in infos {
                let kind = match info.kind {
                    SymbolKind::View => "view",
                    SymbolKind::Component => "component",
                    SymbolKind::Layout => "layout",
                };
                macros::format_to!(buf, "\n\n({}) <{}/> {}", kind, info.name, info.range.path);
            }
            buf
        }
        None => "".into(),
    };
    expect.assert_eq(&actual.trim_start());
}

#[test]
fn search_a() {
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
//- /resources/views/layouts/foo.blade.php
Boo!
//- /resources/views/components/foo.blade.php
@props(['w', 'x'])
//- /resources/views/components/bar.blade.php
@props(['y', 'z'])
//- /resources/views/components/index-table.blade.php
@props(['subject', 'rows' => [], 'columns' => []])

//- /resources/views/index.blade.php
$0
        "#,
        "a",
        expect![[r#"
            (component) <x-bar/> /resources/views/components/bar.blade.php

            (layout) <x-foo-layout/> /resources/views/layouts/foo.blade.php

            (component) <x-index-table/> /resources/views/components/index-table.blade.php"#]],
    );
}

#[test]
fn empty_workspace() {
    check(
        r#"
            Hi$0
        "#,
        "",
        expect![[""]],
    )
}
