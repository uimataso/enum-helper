// `impl_from_str(disable)` must not generate `FromStr`.
#[derive(enum_helper::EnumStr, PartialEq, Eq)]
#[enum_str(impl_from_str(disable))]
enum Foo {
    Bar,
}

fn main() {
    let _ = "Bar".parse::<Foo>().unwrap();
}