use derive_more::Display;
use crate::ast::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{delimited, pair, tuple},
    IResult,
};

#[derive(Debug, Clone, Display)]
#[display(fmt = ":")]
pub struct Colon;
impl ParseInto for Colon {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        map(tag(":"), |_| Colon)(input)
    }
}

#[derive(Debug, Clone, Display)]
#[display(fmt = ",")]
pub struct Comma;
impl ParseInto for Comma {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        map(tag(","), |_| Comma)(input)
    }
}

#[derive(Debug, Clone)]
pub enum Object {
    Empty(MetaNode<Empty>),
    Pairs {
        pairs: Vec<(MetaNode<Expression>, MetaNode<Expression>)>,
        last_comma: Option<MetaNode<Comma>>,
    },
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;

        match self {
            Object::Empty(e) => write!(f, "{}", e)?,
            Object::Pairs {
                pairs,
                last_comma,
            } => {
                for (i, (key, value)) in pairs.iter().enumerate() {
                    write!(f, "{}:{}", key, value)?;
                    if i < pairs.len() - 1 {
                        write!(f, ",")?;
                    }
                }
                if let Some(last_comma) = last_comma {
                    write!(f, "{}", last_comma)?;
                }
            }
        }

        write!(f, "}}")?;
        Ok(())
    }
}

impl ParseInto for Object {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        alt((
            map(delimited(tag("{"), Empty::parse, tag("}")), |e| {
                Object::Empty(e)
            }),
            map(
                delimited(
                    tag("{"),
                    pair(separated_list1(
                        tag(","),
                        tuple((Expression::parse, Colon::parse_inner, Expression::parse)),
                    ),
                    opt(Comma::parse),
                    ),
                    tag("}"),
                ),
                |(pairs, last_comma)| Object::Pairs {
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
        test_remains_same::<Object, _>("{}", "{}");
        test_remains_same::<Object, _>("{ }", "{ }");
    }

    #[test]
    fn test_single() {
        test_remains_same::<Object, _>("{a:1}", "{a:1}");
        test_remains_same::<Object, _>("{ a : 1 }", "{ a : 1 }");
    }

    #[test]
    fn test_multiple() {
        test_remains_same::<Object, _>("{a:1,b:2}", "{a:1,b:2}");
        test_remains_same::<Object, _>("{ a : 1 , b : 2 }", "{ a : 1 , b : 2 }");
    }
}