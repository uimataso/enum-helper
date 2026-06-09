use enum_helper::EnumStr;

#[derive(EnumStr)]
#[enum_str(error_msg = "invalid msg {")]
enum Foo {
    Bar,
}

fn main() {}
