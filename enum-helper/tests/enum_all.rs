use assert2::check;
use enum_helper::EnumAll;

#[derive(EnumAll, PartialEq, Eq)]
enum BasicCase {
    Foo,
    Bar,
    Baz,
}

#[test]
fn basic_case_all() {
    check!(BasicCase::ALL == [BasicCase::Foo, BasicCase::Bar, BasicCase::Baz]);
}

#[test]
fn basic_case_count() {
    check!(BasicCase::COUNT == 3);
}

#[derive(EnumAll, PartialEq, Eq)]
enum Empty {}

#[test]
fn empty_all() {
    check!(Empty::ALL == []);
}

#[test]
fn empty_count() {
    check!(Empty::COUNT == 0);
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
fn skip_all_const() {
    check!(Skip::ALL == [Skip::Bar, Skip::Baz]);
}

#[test]
fn skip_count() {
    check!(Skip::COUNT == 2);
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
fn skip_all_const_is_empty() {
    check!(SkipAll::ALL == []);
}

#[test]
fn skip_all_count() {
    check!(SkipAll::COUNT == 0);
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
fn skip_non_unit_all() {
    check!(SkipNonUnit::ALL == [SkipNonUnit::Unit]);
}

#[test]
fn skip_non_unit_count() {
    check!(SkipNonUnit::COUNT == 1);
}
