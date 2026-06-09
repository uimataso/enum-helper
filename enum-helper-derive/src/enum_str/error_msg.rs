use crate::template::{TemplateSegment, parse_template};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorMsgVar {
    Name,
    Names(ListMod),
    Aliases(ListMod),
    Input,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ListMod {
    sep: Option<String>,
    quote: Option<String>,
}

#[derive(Debug)]
pub struct ParseTemplateVarError(String);

impl std::str::FromStr for ErrorMsgVar {
    type Err = ParseTemplateVarError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "name" => return Ok(Self::Name),
            "input" => return Ok(Self::Input),
            _ => {}
        }

        let v: Vec<_> = s.split(':').collect();
        if v.is_empty() {
            return Err(ParseTemplateVarError(s.to_string()));
        }

        let sep = v.get(1).map(|s| s.to_string());
        let quote = v.get(2).map(|s| s.to_string());
        let list_mod = ListMod { sep, quote };

        match v[0].trim() {
            "names" => Ok(Self::Names(list_mod)),
            "aliases" => Ok(Self::Aliases(list_mod)),
            _ => Err(ParseTemplateVarError(s.to_string())),
        }
    }
}

impl std::fmt::Display for ParseTemplateVarError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "unknown error msg template variable `{{{}}}`, expected one of `name`, `input`, `names[:sep[:quote]]`, or `aliases[:sep[:quote]]`",
            &self.0
        )
    }
}

pub fn default_error_msg() -> Vec<TemplateSegment<ErrorMsgVar>> {
    let s = "invalid {name}, expected one of {names}";
    parse_template(s).expect("hard coded error msg template")
}

impl ListMod {
    pub fn get(&self) -> (&str, &str) {
        match (&self.sep, &self.quote) {
            (None, None) => (", ", "\""),
            (None, Some(q)) => (", ", q),
            (Some(s), None) => (s, ""),
            (Some(s), Some(q)) => (s, q),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;

    #[test]
    fn error_msg_var_ok() {
        macro_rules! check_ok {
            ($input:literal, $expected:expr) => {
                let var: ErrorMsgVar = $input.parse().unwrap();
                check!(var == $expected);
            };
        }

        check_ok!("name", ErrorMsgVar::Name);
        check_ok!(" name ", ErrorMsgVar::Name);
        check_ok!(" input  ", ErrorMsgVar::Input);
        check_ok!("names", ErrorMsgVar::Names(ListMod::default()));
        check_ok!("aliases", ErrorMsgVar::Aliases(ListMod::default()));
        check_ok!(
            "names: :",
            ErrorMsgVar::Names(ListMod {
                sep: Some(" ".to_string()),
                quote: Some("".to_string())
            })
        );
        check_ok!(
            "names:, ",
            ErrorMsgVar::Names(ListMod {
                sep: Some(", ".to_string()),
                quote: None
            })
        );
        check_ok!(
            "names:|:`",
            ErrorMsgVar::Names(ListMod {
                sep: Some("|".to_string()),
                quote: Some("`".to_string())
            })
        );
        check_ok!(
            "names::",
            ErrorMsgVar::Names(ListMod {
                sep: Some("".to_string()),
                quote: Some("".to_string())
            })
        );
    }
}
