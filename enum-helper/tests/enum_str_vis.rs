use assert2::check;

mod inner {
    use enum_helper::EnumStr;

    #[derive(EnumStr, PartialEq, Eq)]
    pub enum PubInner {
        Foo,
    }
}

#[test]
fn pub_inner_error_vis() {
    let error = inner::InvalidPubInner::new("input");
    check!(error.to_string() == "invalid PubInner, expected one of \"Foo\"");
}
