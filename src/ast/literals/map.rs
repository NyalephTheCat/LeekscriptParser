use std::fmt::Display;

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{delimited, pair, tuple},
    IResult,
};
use object::{Colon, Comma};

use crate::ast::*;

#[derive(Debug, Clone)]
pub enum Map {
    Empty(MetaNode<Colon>),
    Pairs {
        pairs: Vec<(MetaNode<Expression>, MetaNode<Expression>)>,
        last_comma: Option<MetaNode<Comma>>,
    },
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Map::Empty(e) => write!(f, "[{}]", e),
            Map::Pairs { pairs, last_comma } => {
                write!(f, "[")?;
                for (i, (key, value)) in pairs.iter().enumerate() {
                    write!(f, "{}:{}", key, value)?;
                    if i < pairs.len() - 1 {
                        write!(f, ",")?;
                    }
                }
                if let Some(last_comma) = last_comma {
                    write!(f, "{}", last_comma)?;
                }
                write!(f, "]")?;
                Ok(())
            }
        }
    }
}

impl ParseInto for Map {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        alt((
            map(delimited(tag("["), Colon::parse, tag("]")), |e| {
                Map::Empty(e)
            }),
            map(
                delimited(
                    tag("["),
                    pair(
                        separated_list1(
                            tag(","),
                            tuple((Expression::parse, Colon::parse_inner, Expression::parse)),
                        ),
                        opt(Comma::parse),
                    ),
                    tag("]"),
                ),
                |(pairs, last_comma)| Map::Pairs {
                    pairs: pairs.into_iter().map(|(k, _, v)| (k, v)).collect(),
                    last_comma,
                },
            ),
        ))(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test_remains_same;

    use super::*;

    #[test]
    fn test_empty() {
        test_remains_same::<Map, _>("[:]", "[:]");
        test_remains_same::<Map, _>("[ : ]", "[ : ]");
    }

    #[test]
    fn test_single() {
        test_remains_same::<Map, _>("[1:2]", "[1:2]");
        test_remains_same::<Map, _>("[ 1 : 2 ]", "[ 1 : 2 ]");

        test_remains_same::<Map, _>("[\"a\": 2]", "[\"a\": 2]");
        test_remains_same::<Map, _>("[ a : 2 ]", "[ a : 2 ]");

        test_remains_same::<Map, _>("[\"a\": 2,]", "[\"a\": 2,]");
    }

    #[test]
    fn test_multiple() {
        test_remains_same::<Map, _>("[1:2, 3:4]", "[1:2, 3:4]");
        test_remains_same::<Map, _>("[ 1 : 2 , 3 : 4 ]", "[ 1 : 2 , 3 : 4 ]");

        test_remains_same::<Map, _>("[\"a\": 2, \"b\": 3,]", "[\"a\": 2, \"b\": 3,]");
        test_remains_same::<Map, _>("[ a : 2 , b : 3, ]", "[ a : 2 , b : 3, ]");
    }
}
