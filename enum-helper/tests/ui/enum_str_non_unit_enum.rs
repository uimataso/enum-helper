use enum_helper::EnumStr;

#[derive(EnumStr)]
enum Foo {
    Bar(usize),
    Baz,
}

fn main() {}
