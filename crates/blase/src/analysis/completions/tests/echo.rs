use expect_test::expect;

use super::*;

#[test]
fn completes_in_attr_value() {
    check(
        r#"
<span id="{$0">
"#,
        expect![[r#"
            {{  }}
            {!!  !!}"#]],
    );
    check_edit(
        "{{  }}",
        r#"
<span id="{$0">
"#,
        expect![[r#"
            <span id="{{ $0 }}">
        "#]],
    );
}

#[test]
fn completes_after_lbrace() {
    check(
        r#"
Hello {$0
"#,
        expect![[r#"
            {{  }}
            {!!  !!}"#]],
    );
    {
        cov_mark::check!(source_range_ctx_empty);
        check_edit(
            "{{  }}",
            r#"
Hello {$0
"#,
            expect![[r#"
                Hello {{ $0 }}
            "#]],
        );
    }
}

#[test]
fn completes_after_double_lbrace() {
    {
        check(
            r#"
Hello {{$0
"#,
            expect!["{{  }}"],
        );
    }
    {
        cov_mark::check!(source_range_ctx_empty);
        check_edit(
            "{{  }}",
            r#"
Hello {{$0
"#,
            expect![[r#"
                Hello {{ $0 }}
            "#]],
        );
    }
}

#[test]
fn no_complete_in_comments() {
    check(
        r#"
{{--
Some comment {$0
--}}
"#,
        expect![[""]],
    );
}

#[test]
fn no_complete_in_escaped_php_statement() {
    {
        cov_mark::check!(inside_echo_error);
        check(
            r#"
{{ {$0 }}
"#,
            expect![[""]],
        );
    }
}

#[test]
fn no_complete_in_unescaped_php_statement() {
    check(
        r#"
{!! {$0 !!}
"#,
        expect![[""]],
    );
}
