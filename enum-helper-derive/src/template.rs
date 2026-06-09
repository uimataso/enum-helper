use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, PartialEq)]
pub enum TemplateSegment<V: FromStr> {
    Lit(String),
    Var(V),
}

pub fn parse_template<V>(s: &str) -> Result<Vec<TemplateSegment<V>>, String>
where
    V: FromStr,
    V::Err: Display,
{
    let mut segments = Vec::new();
    let mut start = 0;

    while let Some(bracket) = s[start..].find(['{', '}']) {
        let bracket_idx = start + bracket;

        // push the stuffs before bracket
        let lit = s[start..bracket_idx].to_string();
        if !lit.is_empty() {
            segments.push(TemplateSegment::Lit(lit));
        }
        start += bracket;

        match (
            s.get(start..=start).unwrap_or(""),
            s.get(start + 1..=start + 1).unwrap_or(""),
        ) {
            ("{", "{") => {
                segments.push(TemplateSegment::Lit("{".to_string()));
                start += 2;
            }
            ("{", _) => match s[start + 1..].find('}') {
                Some(j) => {
                    let close_idx = start + 1 + j;
                    let var_lit = &s[start + 1..close_idx];
                    let var = V::from_str(var_lit).map_err(|e| e.to_string())?;
                    segments.push(TemplateSegment::Var(var));
                    start = close_idx + 1;
                }
                None => {
                    let msg = "unclosed `{` in error message template";
                    return Err(msg.to_string());
                }
            },
            ("}", "}") => {
                segments.push(TemplateSegment::Lit("}".to_string()));
                start += 2;
            }
            ("}", _) => {
                let msg = "unmatched `}` in error message template";
                return Err(msg.to_string());
            }
            _ => unreachable!(),
        }
    }

    // push remaining literal
    if start < s.len() {
        segments.push(TemplateSegment::Lit(s[start..].to_string()));
    }

    Ok(segments)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;

    #[test]
    fn template_ok() {
        use TemplateSegment as Seg;

        macro_rules! check_ok {
            ($input:literal, $expected:expr) => {
                let segs = parse_template::<String>($input).unwrap();
                check!(segs == $expected);
            };
        }

        check_ok!(
            "hello world",
            vec![TemplateSegment::Lit("hello world".to_string())]
        );
        check_ok!("", vec![]);
        check_ok!("{name}", vec![Seg::Var("name".to_string())]);
        check_ok!("{ name }", vec![Seg::Var(" name ".to_string())]);
        check_ok!(
            "hello {name}!",
            vec![
                Seg::Lit("hello ".to_string()),
                Seg::Var("name".to_string()),
                Seg::Lit("!".to_string())
            ]
        );
        check_ok!(
            "before{{after",
            vec![
                Seg::Lit("before".to_string()),
                Seg::Lit("{".to_string()),
                Seg::Lit("after".to_string())
            ]
        );
        check_ok!(
            "{{}}",
            vec![Seg::Lit("{".to_string()), Seg::Lit("}".to_string())]
        );
    }

    #[test]
    fn template_err() {
        macro_rules! check_err {
            ($input:literal) => {
                parse_template::<String>($input).unwrap_err();
            };
        }

        check_err!("hello {");
        check_err!("hello }");
        check_err!("}");
    }
}
