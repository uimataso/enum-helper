// `all(disable)` must not generate the `ALL` const.
#[derive(enum_helper::EnumAll, PartialEq, Eq)]
#[enum_all(all(disable))]
enum Foo {
    Bar,
}

fn main() {
    let _ = Foo::ALL;
}