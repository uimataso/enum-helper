use enum_helper::EnumStr;

#[derive(EnumStr)]
#[enum_str(rename_all = "bogus")]
enum Foo {
    Bar,
}

fn main() {}
