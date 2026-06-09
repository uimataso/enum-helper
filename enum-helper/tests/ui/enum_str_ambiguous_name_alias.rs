use enum_helper::EnumStr;

#[derive(EnumStr, Clone, Copy, PartialEq, Eq)]
enum Foo {
    #[enum_str(rename = "aaa")]
    Bar,
    #[enum_str(alias = "aaa")]
    Baz,
}

fn main() {}
