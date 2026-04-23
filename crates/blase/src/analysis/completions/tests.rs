use std::{ops::Not, sync::LazyLock};

use expect_test::Expect;
use line_index::TextSize;

use crate::{
    analysis::{
        completions::{CompletionItem, completions},
        fixture,
    },
    config::Config,
};

mod components_and_layouts;
mod directives;
mod echo;

const TEST_CONFIG: LazyLock<Config> = LazyLock::new(|| Config {
    capabilities: async_lsp::lsp_types::ClientCapabilities::default(),
    workspace_folder: camino::Utf8PathBuf::from_path_buf(std::path::PathBuf::from("/")).unwrap(),
    client_info: None,
});

fn get_completion_items(blade_fixture: &str) -> Vec<CompletionItem> {
    let (analysis, position) = fixture::position(blade_fixture);
    let contents = analysis.contents(&position.path).unwrap();
    let offset = position.offset.into();
    let trigger = contents
        .get(offset - 1..offset)
        .and_then(|s| s.chars().next());
    let result = analysis
        .completion(&TEST_CONFIG, position, trigger)
        .unwrap()
        .unwrap_or_default();
    result.iter().for_each(|item| {
        let sr = item.source_range;
        let offset = TextSize::from(offset as u32);
        assert!(
            sr.contains_inclusive(offset),
            "source range {sr:?} does not contain the offset {:?} of the completion request: {item:?}",
            offset
        );
    });
    let mut result = result;
    result.sort_by(|a, b| a.relevance.score().cmp(&b.relevance.score()));
    result
}

pub fn completion_list(blade_fixture: &str) -> String {
    let items = get_completion_items(blade_fixture)
        .into_iter()
        .map(|i| i.label)
        .collect::<Vec<_>>();
    items.join("\n")
}

pub fn check(blade_fixture: &str, expect: Expect) {
    let actual = completion_list(blade_fixture);
    expect.assert_eq(&actual);
}

pub fn check_edit(label: &'static str, before_fixture: &str, expect: Expect) {
    let (analysis, position) = fixture::position(before_fixture);
    let contents = analysis.contents(&position.path).unwrap();
    let offset = position.offset.into();
    let trigger_char = contents
        .get(offset - 1..offset)
        .and_then(|s| s.chars().next())
        .filter(|c| c.is_alphabetic().not());
    let completions = completions(&analysis.db, &TEST_CONFIG, position, trigger_char).unwrap();
    let Some(completion) = completions.iter().find(|i| i.label == label) else {
        panic!("can't find {label:?} completion in {completions:#?}")
    };
    let mut actual = contents.to_string();
    let combined_edit = completion.edit.clone();

    combined_edit.apply(&mut actual);
    expect.assert_eq(&actual);
}

#[test]
fn no_completions_in_comments() {
    assert_eq!(
        completion_list(
            r#"
{{--
Some multi-line comment $0
--}}
"#,
        ),
        String::new(),
    );
}
