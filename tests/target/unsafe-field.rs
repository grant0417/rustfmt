#![feature(unsafe_fields)]

struct Foo {
    unsafe field: (),
}

enum Bar {
    Variant { unsafe field: () },
}

union Baz {
    unsafe field: (),
}