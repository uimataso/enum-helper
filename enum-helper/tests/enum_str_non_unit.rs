use assert2::check;
use enum_helper::EnumStr;

#[derive(EnumStr, PartialEq, Eq, Debug)]
#[enum_str(default)]
enum SimpleNonUnit {
    Alpha,
    Bravo { x: usize },
    Charlie(String),
}

#[test]
fn default_struct_variant_rendering() {
    check!(SimpleNonUnit::Bravo { x: 5 }.as_name() == "Bravo");
    check!(SimpleNonUnit::Bravo { x: 5 }.as_aliases() == &["Bravo"]);
    let s: &str = SimpleNonUnit::Bravo { x: 5 }.as_ref();
    check!(s == "Bravo");
    let s: &'static str = SimpleNonUnit::Bravo { x: 5 }.into();
    check!(s == "Bravo");
}

#[test]
fn default_tuple_variant_rendering() {
    check!(SimpleNonUnit::Charlie("hello".into()).as_name() == "Charlie");
    check!(SimpleNonUnit::Charlie("hello".into()).as_aliases() == &["Charlie"]);
}

#[test]
fn default_unit_variant_rendering() {
    check!(SimpleNonUnit::Alpha.as_name() == "Alpha");
    check!(SimpleNonUnit::Alpha.as_aliases() == &["Alpha"]);
}

#[test]
fn default_struct_variant_parsing() {
    let v = "Bravo".parse::<SimpleNonUnit>().unwrap();
    check!(v == SimpleNonUnit::Bravo { x: 0 });
}

#[test]
fn default_tuple_variant_parsing() {
    let v = "Charlie".parse::<SimpleNonUnit>().unwrap();
    check!(v == SimpleNonUnit::Charlie(String::default()));
}

#[test]
fn default_unit_variant_parsing() {
    check!("Alpha".parse::<SimpleNonUnit>().unwrap() == SimpleNonUnit::Alpha);
}

#[test]
fn default_unknown_string_errors() {
    check!("nope".parse::<SimpleNonUnit>().is_err());
}

#[derive(EnumStr, PartialEq, Eq, Debug)]
#[enum_str(default, rename_all = "snake_case")]
enum DefaultRenameAll {
    AlphaBravo,
    CharlieDelta { x: usize },
}

#[test]
fn default_rename_all_rendering() {
    check!(DefaultRenameAll::AlphaBravo.as_name() == "alpha_bravo");
    check!(DefaultRenameAll::CharlieDelta { x: 1 }.as_name() == "charlie_delta");
}

#[test]
fn default_rename_all_parsing() {
    check!("alpha_bravo".parse::<DefaultRenameAll>().unwrap() == DefaultRenameAll::AlphaBravo);
    check!(
        "charlie_delta".parse::<DefaultRenameAll>().unwrap()
            == DefaultRenameAll::CharlieDelta { x: 0 }
    );
}

#[derive(EnumStr, PartialEq, Eq, Debug)]
#[enum_str(default, alias_all = "lowercase")]
enum DefaultAliasAll {
    Alpha,
    Bravo { x: usize },
}

#[test]
fn default_alias_all_parsing() {
    check!("Alpha".parse::<DefaultAliasAll>().unwrap() == DefaultAliasAll::Alpha);
    check!("alpha".parse::<DefaultAliasAll>().unwrap() == DefaultAliasAll::Alpha);
    check!("Bravo".parse::<DefaultAliasAll>().unwrap() == DefaultAliasAll::Bravo { x: 0 });
    check!("bravo".parse::<DefaultAliasAll>().unwrap() == DefaultAliasAll::Bravo { x: 0 });
}

#[test]
fn default_alias_all_rendering() {
    check!(DefaultAliasAll::Alpha.as_aliases() == &["Alpha", "alpha"]);
    check!(DefaultAliasAll::Bravo { x: 0 }.as_aliases() == &["Bravo", "bravo"]);
}

#[derive(EnumStr, PartialEq, Eq, Debug)]
#[enum_str(default)]
enum DefaultWithAlias {
    Alpha,
    #[enum_str(alias = "b")]
    Bravo {
        x: usize,
    },
}

#[test]
fn default_with_alias_parsing() {
    check!("Bravo".parse::<DefaultWithAlias>().unwrap() == DefaultWithAlias::Bravo { x: 0 });
    check!("b".parse::<DefaultWithAlias>().unwrap() == DefaultWithAlias::Bravo { x: 0 });
}

