// `as_name(disable)` must not generate the inherent `as_name` method.
// Here the conversion impls are disabled too, so the only error is the
// user-side call to a missing method.
#[derive(enum_helper::EnumStr)]
#[enum_str(as_name(disable), impl_into_static_str(disable), impl_as_ref_str(disable))]
enum Foo {
    Bar,
}

fn main() {
    let _ = Foo::Bar.as_name();
}