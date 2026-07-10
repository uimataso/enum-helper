//! Procedural macro implementations for [`enum-helper`].
//!
//! You normally depend on `enum-helper` (which re-exports these macros)
//! rather than using this crate directly.
//!
//! [`enum-helper`]: https://docs.rs/enum-helper

mod attr;
mod ctxt;
mod enum_all;
mod enum_kind;
mod enum_str;
mod gen_option;
mod symbol;
mod template;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

/// Generate string conversion for an enum.
///
/// Supports unit enums by default. Use `#[enum_str(default)]` for non-unit enums.
///
/// This macro generates inherent items and impls on the enum:
///
/// - `const fn as_name(&self) -> &'static str`
/// - `const fn as_aliases(&self) -> &'static [&'static str]`
/// - `const ALL_NAMES: [&'static str; N]`
/// - `const ALL_ALIASES: [&'static str; M]`
/// - `impl From<T> for &'static str`
/// - `impl AsRef<str> for T`
/// - `impl Display for T`
/// - `impl FromStr for T`
/// - `impl TryFrom<&str> for T`
/// - Error struct for invalid conversion from str
///
/// # Container attributes
///
/// - `#[enum_str(rename_all = "snake_case")]`: rename all variants by rule
/// - `#[enum_str(alias_all = "lowercase")]`: add aliases to all variants by rule (repeatable)
/// - `#[enum_str(default)]`: fills non-unit variant's fields with default value during parsing
/// - `#[enum_str(no_rendering)]`: skip `as_name`, `as_aliases`, `ALL_NAMES`, `ALL_ALIASES`, `From`, `AsRef`, `Display`
/// - `#[enum_str(no_parsing)]`: skip `FromStr`, `TryFrom`, and the error struct
/// - `#[enum_str(as_name(name = ..., vis = "...", enable/disable))]`: control the `as_name` method
/// - `#[enum_str(as_aliases(...))]`: control the `as_aliases` method
/// - `#[enum_str(all_names(...))]`: control the `ALL_NAMES` constant
/// - `#[enum_str(all_aliases(...))]`: control the `ALL_ALIASES` constant
/// - `#[enum_str(impl_into_static_str(enable/disable))]`: control `impl From<T> for &'static str`
/// - `#[enum_str(impl_as_ref_str(enable/disable))]`: control `impl AsRef<str> for T`
/// - `#[enum_str(impl_display(enable/disable))]`: control `impl std::fmt::Display for T`
/// - `#[enum_str(impl_from_str(enable/disable))]`: control `impl FromStr for T`
/// - `#[enum_str(impl_try_from_str(enable/disable))]`: control `impl TryFrom<&str> for T`
/// - `#[enum_str(error(name = ..., vis = "...", enable/disable))]`: control the error struct (default name `Invalid{Enum}`)
/// - `#[enum_str(error_msg = "...")]`: custom error message template
///
/// # Variant attributes
///
/// - `#[enum_str(rename = "custom_name")]`: override the variant's name
/// - `#[enum_str(alias = "alt")]`: add an alias (repeatable)
/// - `#[enum_str(skip)]`: skip variant from parsing and `ALL_NAMES` / `ALL_ALIASES`
///
/// # Available rename rules
///
/// - `lowercase`
/// - `UPPERCASE`
/// - `PascalCase`
/// - `camelCase`
/// - `snake_case`
/// - `SCREAMING_SNAKE_CASE`
/// - `kebab-case`
/// - `SCREAMING-KEBAB-CASE`
///
/// # Available error message template variables
///
/// - `{name}`: the enum's type name
/// - `{input}`: the invalid input string (requires allocation)
/// - `{names}`: all variant primary names
/// - `{aliases}`: all names and aliases
///
/// List variables accept format modifiers:
///
/// - `{names}`: default, comma-separated, double-quoted: `"Bar", "Baz"`
/// - `{names:|}`: custom separator, no quotes: `Bar|Baz`
/// - `{names: - :'}`: custom separator and quote char: `'Bar' - 'Baz'`
///
/// Use `{{` and `}}` for literal braces. `:` cannot be used as a separator or quote character.
///
/// # Example
///
/// ```
/// use enum_helper::EnumStr;
///
/// #[derive(EnumStr, Debug, PartialEq, Eq)]
/// #[enum_str(rename_all = "lowercase")]
/// pub enum Foo {
///     Bar,
///     #[enum_str(alias = "bazzz")]
///     Baz,
/// }
///
/// assert_eq!(Foo::Bar.as_name(), "bar");
/// assert_eq!("bazzz".parse::<Foo>().unwrap(), Foo::Baz);
/// ```
///
/// [`EnumStr`]: https://docs.rs/enum-helper/latest/enum_helper/derive.EnumStr.html
#[proc_macro_derive(EnumStr, attributes(enum_str))]
pub fn derive_enum_str(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ir = match enum_str::parse::parse_ir(&input) {
        Ok(ir) => ir,
        Err(err) => return err.into_compile_error().into(),
    };

    enum_str::generate::generate(ir).into()
}

