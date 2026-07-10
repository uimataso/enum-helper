use enum_helper::EnumStr;

#[derive(EnumStr)]
#[enum_str(error(disable), error_msg = "custom {input}")]
enum Foo {
    Bar,
}

fn main() {}