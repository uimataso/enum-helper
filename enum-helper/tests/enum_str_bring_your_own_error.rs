use core::fmt;

use assert2::check;
use enum_helper::EnumStr;

#[derive(Debug, EnumStr, PartialEq, Eq)]
#[enum_str(error(name = YourOwnError, disable))]
enum Foo {
    Bar,
}

struct YourOwnError {
    input: String,
}

impl fmt::Display for YourOwnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "my own error, input: {}", self.input)
    }
}

impl YourOwnError {
    fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
        }
    }
}

#[test]
fn bring_your_own_error() {
    let res: Result<Foo, YourOwnError> = "test".parse();
    check!(let Err(YourOwnError { .. }) = res);
}
