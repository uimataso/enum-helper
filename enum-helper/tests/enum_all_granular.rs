use assert2::check;
use enum_helper::EnumAll;

// Custom const names via `name = ...`.
#[derive(EnumAll, PartialEq, Eq)]
#[enum_all(all(name = ITEMS), count(name = NUM))]
enum CustomNames {
    Foo,
    Bar,
}

#[test]
fn custom_names() {
    check!(CustomNames::ITEMS == [CustomNames::Foo, CustomNames::Bar]);
    check!(CustomNames::NUM == 2);
}

// `all(disable)` must not generate `ALL`, but `COUNT` is unaffected.
#[derive(EnumAll, PartialEq, Eq)]
#[enum_all(all(disable))]
#[allow(dead_code)]
enum NoAll {
    Foo,
    Bar,
}

#[test]
fn no_all_keeps_count() {
    check!(NoAll::COUNT == 2);
}

// `count(disable)` must not generate `COUNT`, but `ALL` is unaffected.
#[derive(EnumAll, PartialEq, Eq)]
#[enum_all(count(disable))]
enum NoCount {
    Foo,
    Bar,
}

#[test]
fn no_count_keeps_all() {
    check!(NoCount::ALL == [NoCount::Foo, NoCount::Bar]);
}

// Explicit `enable` is a no-op when the feature is on by default.
#[derive(EnumAll, PartialEq, Eq)]
#[enum_all(all(enable), count(enable))]
enum ExplicitEnable {
    Foo,
}

#[test]
fn explicit_enable() {
    check!(ExplicitEnable::ALL == [ExplicitEnable::Foo]);
    check!(ExplicitEnable::COUNT == 1);
}

// Visibility of generated consts follows `vis = ...`, defaulting to the enum's
// own visibility. A pub enum in a child module exposes its consts.
mod vis_inner {
    use enum_helper::EnumAll;

    #[derive(EnumAll, PartialEq, Eq)]
    #[enum_all(all(vis = "pub"), count(vis = "pub"))]
    pub enum VisPub {
        Foo,
    }
}

#[test]
fn vis_pub_consts_accessible() {
    check!(vis_inner::VisPub::ALL == [vis_inner::VisPub::Foo]);
    check!(vis_inner::VisPub::COUNT == 1);
}
