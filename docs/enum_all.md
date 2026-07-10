# `EnumAll`

Generate an array of all variants and a variant count.

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
assert_eq!(Foo::COUNT, 2);

// expand to:

impl Foo {
    pub const ALL: [Self; 2] = [Self::Bar, Self::Baz];
    pub const COUNT: usize = 2;
}
```

Only `ALL` requires unit enums, since it builds an array of variant values.
For non-unit enums, use `#[enum_all(all(disable))]` to derive only `COUNT`,
or `#[enum_all(skip)]` on the non-unit variants.

## Container attributes

### `#[enum_all(all(name = ..., vis = "...", enable/disable))]`

Control the `ALL` constant.

- `name`: constant name, default `ALL`
- `vis`: constant visibility, default to the enum's visibility
- `enable`/`disable`: generate or skip

### `#[enum_all(count(name = ..., vis = "...", enable/disable))]`

Control the `COUNT` constant.
Same options as `all`, default name `COUNT`.

## Variant attributes

### `#[enum_all(skip)]`

Exclude this variant from the `ALL` array and the `COUNT`.

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
