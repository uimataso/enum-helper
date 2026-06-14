//! Serde helpers for [`EnumStr`].
//!
//! Use with `#[serde(with = "...")]` on struct fields.
//!
//! ```
//! use enum_helper::EnumStr;
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(EnumStr)]
//! #[enum_str(rename_all = "lowercase")]
//! enum Foo { Bar, Baz }
//!
//! #[derive(Serialize, Deserialize)]
//! struct MyStruct {
//!     #[serde(with = "enum_helper::serde::enum_str")]
//!     foo: Foo,
//!     #[serde(with = "enum_helper::serde::option_enum_str")]
//!     maybe_foo: Option<Foo>,
//! }
//! ```

use std::str::FromStr;

use serde::{Deserialize as _, Deserializer, Serializer};

use crate::EnumStr;

/// Serde helper for serializing/deserializing an [`EnumStr`] type as a string.
///
/// Use with `#[serde(with = "enum_helper::serde::enum_str")]`.
pub mod enum_str {
    use super::*;

    /// Serializes an [`EnumStr`] value as a string.
    pub fn serialize<S, T>(t: &T, se: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: EnumStr,
    {
        se.serialize_str(t.as_name())
    }

    /// Deserializes a string into an [`EnumStr`] type via [`FromStr`].
    pub fn deserialize<'de, D, T>(de: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: FromStr,
        T::Err: std::error::Error,
    {
        // String is used instead of &str because zero-copy deserialization
        // is not always possible
        let s = String::deserialize(de)?;
        T::from_str(&s).map_err(serde::de::Error::custom)
    }
}

/// Serde helper for serializing/deserializing an `Option<impl EnumStr>` type.
///
/// Use with `#[serde(with = "enum_helper::serde::option_enum_str")]`.
pub mod option_enum_str {
    use super::*;

    /// Serializes an `Option<impl EnumStr>` as a string or `null`.
    pub fn serialize<S, T>(t: &Option<T>, se: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: EnumStr,
    {
        match t {
            Some(x) => se.serialize_str(x.as_name()),
            None => se.serialize_none(),
        }
    }

    /// Deserializes a string or `null` into an `Option<impl EnumStr>` via [`FromStr`].
    pub fn deserialize<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: FromStr,
        T::Err: std::error::Error,
    {
        let s = <Option<String>>::deserialize(de)?;
        s.map(|s| T::from_str(&s).map_err(serde::de::Error::custom))
            .transpose()
    }
}
