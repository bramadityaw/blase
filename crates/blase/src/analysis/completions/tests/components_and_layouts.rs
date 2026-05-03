use expect_test::expect;

use super::*;

#[test]
fn attribute_completion_edit() {
    check_edit(
        "bar",
        r#"
//- /resources/views/components/foo.blade.php
@props(['bar', 'baz'])

//- /resources/views/index.blade.php
<x-foo b$0/>
"#,
        expect![[r#"
            <x-foo bar="$0"/>
        "#]],
    );
}

#[test]
fn attribute_completion() {
    check(
        r#"
//- /resources/views/components/foo.blade.php
@props(['x', 'y'])

//- /resources/views/index.blade.php
<x-foo $0/>
"#,
        expect![[r#"
            x
            y"#]],
    );
}
