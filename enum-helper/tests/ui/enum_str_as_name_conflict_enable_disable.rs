use enum_helper::EnumStr;

#[derive(EnumStr)]
#[enum_str(as_name(enable, disable))]
enum Foo {
    Bar,
}

fn main() {}
