use assert2::check;
use core::{convert::TryFrom, fmt};
use enum_helper::EnumStr;
use std::str::FromStr;

// no_rendering: derive must not generate as_name/as_aliases/Into<&str>/AsRef<str>.
// We provide our own impls and confirm there is no conflict.
#[derive(Debug, EnumStr, PartialEq, Eq)]
#[enum_str(no_rendering)]
enum NoRendering {
    Alpha,
    Bravo,
}

#[test]
fn no_rendering_has_from_str() {
    check!("Alpha".parse::<NoRendering>().unwrap() == NoRendering::Alpha);
    check!("Bravo".parse::<NoRendering>().unwrap() == NoRendering::Bravo);
}

#[test]
fn no_rendering_has_try_from() {
    check!(NoRendering::try_from("Alpha").unwrap() == NoRendering::Alpha);
}

#[allow(dead_code)]
impl NoRendering {
    fn as_name(&self) -> &'static str {
        "test"
    }

    fn as_aliases(&self) -> &'static [&'static str] {
        &["test"]
    }
}

impl From<NoRendering> for &'static str {
    fn from(_: NoRendering) -> Self {
        "test"
    }
}

impl AsRef<str> for NoRendering {
    fn as_ref(&self) -> &str {
        "test"
    }
}

impl fmt::Display for NoRendering {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "test")
    }
}

// no_parsing: derive must not generate FromStr/TryFrom/error struct.
// We provide our own and confirm there is no conflict.
#[derive(EnumStr, PartialEq, Eq)]
#[enum_str(no_parsing)]
enum NoParsing {
    Alpha,
    Bravo,
}

#[test]
fn no_parsing_has_as_name() {
    check!(NoParsing::Alpha.as_name() == "Alpha");
    check!(NoParsing::Bravo.as_name() == "Bravo");
}

#[derive(Debug, Clone)]
struct NoParsingError {}
impl fmt::Display for NoParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "test")
    }
}

impl FromStr for NoParsing {
    type Err = NoParsingError;

    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Ok(Self::Alpha)
    }
}

impl TryFrom<&str> for NoParsing {
    type Error = NoParsingError;

    fn try_from(_: &str) -> Result<Self, Self::Error> {
        Ok(Self::Alpha)
    }
}

// error(disable): derive must not generate the error struct, but still
// generates FromStr using the (user-provided) error type.
#[derive(Debug, PartialEq, Eq, EnumStr)]
#[enum_str(error(disable))]
enum NoErrorStruct {
    Alpha,
}

#[derive(Debug, Clone)]
struct InvalidNoErrorStruct;

impl fmt::Display for InvalidNoErrorStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "test")
    }
}

impl InvalidNoErrorStruct {
    fn new(_: &str) -> Self {
        Self
    }
}

#[test]
fn no_error_struct_has_from_str() {
    let res: Result<NoErrorStruct, InvalidNoErrorStruct> = "test".parse();
    check!(let Err(InvalidNoErrorStruct) = res);
}
