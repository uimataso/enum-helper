use assert2::check;
use enum_helper::EnumStr;

#[derive(EnumStr, PartialEq, Eq)]
enum BasicCase {
    FooBar,
    Baz,
}

#[test]
fn basic_case_as_name() {
    check!(BasicCase::FooBar.as_name() == "FooBar");
    check!(BasicCase::Baz.as_name() == "Baz");
}

#[test]
fn basic_case_as_aliases() {
    check!(BasicCase::FooBar.as_aliases() == &["FooBar"]);
    check!(BasicCase::Baz.as_aliases() == &["Baz"]);
}

#[test]
fn basic_case_into_str() {
    let s: &'static str = BasicCase::FooBar.into();
    check!(s == "FooBar");
    let s: &'static str = BasicCase::Baz.into();
    check!(s == "Baz");
}

#[test]
fn basic_case_as_ref_str() {
    let s: &str = BasicCase::FooBar.as_ref();
    check!(s == "FooBar");
    let s: &str = BasicCase::Baz.as_ref();
    check!(s == "Baz");
}

#[test]
fn basic_case_from_str() {
    check!("FooBar".parse::<BasicCase>().unwrap() == BasicCase::FooBar);
    check!("Baz".parse::<BasicCase>().unwrap() == BasicCase::Baz);
    check!("bar".parse::<BasicCase>().is_err());
}

#[test]
fn basic_case_try_from_str() {
    let x: BasicCase = "FooBar".try_into().unwrap();
    check!(x == BasicCase::FooBar);

    let x = BasicCase::try_from("Baz").unwrap();
    check!(x == BasicCase::Baz);

    check!(BasicCase::try_from("baz").is_err());
}

#[derive(EnumStr, PartialEq, Eq)]
enum SpecialName {
    #[allow(non_camel_case_types)]
    r#type,
    中文,
}

#[test]
fn special_name_as_name() {
    check!(SpecialName::r#type.as_name() == "r#type"); // we could make this produce "type", but why?
    check!(SpecialName::中文.as_name() == "中文");
}

#[test]
fn special_name_from_name() {
    check!("r#type".parse::<SpecialName>().unwrap() == SpecialName::r#type); // we could make this produce "type", but why?
    check!("中文".parse::<SpecialName>().unwrap() == SpecialName::中文);
}

#[derive(Debug, EnumStr, PartialEq, Eq)]
#[enum_str(error_name = MyBadError)]
enum ErrorCustomName {
    One,
    Two,
}

#[test]
fn error_custom_name() {
    let err = "nope".parse::<ErrorCustomName>().unwrap_err();
    check!(format!("{err}") == "invalid ErrorCustomName, expected one of \"One\", \"Two\"");
    let _: MyBadError = err;
}
