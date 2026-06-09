use enum_helper::EnumKind;

#[derive(EnumKind)]
#[enum_kind(attr(derive(Debug)))]
enum DupDefaultDerive {
    Foo,
}

fn main() {}
