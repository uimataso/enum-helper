use enum_helper::EnumStr;

struct NoDefault {}

#[derive(EnumStr)]
#[enum_str(default)]
enum Foo {
    Bar { x: NoDefault },
    Baz,
}

fn main() {}
