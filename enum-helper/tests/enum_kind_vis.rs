use assert2::check;
use enum_helper::EnumKind as _;

mod inner {
    use enum_helper::EnumKind;

    #[derive(EnumKind, PartialEq, Eq)]
    pub enum PubInner {
        Foo,
    }
}

#[test]
fn pub_outer() {
    check!(inner::PubInner::Foo.kind() == inner::PubInnerKind::Foo);
}
