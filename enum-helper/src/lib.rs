//! [![CI](https://github.com/uimataso/enum-helper/actions/workflows/ci.yml/badge.svg)](https://github.com/uimataso/enum-helper/actions/workflows/ci.yml)
//! [![Latest Version](https://img.shields.io/crates/v/enum-helper.svg)](https://crates.io/crates/enum-helper)
//! [![Rust Documentation](https://docs.rs/enum-helper/badge.svg)](https://docs.rs/enum-helper)
//! ![Crates.io](https://img.shields.io/crates/l/enum-helper)
//! ![Crates.io](https://img.shields.io/crates/d/enum-helper)
//!
//! Yet another enum helper.
//!
//! This crate provides derive macros that generate common boilerplate for enums.
//!
//! - [`EnumStr`]: convert between enum and string
//! - [`EnumAll`]: get an array of all variants
//! - [`EnumKind`]: generate a unit kind enum from a data-carrying enum
//!
//! ## Feature flags
//!
//! - `derive` (default): re-exports derive macros
//!
//! [`EnumStr`]: derive@enum_helper_derive::EnumStr
//! [`EnumAll`]: derive@enum_helper_derive::EnumAll
//! [`EnumKind`]: derive@enum_helper_derive::EnumKind

#[cfg(feature = "derive")]
pub use enum_helper_derive::EnumAll;
#[cfg(feature = "derive")]
pub use enum_helper_derive::EnumKind;
#[cfg(feature = "derive")]
pub use enum_helper_derive::EnumStr;
