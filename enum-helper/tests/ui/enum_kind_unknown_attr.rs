use enum_helper::EnumKind;

#[derive(EnumKind)]
#[enum_kind(unknown)]
enum Foo {
    Bar,
}

fn main() {}
