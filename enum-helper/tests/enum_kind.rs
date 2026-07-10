use assert2::check;
use enum_helper::EnumKind;

#[derive(EnumKind, PartialEq, Eq)]
enum BasicCase {
    Foo { x: usize },
    Bar(String),
    Baz,
}

#[test]
fn basic_case() {
    check!(BasicCase::Foo { x: 1 }.kind() == BasicCaseKind::Foo);
    check!(BasicCase::Bar(String::new()).kind() == BasicCaseKind::Bar);
    check!(BasicCase::Baz.kind() == BasicCaseKind::Baz);
}

#[derive(EnumKind, PartialEq, Eq)]
#[enum_kind(attr(derive(Default)))]
#[allow(dead_code)]
enum Attr {
    Foo {
        x: usize,
    },
    Bar(String),
    #[enum_kind(attr(default))]
    Baz,
}

#[test]
fn attr() {
    check!(Attr::Foo { x: 0 }.kind() == AttrKind::Foo);
    check!(Attr::Bar(String::new()).kind() == AttrKind::Bar);
    check!(Attr::Baz.kind() == AttrKind::Baz);
    check!(AttrKind::default() == AttrKind::Baz);
}

#[derive(EnumKind, PartialEq, Eq)]
#[enum_kind(attr(derive(Hash)), attr(derive(Default)))]
#[allow(dead_code)]
enum AttrsInOnePath {
    #[enum_kind(attr(default))]
    Foo,
}

#[test]
fn attrs_in_one_path() {
    check!(AttrsInOnePathKind::Foo == AttrsInOnePathKind::Foo);
    check!(AttrsInOnePathKind::Foo == AttrsInOnePathKind::default());
}

#[derive(EnumKind, PartialEq, Eq)]
#[enum_kind(name = KindRename)]
enum Rename {
    Foo {
        x: usize,
    },
    #[enum_kind(rename = Barrrr)]
    Bar,
}

#[test]
fn rename() {
    check!(Rename::Foo { x: 1 }.kind() == KindRename::Foo);
    check!(Rename::Bar.kind() == KindRename::Barrrr);
}

#[derive(EnumKind)]
enum Generic<T> {
    Foo(T),
    Bar,
}

#[test]
fn generic() {
    check!(Generic::<i32>::Foo(1).kind() == GenericKind::Foo);
    check!(Generic::<i32>::Bar.kind() == GenericKind::Bar);
}

#[derive(EnumKind)]
#[enum_kind(no_default_derive, attr(derive(Debug)))]
enum NoDefaultDerive {
    Foo,
}

#[test]
fn no_default_derive() {
    check!(format!("{:?}", NoDefaultDerive::Foo.kind()) == "Foo");
}
