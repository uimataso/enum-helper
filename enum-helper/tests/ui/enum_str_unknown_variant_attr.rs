#[derive(enum_helper::EnumStr)]
enum Foo {
    #[enum_str(bogus)]
    Bar,
}

fn main() {}