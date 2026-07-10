use enum_helper::EnumStr;

#[derive(EnumStr)]
enum Foo {
    #[enum_str(skip)]
    #[enum_str(skip)]
    Bar,
}

fn main() {}
