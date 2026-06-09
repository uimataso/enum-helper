use enum_helper::EnumStr;

#[derive(EnumStr, PartialEq, Eq)]
#[enum_str(no_parsing)]
enum NoErrorStruct {
    Alpha,
}

fn main() {
    let _ = InvalidNoErrorStruct::new("test");
}

