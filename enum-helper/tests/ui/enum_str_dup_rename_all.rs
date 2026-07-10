use enum_helper::EnumStr;

#[derive(EnumStr)]
#[enum_str(rename_all = "lowercase")]
#[enum_str(rename_all = "UPPERCASE")]
enum Foo {
    Bar,
}

fn main() {}
