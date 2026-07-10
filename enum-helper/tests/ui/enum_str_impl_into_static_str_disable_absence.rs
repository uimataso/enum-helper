// `impl_into_static_str(disable)` must not generate `From<Foo> for &'static str`.
#[derive(enum_helper::EnumStr, PartialEq, Eq)]
#[enum_str(impl_into_static_str(disable))]
enum Foo {
    Bar,
}

fn main() {
    let _: &'static str = Foo::Bar.into();
}