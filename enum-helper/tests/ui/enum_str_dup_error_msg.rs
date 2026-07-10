use enum_helper::EnumStr;

#[derive(EnumStr)]
#[enum_str(error_msg = "a")]
#[enum_str(error_msg = "b")]
enum Foo {
    Bar,
}

fn main() {}
