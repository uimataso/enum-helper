use enum_helper::EnumKind;

#[derive(EnumKind)]
enum Foo {
    #[enum_kind(rename = X)]
    #[enum_kind(rename = Y)]
    Bar,
}

fn main() {}
