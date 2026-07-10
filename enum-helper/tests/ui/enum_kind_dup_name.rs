use enum_helper::EnumKind;

#[derive(EnumKind)]
#[enum_kind(name = A)]
#[enum_kind(name = B)]
enum Foo {
    Bar,
}

fn main() {}
