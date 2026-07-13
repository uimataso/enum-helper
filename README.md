# enum-helper

[![CI](https://github.com/uimataso/enum-helper/actions/workflows/ci.yml/badge.svg)](https://github.com/uimataso/enum-helper/actions/workflows/ci.yml)
[![Latest Version](https://img.shields.io/crates/v/enum-helper.svg)](https://crates.io/crates/enum-helper)
[![Rust Documentation](https://docs.rs/enum-helper/badge.svg)](https://docs.rs/enum-helper)
![Crates.io](https://img.shields.io/crates/l/enum-helper)
![Crates.io](https://img.shields.io/crates/d/enum-helper)

Yet another enum helper.

This crate provides derive macros that generate common boilerplate for enums.

```toml
[dependencies]
enum-helper = "0.3"
```

The `derive` feature is enabled by default.

## Derive macros

- [`EnumStr`](docs/enum_str.md): convert between enum and string
- [`EnumAll`](docs/enum_all.md): get an array of all variants
- [`EnumKind`](docs/enum_kind.md): generate a unit kind enum from a data-carrying enum

## Feature flags

- `derive` (default): re-exports derive macros

## Similar crates

- [`strum`](https://github.com/Peternator7/strum): the most feature-rich enum derive crate ([comparison](docs/comparison_with_strum.md))
- [`enum-kinds`](https://github.com/Soft/enum-kinds): generates a unit kind enum (similar to [`EnumKind`](docs/enum_kind.md))
- [`enum-iterator`](https://github.com/stephaneyfx/enum-iterator): iterate over all variants (similar to [`EnumAll`](docs/enum_all.md))
- [`derive_more`](https://github.com/JelteF/derive_more): general-purpose derive macros

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
