use crate::ast::*;
use crate::utils::kw;
use derive_builder::Builder;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1},
    combinator::{map, not, recognize},
    multi::many0,
    sequence::tuple,
    IResult,
};
use derive_more::Display;

const KEYWORDS: [&str; 46] = [
    "true", "false", "null", "undefined",
    "not", "and", "or", "is", "in", "as",
    "if", "else", "elif",
    "switch", "case", "default",
    "for", "do", "while", "break", "continue",
    "function", "return", "yield",
    "raise", "try", "except", "finally",
    "import", "include", "as", "with",
    "global", "var", "const", "let", "static",
    "class", "extends", "implements", "async", "await",
    "public", "private", "protected", "abstract",
];

#[derive(Debug, Clone, Display, PartialEq, Builder)]
#[builder(setter(into))]
#[display(fmt = "{}", name)]
pub struct Identifier {
    pub name: String,
}

impl ParseInto for Identifier {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        let is_kw = |s: Span<'a>| KEYWORDS.iter()
            .find_map(|&keyword| {
                let out = kw(keyword)(s);
                if out.is_ok() {
                    Some(out)
                } else {
                    None
                }
            }).ok_or(nom::Err::Error(nom::error::Error::new(s, nom::error::ErrorKind::Tag)))?;

        let (input, name) = map(
            recognize(tuple((
                not(is_kw),
                alt((alpha1, tag("_"))),
                many0(alt((alphanumeric1, tag("_")))),
            ))),
            |s: Span| s.fragment().to_string(),
        )(input)?;

        Ok((input, Identifier { name }))
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test_remains_same;

    use super::*;

    #[test]
    fn basic_identifiers() {
        vec![
            "hello",
            "foo_bar",
            "fooBar",
            "FooBar",
            "foo1",
            "foo_1",
            "foo_",
            "_foo",
            "_1",
        ].iter().for_each(|input| {
            test_remains_same::<Identifier, _>(input, input);
        });
    }

    #[test]
    fn keyword_in_identifier() {
        for keyword in KEYWORDS.iter() {
            let input = format!("{}foo", keyword);
            test_remains_same::<Identifier, _>(&input, &input);

            let input = format!("foo{}", keyword);
            test_remains_same::<Identifier, _>(&input, &input);

            let input = format!("foo{}bar", keyword);
            test_remains_same::<Identifier, _>(&input, &input);
        }
    }

    #[test]
    #[should_panic]
    fn exact_keyword() {
        test_remains_same::<Identifier, _>("true", "true");
    }
}