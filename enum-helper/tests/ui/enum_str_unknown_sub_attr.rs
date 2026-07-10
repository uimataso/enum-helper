use enum_helper::EnumStr;

#[derive(EnumStr)]
#[enum_str(as_name(unknown = 1))]
enum Foo {
    Bar,
}

fn main() {}