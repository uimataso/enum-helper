# `EnumStr`

Generate string conversion for an enum.

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

impl Foo {
    pub const fn as_name(&self) -> &'static str {
        match self {
            Self::Bar => "bar",
            Self::Baz => "baz",
        }
    }

    pub const fn as_aliases(&self) -> &'static [&'static str] {
        match self {
            Self::Bar => &["bar"],
            Self::Baz => &["baz", "bazzz"],
        }
    }

    pub const ALL_NAMES: [&'static str; 2] = ["bar", "baz"];
    pub const ALL_ALIASES: [&'static str; 3] = ["bar", "baz", "bazzz"];
}

impl From<Foo> for &'static str {
    fn from(value: Foo) -> Self { ... }
}

impl AsRef<str> for Foo {
    fn as_ref(&self) -> &str { ... }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
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

### `#[enum_str(default)]`

Enable non-unit enum support.
When parsing non-unit variants, fills all field with default value.

```rust
#[derive(EnumStr)]
#[enum_str(default)]
enum Foo {
    Bar { x: usize }
    Baz(String)
}

// "Bar" -> Foo::Bar { x: Default::default() }
// "Baz" -> Foo::Baz(Default::default())
```

### `#[enum_str(no_rendering)]`

Skip all rendering. Equivalent to disabling:

- `as_name`
- `as_aliases`
- `ALL_NAMES`
- `ALL_ALIASES`
- `impl From<T> for &'static str`
- `impl AsRef<str> for T`

### `#[enum_str(no_parsing)]`

Skip all parsing and the error struct. Equivalent to disabling:

- `impl FromStr for T`
- `impl TryFrom<&str> for T`
- Error struct generation

### `#[enum_str(as_name(name = ..., vis = "...", enable/disable))]`

Control the `as_name` method.

- `name`: method name, default `as_name`
- `vis`: method visibility, default to the enum's visibility
- `enable`/`disable`: generate or skip

Disabling `as_name` does **not** disable `impl From<T> for &'static str` or `impl AsRef<str>` — those delegate to an internal helper, not the public method. Use [`impl_into_static_str(disable)`](#enum_strimpl_into_static_strenable_disable) / [`impl_as_ref_str(disable)`](#enum_strimpl_as_ref_strenable_disable) (or [`no_rendering`](#enum_strno_rendering)) to remove them.

### `#[enum_str(as_aliases(name = ..., vis = "...", enable/disable))]`

Control the `as_aliases` method.
Same options as `as_name`, default name `as_aliases`.

### `#[enum_str(all_names(name = ..., vis = "...", enable/disable))]`

Control the `ALL_NAMES` constant.
Default name `ALL_NAMES`.

### `#[enum_str(all_aliases(name = ..., vis = "...", enable/disable))]`

Control the `ALL_ALIASES` constant.
Default name `ALL_ALIASES`.

### `#[enum_str(impl_into_static_str(enable/disable))]`

Control `impl From<T> for &'static str`.
Enabled by default, disabled by `no_rendering`.

### `#[enum_str(impl_as_ref_str(enable/disable))]`

Control `impl AsRef<str> for T`.
Enabled by default, disabled by `no_rendering`.

### `#[enum_str(impl_from_str(enable/disable))]`

Control `impl FromStr for T`.
Enabled by default, disabled by `no_parsing`.

### `#[enum_str(impl_try_from_str(enable/disable))]`

Control `impl TryFrom<&str> for T`.
Enabled by default, disabled by `no_parsing`.

### `#[enum_str(error(name = ..., vis = "...", enable/disable))]`

Control the generated error struct.

- `name`: struct name, default `Invalid{Enum}`
- `vis`: struct visibility, default to the enum's visibility
- `enable`/`disable`: generate or skip (see [Bring your own error](#bring-your-own-error))

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

### `#[enum_str(skip)]`

Excludes a variant from parsing and the `ALL_NAMES` / `ALL_ALIASES` constants.
Can be used to skip non-unit variants.

Rendering is not affected: `as_name`, `as_aliases`, `AsRef<str>` etc still work on a skipped variant value.

## Bring your own error

If you want, you can provide your own error type for `FromStr` / `TryFrom<&str>`:

1. Add `#[enum_str(error(disable))]` to skip the generated error struct.
2. Define a type named `Invalid{Enum}`, or use `#[enum_str(error(name = YourError))]`.
3. Implement `YourError::new(input: &str) -> Self` and `fmt::Display`.

For example:

```rust
#[derive(EnumStr)]
#[enum_str(error(name = YourOwnError, disable))]
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

```rust
#[derive(Clone, EnumStr, Serialize, Deserialize)]
#[serde(try_from = "&str", into = "&'static str")]
enum Foo {
    Bar,
}
```