/// Generate an array of all variants and a variant count.
///
/// This macro generates inherent constants on the enum:
///
/// - `const ALL: [Self; N]`
/// - `const COUNT: usize`
///
/// `ALL` requires unit enums (it builds an array of variant values). For
/// non-unit enums, use `#[enum_all(all(disable))]` to derive only `COUNT`, or
/// `#[enum_all(skip)]` on the non-unit variants.
///
/// # Container attributes
///
/// - `#[enum_all(all(name = ..., vis = "...", enable/disable))]`: control the `ALL` constant
/// - `#[enum_all(count(name = ..., vis = "...", enable/disable))]`: control the `COUNT` constant
///
/// # Variant attributes
///
/// - `#[enum_all(skip)]`: exclude this variant from `ALL` and `COUNT`
///
/// # Example
///
/// ```
/// use enum_helper::EnumAll;
///
/// #[derive(EnumAll, Debug, PartialEq, Eq)]
/// enum Foo {
///     Bar,
///     Baz,
///     #[enum_all(skip)]
///     Skipped
/// }
///
/// assert_eq!(Foo::ALL, [Foo::Bar, Foo::Baz]);
/// assert_eq!(Foo::COUNT, 2);
/// ```
///
/// [`EnumAll`]: https://docs.rs/enum-helper/latest/enum_helper/derive.EnumAll.html
#[proc_macro_derive(EnumAll, attributes(enum_all))]
pub fn derive_enum_all(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ir = match enum_all::parse::parse_ir(&input) {
        Ok(ir) => ir,
        Err(err) => return err.into_compile_error().into(),
    };

    enum_all::generate::generate(ir).into()
}

/// Generate a unit kind enum from a data-carrying enum.
///
/// This macro generates:
///
/// - Kind enum (default name `{Enum}Kind`)
/// - `const fn kind(&self) -> {Enum}Kind` inherent method
///
/// # Container attributes
///
/// - `#[enum_kind(name = MyKind)]`: custom name for the generated kind enum (default: `{Enum}Kind`)
/// - `#[enum_kind(attr(...))]`: forward attributes to the generated kind enum (repeatable)
/// - `#[enum_kind(no_default_derive)]`: disable the default `Debug, Clone, Copy, PartialEq, Eq` derives on generated kind enum
///
/// # Variant attributes
///
/// - `#[enum_kind(rename = Bar)]`: rename the kind variant
/// - `#[enum_kind(attr(...))]`: forward attributes to the kind variant (repeatable)
///
/// # Example
///
/// ```
/// use enum_helper::EnumKind;
///
/// #[derive(EnumKind)]
/// enum Message {
///     Text(String),
///     Quit,
/// }
///
/// let msg = Message::Text("hello".into());
/// assert_eq!(msg.kind(), MessageKind::Text);
/// ```
///
/// [`EnumKind`]: https://docs.rs/enum-helper/latest/enum_helper/derive.EnumKind.html
#[proc_macro_derive(EnumKind, attributes(enum_kind))]
pub fn derive_enum_kind(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ir = match enum_kind::parse::parse_ir(&input) {
        Ok(ir) => ir,
        Err(err) => return err.into_compile_error().into(),
    };

    enum_kind::generate::generate(ir).into()
}
