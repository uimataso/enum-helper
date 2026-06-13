# `EnumStr`

Derive the [`EnumStr`](enum_helper::EnumStr) trait to convert between a unit enum and string.

```rust
use enum_helper::EnumStr;

#[derive(EnumStr)]
#[enum_str(rename_all = "lowercase")]
pub enum Foo {
    Bar,
    #[enum_str(alias = "bazzz")]
    Baz,
}

// expand to

impl EnumStr for Foo {
    fn as_name(&self) -> &'static str {
        match self {
            Self::Bar => "bar",
            Self::Baz => "baz",
        }
    }

    fn as_aliases(&self) -> &'static [&'static str] {
        match self {
            Self::Bar => &["bar"],
            Self::Baz => &["baz", "bazzz"],
        }
    }
}

impl From<Foo> for &'static str {
    fn from(value: Foo) -> Self { ... }
}

impl AsRef<str> for Foo {
    fn as_ref(&self) -> &str { ... }
}

#[derive(Debug, Clone)]
pub struct InvalidFoo {}

impl InvalidFoo {
    pub fn new(_input: &str) -> Self { ... }
}

impl fmt::Display for InvalidFoo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { ... }
}

impl Error for InvalidFoo {}

impl FromStr for Foo {
    type Err = InvalidFoo;

    fn from_str(s: &str) -> Result<Self, InvalidFoo> {
        match s {
            "bar" => Ok(Self::Bar),
            "baz" => Ok(Self::Baz),
            "bazzz" => Ok(Self::Baz),
            _ => Err(InvalidFoo::new(s)),
        }
    }
}

impl TryFrom<&str> for Foo {
    type Error = InvalidFoo;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "bar" => Ok(Self::Bar),
            "baz" => Ok(Self::Baz),
            "bazzz" => Ok(Self::Baz),
            _ => Err(InvalidFoo::new(value)),
        }
    }
}
```

## Container attributes

### `#[enum_str(rename_all = "...")]`

Rename all variants by rule. Available values:

- `lowercase`
- `UPPERCASE`
- `PascalCase`
- `camelCase`
- `snake_case`
- `SCREAMING_SNAKE_CASE`
- `kebab-case`
- `SCREAMING-KEBAB-CASE`

### `#[enum_str(alias_all = "...")]`

Add aliases to every variant by rule.
Can be specified multiple times.
See `rename_all` for available values.

```rust
#[derive(EnumStr)]
#[enum_str(alias_all = "lowercase", alias_all = "UPPERCASE")]
enum Foo {
    Bar,
    Baz,
}
// "Bar", "bar", and "BAR" all parse to Foo::Bar
```

### `#[enum_str(error_name = InvalidFoo)]`

Customize the generated error struct's name.
Defaults to `Invalid{Enum}`.

### `#[enum_str(error_msg = "...")]`

Customize the error message. Template variables:

- `{name}`: the enum's type name (`Foo`)
- `{input}`: the invalid input string (`input`)
- `{names}`: all variant primary names (`"bar", "baz"`)
- `{aliases}`: all names and aliases (`"bar", "baz", "bazzz"`)

List variables accept format modifiers:

- `{names}`: Default: comma-separated, double-quoted (`"bar", "baz"`)
- `{names:|}`: Custom separator `|`, no quotes (`bar|baz`)
- `{names: - : '}`: Custom separator ` - `, quote char `'` (`'bar' - 'baz'`)

Use `{{` and `}}` for literal braces.
Cannot use `:` as a separator or quote character.

**Limitation:**
If `{input}` appears in the template, the error struct stores the input as `String`, which requires allocation.

### `#[enum_str(no_rendering)]`

Skip all rendering impls. Equivalent to disabling:

- `impl EnumStr for T`
- `impl From<T> for &'static str`
- `impl AsRef<str> for T`

### `#[enum_str(no_parsing)]`

Skip all parsing impls and the error struct. Equivalent to disabling:

- `impl FromStr for T`
- `impl TryFrom<&str> for T`
- Error struct generation

### `#[enum_str(no_error_struct)]`

Skip generating the error struct. See [Bring your own error](#bring-your-own-error).

## Variant attributes

### `#[enum_str(rename = "custom_name")]`

Override the variant's string name.

### `#[enum_str(alias = "alt")]`

Add a parsing alias for the variant. Can be specified multiple times:

```rust
#[derive(EnumStr)]
enum Foo {
    #[enum_str(alias = "b")]
    #[enum_str(alias = "bar")]
    Bar,
}
// "Bar", "b", and "bar" all parse to Foo::Bar
```

## Bring your own error

If you want, you can provide your own error type for `FromStr` / `TryFrom<&str>`:

1. Add `#[enum_str(no_error_struct)]` to skip the generated error struct.
2. Define a type named `Invalid{Enum}`, or use `#[enum_str(error_name = YourError)]`.
3. Implement `YourError::new(input: &str) -> Self` and `fmt::Display`.

For example:

```rust
#[derive(EnumStr)]
#[enum_str(error_name = YourOwnError, no_error_struct)]
enum Foo { Bar }

struct YourOwnError { input: String }

impl YourOwnError {
    fn new(input: &str) -> Self { Self { input: input.to_string() } }
}

impl fmt::Display for YourOwnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "my own error: {}", self.input)
    }
}
```

## Work with `serde`

For unit enums, just add `#[serde(try_from = "&str", into = "&'static str")]`:

For example:

```rust
#[derive(Clone, EnumStr, Serialize, Deserialize)]
#[serde(try_from = "&str", into = "&'static str")]
enum Foo {
    Bar,
}
```

For data-carrying enums, since `EnumStr` doesn't support non-unit variants, you need to use `EnumKind` to generate a unit kind enum and then derive `EnumStr` on it.

Also note that the rename rule must be specified for both `serde` and `enum_str` separately (unfortunately):

```rust
#[derive(Clone, EnumKind, Serialize, Deserialize)]
#[serde(tag = "kind", content = "data")]
#[serde(rename_all = "lowercase")]
#[enum_kind(attr(derive(EnumStr)))]
#[enum_kind(attr(enum_str(rename_all = "lowercase")))]
enum Foo {
    Bar { x: usize },
}
```

For using an `EnumStr` type in a struct field, enable the `serde` feature and use `#[serde(with = "...")]`:

```toml
[dependencies]
enum-helper = { version = "0.1", features = ["serde"] }
```

```rust
#[derive(Serialize, Deserialize)]
struct MyStruct {
    #[serde(with = "enum_helper::serde::enum_str")]
    foo: Foo,
    #[serde(with = "enum_helper::serde::option_enum_str")]
    maybe_foo: Option<Foo>,
}
```
