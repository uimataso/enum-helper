use assert2::check;
use enum_helper::EnumStr;

macro_rules! test_error_msg {
    ($name:ident, $expect:literal) => {
        let res = "input".parse::<$name>();
        check!(res.unwrap_err().to_string() == $expect);
    };
}

macro_rules! test_default_error_msg {
    ($name:ident, $expect:literal, $(#[$attr:meta])*) => {
        #[derive(Debug, EnumStr, PartialEq, Eq)]
        $(#[$attr])*
        enum $name {
            #[enum_str(alias = "foo_bar")]
            FooBar,
            Baz,
        }
        test_error_msg!($name, $expect);
    }
}

macro_rules! test_custom_error_msg {
    ($name:ident, $msg:literal, $expect:literal) => {
        #[derive(Debug, EnumStr, PartialEq, Eq)]
        #[enum_str(error_msg = $msg)]
        enum $name {
            #[enum_str(alias = "foo_bar")]
            FooBar,
            Baz,
        }
        test_error_msg!($name, $expect);
    };
}

#[test]
fn default_error_msg() {
    test_default_error_msg!(
        DefaultErrorMsg,
        "invalid DefaultErrorMsg, expected one of \"FooBar\", \"Baz\"",
    );
}

#[test]
fn error_msg() {
    test_custom_error_msg!(LitErrorMsg, "lit error msg", "lit error msg");
    test_custom_error_msg!(
        ErrorMsgWithVar,
        "name: {name}, names: {names}, aliases: {aliases}, input: {input}",
        "name: ErrorMsgWithVar, names: \"FooBar\", \"Baz\", aliases: \"FooBar\", \"foo_bar\", \"Baz\", input: input"
    );

    test_custom_error_msg!(ErrorMsgListSepSpace, "{names: }", "FooBar Baz");
    test_custom_error_msg!(ErrorMsgListSepQuote, "{names: :`}", "`FooBar` `Baz`");
}
