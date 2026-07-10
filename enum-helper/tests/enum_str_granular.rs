use assert2::check;
use core::fmt;
use enum_helper::EnumStr;

// Custom method names via `name = ...`. The generated `Into<&'static str>` and
// `AsRef<str>` impls are expected to delegate to the *renamed* method.
#[derive(EnumStr, PartialEq, Eq)]
#[enum_str(as_name(name = my_name), as_aliases(name = my_aliases))]
enum CustomMethods {
    FooBar,
    Baz,
}

#[test]
fn custom_method_names() {
    check!(CustomMethods::FooBar.my_name() == "FooBar");
    check!(CustomMethods::Baz.my_name() == "Baz");
    check!(CustomMethods::FooBar.my_aliases() == &["FooBar"]);
    check!(CustomMethods::Baz.my_aliases() == &["Baz"]);
}

#[test]
fn custom_method_names_into_and_as_ref() {
    let s: &'static str = CustomMethods::FooBar.into();
    check!(s == "FooBar");
    let s: &str = CustomMethods::Baz.as_ref();
    check!(s == "Baz");
}

// Custom const names via `name = ...`.
#[derive(EnumStr, PartialEq, Eq)]
#[enum_str(all_names(name = NAMES), all_aliases(name = ALIASES))]
enum CustomConsts {
    #[enum_str(alias = "f")]
    Foo,
    Bar,
}

#[test]
fn custom_const_names() {
    check!(CustomConsts::NAMES == ["Foo", "Bar"]);
    check!(CustomConsts::ALIASES == ["Foo", "f", "Bar"]);
}

// Disabling specific parsing impls while keeping rendering.
#[derive(EnumStr, PartialEq, Eq)]
#[enum_str(impl_from_str(disable), impl_try_from_str(disable))]
enum NoParse {
    Foo,
}

#[test]
fn no_parse_keeps_rendering() {
    check!(NoParse::Foo.as_name() == "Foo");
    check!(NoParse::ALL_NAMES == ["Foo"]);
}

// Disabling `impl_display` removes the generated `Display` impl; the user can
// provide their own without conflict. Other rendering still works.
#[derive(EnumStr, PartialEq, Eq)]
#[enum_str(impl_display(disable))]
enum NoDisplay {
    Foo,
}

impl fmt::Display for NoDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "custom")
    }
}

#[test]
fn no_display_keeps_other_rendering() {
    check!(NoDisplay::Foo.as_name() == "Foo");
    let s: &str = NoDisplay::Foo.as_ref();
    check!(s == "Foo");
    check!(format!("{}", NoDisplay::Foo) == "custom");
}

// Disabling conversion impls while keeping as_name.
#[derive(EnumStr, PartialEq, Eq)]
#[enum_str(impl_into_static_str(disable), impl_as_ref_str(disable))]
enum NoConv {
    Foo,
}

#[test]
fn no_conv_keeps_as_name() {
    check!(NoConv::Foo.as_name() == "Foo");
    check!(NoConv::ALL_NAMES == ["Foo"]);
}

// Decouple: `as_name(disable)` removes the public `as_name` method, but the
// default `From<T> for &'static str` and `AsRef<str>` impls still work. They
// no longer depend on the public `as_name` method.
#[derive(EnumStr, PartialEq, Eq)]
#[enum_str(as_name(disable))]
enum DecoupledNoAsName {
    Foo,
    Bar,
}

#[test]
fn as_name_disable_keeps_conversions() {
    let s: &'static str = DecoupledNoAsName::Foo.into();
    check!(s == "Foo");
    let s: &str = DecoupledNoAsName::Bar.as_ref();
    check!(s == "Bar");
}

// `no_rendering` disables rendering, but the conversion impls can be re-enabled
// independently of `as_name`.
#[derive(EnumStr, PartialEq, Eq)]
#[enum_str(no_rendering, impl_into_static_str(enable), impl_as_ref_str(enable))]
enum NoRenderingButConversions {
    Foo,
}

#[test]
fn no_rendering_with_conversions() {
    let s: &'static str = NoRenderingButConversions::Foo.into();
    check!(s == "Foo");
    let s: &str = NoRenderingButConversions::Foo.as_ref();
    check!(s == "Foo");
}

// Explicit `enable` is a no-op when the feature is on by default.
#[derive(EnumStr, PartialEq, Eq)]
#[enum_str(as_name(enable), impl_from_str(enable))]
enum ExplicitEnable {
    Foo,
}

#[test]
fn explicit_enable() {
    check!(ExplicitEnable::Foo.as_name() == "Foo");
    check!("Foo".parse::<ExplicitEnable>().unwrap() == ExplicitEnable::Foo);
}

// `no_rendering` disables rendering, but `as_name(enable)` / `all_names(enable)`
// re-enable specific items. The generated `Into<&'static str>` delegates to the
// re-enabled `as_name`, so it is generated too.
#[derive(EnumStr, PartialEq, Eq)]
#[enum_str(
    no_rendering,
    as_name(enable),
    impl_into_static_str(enable),
    all_names(enable)
)]
enum ReEnabled {
    Foo,
}

#[test]
fn re_enable_overrides_no_rendering() {
    check!(ReEnabled::Foo.as_name() == "Foo");
    check!(ReEnabled::ALL_NAMES == ["Foo"]);
    let s: &'static str = ReEnabled::Foo.into();
    check!(s == "Foo");
}

// `error(name = ..., vis = "pub")` customizes the error type name and visibility.
#[derive(Debug, EnumStr, PartialEq, Eq)]
#[enum_str(error(name = MyErr, vis = "pub"))]
enum CustomError {
    One,
    Two,
}

#[test]
fn custom_error_name_and_vis() {
    let err = "nope".parse::<CustomError>().unwrap_err();
    check!(format!("{err}") == "invalid CustomError, expected one of \"One\", \"Two\"");
    let _: MyErr = err;
}

// Visibility of generated inherent items follows `vis = ...`, defaulting to the
// enum's own visibility. Here a pub enum in a child module exposes its items.
mod vis_inner {
    use enum_helper::EnumStr;

    #[derive(EnumStr, PartialEq, Eq)]
    #[enum_str(as_name(vis = "pub"), all_names(vis = "pub"))]
    pub enum VisPub {
        Foo,
    }
}

#[test]
fn vis_pub_items_accessible() {
    check!(vis_inner::VisPub::Foo.as_name() == "Foo");
    check!(vis_inner::VisPub::ALL_NAMES == ["Foo"]);
}
