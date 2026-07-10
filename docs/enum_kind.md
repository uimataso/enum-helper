# `EnumKind`

Generate a unit kind enum from a data-carrying enum.

```rust
use enum_helper::EnumKind;

#[derive(EnumKind, PartialEq, Eq)]
pub enum Foo {
    Bar { x: usize },
    Baz(String),
}

let val = Foo::Bar { x: 1 };
assert_eq!(val.kind(), FooKind::Bar);

// expand to:

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FooKind { Bar, Baz }

impl Foo {
    pub const fn kind(&self) -> FooKind { ... }
}
```

## Container attributes

### `#[enum_kind(name = MyKind)]`

Custom name for the generated kind enum. Defaults to `{Enum}Kind`.

### `#[enum_kind(attr(...))]`

Forward attributes to the generated kind enum.
Can be specified multiple times.

```rust
#[derive(EnumKind)]
#[enum_kind(attr(derive(Default)))]
enum Msg {
    Text(String),
    #[enum_kind(attr(default))]
    Unknown,
}

assert_eq!(MsgKind::default(), MsgKind::Unknown);
```

### `#[enum_kind(no_default_derive)]`

Disable the default `Debug, Clone, Copy, PartialEq, Eq` derives on the
generated kind enum.
You can then add your own derives via `attr(...)`.

## Variant attributes

### `#[enum_kind(rename = Bar)]`

Rename the kind variant.

### `#[enum_kind(attr(...))]`

Forward attributes to the generated kind variant.
Can be specified multiple times.
