use enum_helper::EnumStr;

#[derive(EnumStr)]
#[enum_str(no_parsing, error_msg = "custom {input}")]
enum Foo {
    Bar,
}

fn main() {}