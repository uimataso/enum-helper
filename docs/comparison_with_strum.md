# Compare to `strum`

Why not just use `strum`?

[`strum`](https://github.com/Peternator7/strum) is a fantastic crate created by [Peter Glotfelty](https://github.com/Peternator7), it has everything you need to work with enums.

But the main reason I wanted another crate is that customizing the error message for parse errors in `strum` requires writing a lot of boilerplate:

```rust
use std::fmt;
use strum::{EnumString, VariantNames};

#[derive(Debug, Clone, EnumString, VariantNames)]
#[strum(parse_err_fn = foo_err_fn, parse_err_ty = InvalidFoo)]
enum Foo {
    Bar,
    Baz,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct InvalidFoo;

impl fmt::Display for InvalidFoo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "invalid foo, expected one of {:?}", Foo::VARIANTS)
    }
}

impl std::error::Error for InvalidFoo {}

fn foo_err_fn(_: &str) -> InvalidFoo {
    InvalidFoo {}
}
```

That's a lot of code just to customize an error message. And I need a lot of them.

Of course, I could just write a simple `macro_rules`, or try to push a change upstream, but I also think it will be cool to try to write a proc macro by my own.

This crate also has a different taste on the API.

For example, instead of deriving these separately: `#[derive(EnumString, AsRefStr, IntoStaticStr)]`, I just want one `EnumStr`.
