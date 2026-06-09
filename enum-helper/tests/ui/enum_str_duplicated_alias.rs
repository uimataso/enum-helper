use enum_helper::EnumStr;

#[derive(EnumStr)]
#[enum_str(alias_all = "lowercase")]
#[enum_str(alias_all = "lowercase")]
enum Foo {
    #[enum_str(alias = "bar")]
    #[enum_str(alias = "bar")]
    Bar,
}

fn main() {}
