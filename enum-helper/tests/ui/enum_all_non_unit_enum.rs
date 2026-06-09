use enum_helper::EnumAll;

#[derive(EnumAll, Clone, Copy, PartialEq, Eq)]
enum Foo {
    Bar(usize),
    Baz { x: usize },
}

fn main() {}
