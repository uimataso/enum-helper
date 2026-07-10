use enum_helper::EnumStr;

#[derive(EnumStr)]
#[enum_str(as_name(name = x, name = y))]
enum Foo {
    Bar,
}

fn main() {}
