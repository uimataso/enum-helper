use enum_helper::EnumKind;

#[derive(EnumKind)]
#[enum_kind(no_default_derive)]
#[enum_kind(no_default_derive)]
enum Foo {
    Bar,
}

fn main() {}
