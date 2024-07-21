use std::fmt::Display;

use nom::{bytes::complete::tag, combinator::{map, opt}, multi::separated_list1, sequence::{delimited, pair}};
use object::Comma;

use crate::ast::*;

use super::super::{expression::Expression, MetaNode, ParseInto};

#[derive(Debug, Clone)]
pub enum Set {
    Empty(MetaNode<Empty>),
    Elements {
        elements: Vec<MetaNode<Expression>>,
        last_comma: Option<MetaNode<Comma>>,
    },
}

impl Display for Set {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Set::Empty(e) => write!(f, "<{}>", e),
            Set::Elements {
                elements,
                last_comma,
            } => {
                write!(f, "<")?;
                for (i, element) in elements.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{}", element)?;
                }
                if let Some(last_comma) = last_comma {
                    write!(f, "{}", last_comma)?;
                }

                write!(f, ">")
            }
        }
    }
}
impl ParseInto for Set {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        delimited(
            tag("<"), 
            alt((
                map(pair(
                    separated_list1(tag(","), Expression::parse),
                    opt(Comma::parse),
                ), |(values, last_comma)| {
                    Set::Elements {
                        elements: values,
                        last_comma,
                    }
                }),
                
                map(Empty::parse, Set::Empty),
            )),
            tag(">")
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test_remains_same;

    use super::*;

    #[test]
    fn test_empty() {
        test_remains_same::<Set, _>("<>", "<>");
        test_remains_same::<Set, _>("< >", "< >");
    }

    #[test]
    fn test_single() {
        test_remains_same::<Set, _>("<1>", "<1>");
        test_remains_same::<Set, _>("< 1 >", "< 1 >");
    }

    #[test]
    fn test_multiple() {
        test_remains_same::<Set, _>("<1, 2, 3>", "<1, 2, 3>");
        test_remains_same::<Set, _>("< 1, 2, 3 >", "< 1, 2, 3 >");
    }
}