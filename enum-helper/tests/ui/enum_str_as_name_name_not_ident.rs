use enum_helper::EnumStr;

#[derive(EnumStr)]
#[enum_str(as_name(name = 123))]
enum Foo {
    Bar,
}

fn main() {}
