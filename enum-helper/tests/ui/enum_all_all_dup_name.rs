#[derive(enum_helper::EnumAll)]
#[enum_all(all(name = X, name = Y))]
enum Foo {
    Bar,
}

fn main() {}
