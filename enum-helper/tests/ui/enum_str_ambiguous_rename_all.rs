use enum_helper::EnumStr;

#[derive(EnumStr, Clone, Copy, PartialEq, Eq)]
#[enum_str(rename_all = "lowercase")]
enum Foo {
    FooBar,
    Foobar,
}

fn main() {}
