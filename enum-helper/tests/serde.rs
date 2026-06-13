use assert2::check;
use enum_helper::EnumStr;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(EnumStr, PartialEq, Eq)]
#[enum_str(rename_all = "lowercase")]
enum Foo {
    Bar,
    Baz,
}

#[derive(Serialize, Deserialize, PartialEq, Eq)]
struct SerdeWith {
    #[serde(with = "enum_helper::serde::enum_str")]
    foo: Foo,
    #[serde(with = "enum_helper::serde::option_enum_str")]
    some_foo: Option<Foo>,
    #[serde(with = "enum_helper::serde::option_enum_str")]
    none_foo: Option<Foo>,
}

#[test]
fn test_serialize() {
    let val = SerdeWith {
        foo: Foo::Bar,
        some_foo: Some(Foo::Baz),
        none_foo: None,
    };

    let v = json!({
        "foo": "bar",
        "some_foo": "baz",
        "none_foo": null,
    });

    let s = "{\"foo\":\"bar\",\"some_foo\":\"baz\",\"none_foo\":null}";

    check!(serde_json::to_value(&val).unwrap() == v);
    check!(serde_json::to_string(&val).unwrap() == s);
    check!(serde_json::from_value::<SerdeWith>(v).unwrap() == val);
    check!(serde_json::from_str::<SerdeWith>(s).unwrap() == val);
}
