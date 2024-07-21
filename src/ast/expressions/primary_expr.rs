use derive_more::Display;
use nom::{branch::alt, bytes::complete::tag, combinator::map, sequence::delimited, IResult};
use crate::ast::*;

#[derive(Debug, Clone, Display)]
pub enum PrimaryExpr {
    IdentifierOrMember(MetaNode<IdentifierOrMember>),
    Literal(Literal),

    Array(Array),
    Object(Object),
    Set(Set),
    Map(Map),
    // TODO: Add intervals

    #[display(fmt = "({})", _0)]
    ParenthesizedExpr(MetaNode<Expression>),
}

impl ParseInto for PrimaryExpr {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        alt((
            map(delimited(
                tag("("),
                Expression::parse,
                tag(")"),
            ), PrimaryExpr::ParenthesizedExpr),

            map(Array::parse_inner, PrimaryExpr::Array),
            map(Object::parse_inner, PrimaryExpr::Object),
            map(Set::parse_inner, PrimaryExpr::Set),
            map(Map::parse_inner, PrimaryExpr::Map),

            map(IdentifierOrMember::parse, PrimaryExpr::IdentifierOrMember),
            map(Literal::parse_inner, PrimaryExpr::Literal),
        ))(input)
    }
}


#[cfg(test)]
mod tests {
    use crate::utils::test_remains_same;

    use super::*;

    #[test]
    fn test_primary_expr() {
        test_remains_same::<PrimaryExpr, _>("1", "1");
        test_remains_same::<PrimaryExpr, _>("'hello'", "'hello'");
        test_remains_same::<PrimaryExpr, _>("a", "a");
        test_remains_same::<PrimaryExpr, _>("[]", "[]");
        test_remains_same::<PrimaryExpr, _>("{}", "{}");
        test_remains_same::<PrimaryExpr, _>("this", "this");
        test_remains_same::<PrimaryExpr, _>("super", "super");
    }

    #[test]
    fn test_parenthesized_expr() {
        test_remains_same::<PrimaryExpr, _>("(1)", "(1)");
        test_remains_same::<PrimaryExpr, _>("(1 + 2)", "(1 + 2)");
    }
}