use derive_more::Display;

use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult};

use crate::ast::*;

#[derive(Debug, Clone, Display, PartialEq)]
pub enum Literal {
    String(StringLiteral),
    Number(NumberLiteral),
    Boolean(bool),
    #[display(fmt = "null")]
    Null,
}

impl ParseInto for Literal {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        alt((
            map(StringLiteral::parse_inner, Literal::String),
            map(NumberLiteral::parse_inner, Literal::Number),
            map(tag("true"), |_| Literal::Boolean(true)),
            map(tag("false"), |_| Literal::Boolean(false)),
            map(tag("null"), |_| Literal::Null),
        ))(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test_remains_same;

    use super::*;

    #[test]
    fn test_boolean() {
        test_remains_same::<Literal, _>("true", "true");
        test_remains_same::<Literal, _>("false", "false");
    }

    #[test]
    fn test_null() {
        test_remains_same::<Literal, _>("null", "null");
    }

    #[test]
    fn test_string() {
        test_remains_same::<Literal, _>("'hello'", "'hello'");
        test_remains_same::<Literal, _>("\"hello\"", "\"hello\"");
    }

    #[test]
    fn test_number() {
        test_remains_same::<Literal, _>("123", "123");
        test_remains_same::<Literal, _>("123.456", "123.456");
        test_remains_same::<Literal, _>(".456", ".456");
        test_remains_same::<Literal, _>("123.", "123.");
    }
}