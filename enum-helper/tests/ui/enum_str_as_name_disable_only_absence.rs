// `as_name(disable)` removes the public `as_name` method. The conversion
// impls (`From`, `AsRef`) are decoupled and would still work, but the public
// method is gone, so this call must fail.
#[derive(enum_helper::EnumStr, PartialEq, Eq)]
#[enum_str(as_name(disable))]
enum Foo {
    Bar,
}

fn main() {
    let _ = Foo::Bar.as_name();
}