#[test]
fn default_with_alias_rendering() {
    check!(DefaultWithAlias::Bravo { x: 0 }.as_aliases() == &["Bravo", "b"]);
}

#[derive(EnumStr, PartialEq, Eq, Debug)]
#[enum_str(default, rename_all = "lowercase")]
enum DefaultRenameOverride {
    Alpha,
    #[enum_str(rename = "CustomName")]
    Bravo {
        x: usize,
    },
}

#[test]
fn default_rename_override_parsing() {
    check!("alpha".parse::<DefaultRenameOverride>().unwrap() == DefaultRenameOverride::Alpha);
    check!(
        "CustomName".parse::<DefaultRenameOverride>().unwrap()
            == DefaultRenameOverride::Bravo { x: 0 }
    );
    check!("bravo".parse::<DefaultRenameOverride>().is_err());
}

#[test]
fn default_rename_override_rendering() {
    check!(DefaultRenameOverride::Bravo { x: 0 }.as_name() == "CustomName");
}

#[derive(EnumStr, PartialEq, Eq, Debug)]
enum SkipUnit {
    Alpha,
    Bravo,
    #[enum_str(skip)]
    Charlie,
}

#[test]
fn skip_unit_rendering() {
    check!(SkipUnit::Charlie.as_name() == "Charlie");
    check!(SkipUnit::Charlie.as_aliases() == &["Charlie"]);
    let s: &'static str = SkipUnit::Charlie.into();
    check!(s == "Charlie");
    let s: &str = SkipUnit::Charlie.as_ref();
    check!(s == "Charlie");
}

#[test]
fn skip_unit_parsing() {
    check!("Alpha".parse::<SkipUnit>().unwrap() == SkipUnit::Alpha);
    check!("Bravo".parse::<SkipUnit>().unwrap() == SkipUnit::Bravo);
    check!("Charlie".parse::<SkipUnit>().is_err());
}

#[test]
fn skip_unit_excluded_from_all_names() {
    check!(SkipUnit::ALL_NAMES == ["Alpha", "Bravo"]);
}

#[test]
fn skip_unit_excluded_from_all_aliases() {
    check!(SkipUnit::ALL_ALIASES == ["Alpha", "Bravo"]);
}

#[derive(EnumStr, PartialEq, Eq, Debug)]
#[enum_str(rename_all = "lowercase")]
enum SkipWithRenameAll {
    Alpha,
    #[enum_str(skip)]
    Bravo,
}

#[test]
fn skip_rename_all_rendering() {
    check!(SkipWithRenameAll::Bravo.as_name() == "bravo");
}

#[test]
fn skip_rename_all_parsing() {
    check!("alpha".parse::<SkipWithRenameAll>().unwrap() == SkipWithRenameAll::Alpha);
    check!("bravo".parse::<SkipWithRenameAll>().is_err());
}

#[derive(EnumStr, PartialEq, Eq, Debug)]
enum SkipWithAlias {
    #[enum_str(alias = "a")]
    Alpha,
    #[enum_str(skip, alias = "b")]
    Bravo,
}

#[test]
fn skip_alias_rendering() {
    check!(SkipWithAlias::Bravo.as_aliases() == &["Bravo", "b"]);
}

#[test]
fn skip_alias_parsing() {
    check!("Alpha".parse::<SkipWithAlias>().unwrap() == SkipWithAlias::Alpha);
    check!("a".parse::<SkipWithAlias>().unwrap() == SkipWithAlias::Alpha);
    check!("Bravo".parse::<SkipWithAlias>().is_err());
    check!("b".parse::<SkipWithAlias>().is_err());
}

#[derive(EnumStr, PartialEq, Eq, Debug)]
#[enum_str(default)]
enum SkipNonUnit {
    Alpha,
    #[enum_str(skip)]
    Bravo {
        x: usize,
    },
    Charlie(String),
}

#[test]
fn skip_non_unit_rendering() {
    check!(SkipNonUnit::Bravo { x: 5 }.as_name() == "Bravo");
    check!(SkipNonUnit::Bravo { x: 5 }.as_aliases() == &["Bravo"]);
}

#[test]
fn skip_non_unit_parsing() {
    check!("Alpha".parse::<SkipNonUnit>().unwrap() == SkipNonUnit::Alpha);
    check!("Bravo".parse::<SkipNonUnit>().is_err());
    check!("Charlie".parse::<SkipNonUnit>().unwrap() == SkipNonUnit::Charlie(String::default()));
}
