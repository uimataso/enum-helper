use enum_helper::EnumStr;

#[derive(EnumStr)]
#[enum_str(impl_from_str(bogus))]
enum Foo {
    Bar,
}

fn main() {}
