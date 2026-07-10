use enum_helper::EnumStr;

#[derive(EnumStr)]
#[enum_str(as_name(vis = "pub", vis = "pub(crate)"))]
enum Foo {
    Bar,
}

fn main() {}
