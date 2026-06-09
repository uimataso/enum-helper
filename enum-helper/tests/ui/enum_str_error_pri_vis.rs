mod inner {
    use enum_helper::EnumStr;

    #[derive(EnumStr, PartialEq, Eq)]
    enum PriInner {
        Foo,
    }
}

fn main() {
    let _ = inner::InvalidPriInner::new("");
}
