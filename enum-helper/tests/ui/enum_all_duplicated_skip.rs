use enum_helper::EnumAll;

#[derive(EnumAll, Clone, Copy, PartialEq, Eq)]
enum Foo {
    #[enum_all(skip)]
    #[enum_all(skip)]
    Bar,
    Baz,
}

fn main() {}
