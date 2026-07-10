use assert2::check;
use enum_helper::EnumStr;

#[derive(EnumStr, PartialEq, Eq)]
#[enum_str(rename_all = "snake_case")]
#[enum_str(alias_all = "lowercase")]
enum RenameAliasAll {
    FooBar,
    Baz,
}

#[test]
fn rename_all_alias_all_as_aliases() {
    check!(RenameAliasAll::FooBar.as_aliases() == &["foo_bar", "foobar"]);
    check!(RenameAliasAll::Baz.as_aliases() == &["baz"]);
}

#[test]
fn rename_all_alias_all_from_str() {
    check!("foo_bar".parse::<RenameAliasAll>().unwrap() == RenameAliasAll::FooBar);
    check!("foobar".parse::<RenameAliasAll>().unwrap() == RenameAliasAll::FooBar);
    check!("baz".parse::<RenameAliasAll>().unwrap() == RenameAliasAll::Baz);
}

#[derive(EnumStr, PartialEq, Eq)]
#[enum_str(rename_all = "lowercase")]
enum RenameOverride {
    #[enum_str(rename = "CustomName")]
    FooBar,
    Baz,
}

#[test]
fn rename_override_as_name() {
    check!(RenameOverride::FooBar.as_name() == "CustomName");
    check!(RenameOverride::Baz.as_name() == "baz");
}

#[test]
fn rename_override_from_str() {
    check!("CustomName".parse::<RenameOverride>().unwrap() == RenameOverride::FooBar);
    check!("baz".parse::<RenameOverride>().unwrap() == RenameOverride::Baz);
    check!("FooBar".parse::<RenameOverride>().is_err());
    check!("foobar".parse::<RenameOverride>().is_err());
}

#[derive(EnumStr, PartialEq, Eq)]
enum WithAlias {
    #[enum_str(alias = "foo")]
    #[enum_str(alias = "f")]
    FooBar,
    Baz,
}

#[test]
fn alias_as_aliases() {
    check!(WithAlias::FooBar.as_aliases() == &["FooBar", "foo", "f"]);
    check!(WithAlias::Baz.as_aliases() == &["Baz"]);
}

#[test]
fn alias_from_str() {
    check!("FooBar".parse::<WithAlias>().unwrap() == WithAlias::FooBar);
    check!("foo".parse::<WithAlias>().unwrap() == WithAlias::FooBar);
    check!("f".parse::<WithAlias>().unwrap() == WithAlias::FooBar);
    check!("Baz".parse::<WithAlias>().unwrap() == WithAlias::Baz);
    check!("baz".parse::<WithAlias>().is_err());
}

#[derive(EnumStr, PartialEq, Eq)]
#[enum_str(alias_all = "lowercase")]
enum DedupAlias {
    Foo,
    Baz,
}

#[test]
fn dedup_alias_as_aliases() {
    check!(DedupAlias::Foo.as_aliases() == &["Foo", "foo"]);
    check!(DedupAlias::Baz.as_aliases() == &["Baz", "baz"]);
}

#[derive(EnumStr, PartialEq, Eq)]
#[enum_str(rename_all = "snake_case")]
#[enum_str(alias_all = "lowercase")]
enum FullCombo {
    #[enum_str(rename = "overridden")]
    #[enum_str(alias = "extra")]
    #[enum_str(alias = "foobar")]
    FooBar,
    BazQux,
}

#[test]
fn full_combo_as_aliases() {
    check!(FullCombo::FooBar.as_aliases() == &["overridden", "foobar", "extra"]);
    check!(FullCombo::BazQux.as_aliases() == &["baz_qux", "bazqux"]);
}

#[test]
fn full_combo_from_str() {
    check!("overridden".parse::<FullCombo>().unwrap() == FullCombo::FooBar);
    check!("foobar".parse::<FullCombo>().unwrap() == FullCombo::FooBar);
    check!("extra".parse::<FullCombo>().unwrap() == FullCombo::FooBar);
    check!("baz_qux".parse::<FullCombo>().unwrap() == FullCombo::BazQux);
    check!("bazqux".parse::<FullCombo>().unwrap() == FullCombo::BazQux);
    check!("foo_bar".parse::<FullCombo>().is_err());
}
