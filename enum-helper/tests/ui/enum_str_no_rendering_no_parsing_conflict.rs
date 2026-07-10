use enum_helper::EnumStr;

#[derive(EnumStr)]
#[enum_str(no_rendering, no_parsing)]
enum Foo {
    Bar,
}

fn main() {}
