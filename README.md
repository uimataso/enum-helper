# enum-helper

Yet another enum helper.

This crate provides derive macros that generate common boilerplate for enums.

```toml
[dependencies]
enum-helper = "0.1"
```

The `derive` feature is enabled by default, disable with `default-features = false` if you only need the traits.

## Derive macros

- [`EnumStr`](docs/enum_str.md): convert between enum and string
- [`EnumAll`](docs/enum_all.md): get an array of all variants
- [`EnumKind`](docs/enum_kind.md): generate a unit kind enum from a data-carrying enum

## Feature flags

- `derive` (default): re-exports derive macros

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
