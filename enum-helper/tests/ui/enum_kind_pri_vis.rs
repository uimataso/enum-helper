mod inner {
    use enum_helper::EnumKind;

    #[derive(EnumKind, PartialEq, Eq)]
    enum PriInner {
        Foo,
    }
}

fn main() {
    let _ = inner::PriInnerKind::Foo;
}
