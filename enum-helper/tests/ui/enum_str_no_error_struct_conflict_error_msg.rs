use enum_helper::EnumStr;

#[derive(EnumStr)]
#[enum_str(no_error_struct)]
#[enum_str(error_msg = "custom error")]
enum Foo {
    Bar,
}

fn main() {}
