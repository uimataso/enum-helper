use enum_helper::EnumStr;

#[derive(EnumStr)]
enum Foo {
    #[enum_str(rename = "bar")]
    #[enum_str(rename = "bar")]
    Bar,
}

fn main() {}
