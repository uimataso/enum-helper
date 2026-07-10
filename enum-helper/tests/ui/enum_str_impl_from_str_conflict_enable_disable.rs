use enum_helper::EnumStr;

#[derive(EnumStr)]
#[enum_str(impl_from_str(enable, disable))]
enum Foo {
    Bar,
}

fn main() {}
