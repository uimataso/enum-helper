// `count(disable)` must not generate the `COUNT` const.
#[derive(enum_helper::EnumAll, PartialEq, Eq)]
#[enum_all(count(disable))]
enum Foo {
    Bar,
}

fn main() {
    let _ = Foo::COUNT;
}