use expect_test::expect;

use super::*;

#[test]
fn component_completion() {
    check(
        r#"
//- /resources/views/components/foo.blade.php
@props(['w', 'x'])
//- /resources/views/components/bar.blade.php
@props(['y', 'z'])

//- /resources/views/index.blade.php
f$0
"#,
        expect!["x-foo"],
    );
}

#[test]
fn component_completion_edit() {
    check_edit(
        "x-foo",
        r#"
//- /resources/views/components/foo.blade.php
@props(['w', 'x'])
//- /resources/views/components/bar.blade.php
@props(['y', 'z'])

//- /resources/views/index.blade.php
f$0
"#,
        expect![[r#"
            <x-foo w="1" x="0">
        "#]],
    );
}

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

#[test]
fn no_attribute_completion_in_end_tag() {
    check(
        r#"
//- /resources/views/components/foo.blade.php
@props(['x', 'y'])

//- /resources/views/index.blade.php
<x-foo>
</x-foo $0>
"#,
        expect![[""]],
    );
}
