use assert2::check;
use enum_helper::EnumAll;

#[derive(EnumAll, PartialEq, Eq)]
enum BasicCase {
    Foo,
    Bar,
    Baz,
}

#[test]
fn basic_case() {
    check!(BasicCase::ALL == [BasicCase::Foo, BasicCase::Bar, BasicCase::Baz]);
}

#[derive(EnumAll, PartialEq, Eq)]
enum Empty {}

#[test]
fn empty() {
    check!(Empty::ALL == []);
}

#[derive(EnumAll, PartialEq, Eq)]
#[allow(dead_code)]
enum Skip {
    #[enum_all(skip)]
    Foo,
    Bar,
    Baz,
}

#[test]
fn skip() {
    check!(Skip::ALL == [Skip::Bar, Skip::Baz]);
}

#[derive(EnumAll, PartialEq, Eq)]
#[allow(dead_code)]
enum SkipAll {
    #[enum_all(skip)]
    Foo,
    #[enum_all(skip)]
    Bar,
    #[enum_all(skip)]
    Baz,
}

#[test]
fn skip_all() {
    check!(SkipAll::ALL == []);
}

#[derive(EnumAll, PartialEq, Eq)]
#[allow(dead_code)]
enum SkipNonUnit {
    #[enum_all(skip)]
    Named {
        x: usize,
    },
    #[enum_all(skip)]
    Unnamed(usize),
    Unit,
}

#[test]
fn skip_non_unit() {
    check!(SkipNonUnit::ALL == [SkipNonUnit::Unit]);
}
