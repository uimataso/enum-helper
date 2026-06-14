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
mod symbol;
mod template;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

/// Derive the [`EnumStr`] trait to convert between a unit enum and string.
///
/// `EnumStr` only supports unit enums (enums where all variants have no
/// fields).
///
/// This macro generates:
///
/// - `impl EnumStr for T`
/// - `impl From<T> for &'static str`
/// - `impl AsRef<str> for T`
/// - `impl FromStr for T`
/// - `impl TryFrom<&str> for T`
/// - Error struct for invalid conversion from str
///
/// # Container attributes
///
/// - `#[enum_str(rename_all = "snake_case")]`: rename all variants by rule
/// - `#[enum_str(alias_all = "lowercase")]`: add aliases to all variants by rule (repeatable)
/// - `#[enum_str(default)]`: use default value for non-unit variants
/// - `#[enum_str(error_name = InvalidFoo)]`: custom error type name (default: `Invalid{Enum}`)
/// - `#[enum_str(error_msg = "…")]`: custom error message template
/// - `#[enum_str(no_rendering)]`: skip `EnumStr`, `From`, `AsRef` impls
/// - `#[enum_str(no_parsing)]`: skip `FromStr`, `TryFrom`, and the error struct
/// - `#[enum_str(no_error_struct)]`: skip the generated error struct (bring your own)
///
/// # Variant attributes
///
/// - `#[enum_str(rename = "custom_name")]`: override the variant's name
/// - `#[enum_str(alias = "alt")]`: add an alias (repeatable)
/// - `#[enum_str(skip)]`: skip variant, only affect parsing
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
/// [`EnumStr`]: https://docs.rs/enum-helper/latest/enum_helper/trait.EnumStr.html
#[proc_macro_derive(EnumStr, attributes(enum_str))]
pub fn derive_enum_str(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ir = match enum_str::parse::parse_ir(&input) {
        Ok(ir) => ir,
        Err(err) => return err.into_compile_error().into(),
    };

    enum_str::generate::generate(ir).into()
}

/// Derive the [`EnumAll`] trait and generate an array of all variants.
///
/// This macro generates:
///
/// - `impl EnumAll for T`
///
/// # Container attributes
///
/// *(none)*
///
/// # Variant attributes
///
/// - `#[enum_all(skip)]`: exclude this variant from the `ALL` array
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
/// ```
///
/// [`EnumAll`]: https://docs.rs/enum-helper/latest/enum_helper/trait.EnumAll.html
#[proc_macro_derive(EnumAll, attributes(enum_all))]
pub fn derive_enum_all(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ir = match enum_all::parse::parse_ir(&input) {
        Ok(ir) => ir,
        Err(err) => return err.into_compile_error().into(),
    };

    enum_all::generate::generate(ir).into()
}

/// Derive the [`EnumKind`] trait and generate a unit kind enum from a data-carrying enum.
///
/// This macro generates:
///
/// - Kind enum
/// - `impl EnumKind for T`
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
/// [`EnumKind`]: https://docs.rs/enum-helper/latest/enum_helper/trait.EnumKind.html
#[proc_macro_derive(EnumKind, attributes(enum_kind))]
pub fn derive_enum_kind(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ir = match enum_kind::parse::parse_ir(&input) {
        Ok(ir) => ir,
        Err(err) => return err.into_compile_error().into(),
    };

    enum_kind::generate::generate(ir).into()
}
