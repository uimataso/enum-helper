# `EnumAll`

Derive the [`EnumAll`](enum_helper::EnumAll) trait and generate an array of all variants.

```rust
use enum_helper::EnumAll;

#[derive(EnumAll, PartialEq, Eq)]
pub enum Foo {
    Bar,
    Baz,
    #[enum_all(skip)]
    Skipped,
}

assert_eq!(Foo::ALL, [Foo::Bar, Foo::Baz]);

// expand to:

impl EnumAll for Foo {
    type All = [Self; 2];
    const ALL: Self::All = [Self::Bar, Self::Baz];
}
```

## Container attributes

*(none)*

## Variant attributes

### `#[enum_all(skip)]`

Exclude this variant from the `ALL` array.

You can use this to skip non-unit variants:

```rust
#[derive(EnumAll, PartialEq, Eq)]
pub enum Foo {
    Unit,
    #[enum_all(skip)]
    Named { x: usize },
    #[enum_all(skip)]
    Unnamed(usize),
}
```
