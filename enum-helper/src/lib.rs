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
//! - `serde`: provides serde helpers for use with [`EnumStr`]
//!
//! [`EnumStr`]: derive@enum_helper_derive::EnumStr
//! [`EnumAll`]: derive@enum_helper_derive::EnumAll
//! [`EnumKind`]: derive@enum_helper_derive::EnumKind

#[cfg(feature = "serde")]
pub mod serde;

#[cfg(feature = "derive")]
pub use enum_helper_derive::EnumAll;
#[cfg(feature = "derive")]
pub use enum_helper_derive::EnumKind;
#[cfg(feature = "derive")]
pub use enum_helper_derive::EnumStr;

/// Convert between enum and string.
///
/// Derive this trait on unit enums via [`#[derive(EnumStr)]`](derive@enum_helper_derive::EnumStr).
pub trait EnumStr: Sized {
    /// Returns the primary string name of this variant.
    fn as_name(&self) -> &'static str;

    /// Returns all string names and aliases of this variant.
    ///
    /// The first element is always the primary name (returned by [`as_name`](EnumStr::as_name)).
    /// Subsequent elements are aliases added via `alias_all` and `alias`.
    fn as_aliases(&self) -> &'static [&'static str];
}

/// Get an array of all variants of an enum.
///
/// Derive this trait on unit enums via [`#[derive(EnumAll)]`](derive@enum_helper_derive::EnumAll).
pub trait EnumAll: Sized {
    /// Array type that holds every variant.
    type All;

    /// An array containing all variants of the enum.
    const ALL: Self::All;
}

/// Get the kind of an enum variant.
///
/// Derive this trait on enums via [`#[derive(EnumKind)]`](derive@enum_helper_derive::EnumKind).
pub trait EnumKind: Sized {
    /// The unit kind enum representing the kinds of this enum.
    type Kind;

    /// Returns the kind of this variant.
    fn kind(&self) -> Self::Kind;
}
