#[derive(enum_helper::EnumAll)]
enum Foo {
    #[enum_all(bogus)]
    Bar,
}

fn main() {}