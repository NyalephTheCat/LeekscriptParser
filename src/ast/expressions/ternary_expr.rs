use derive_more::Display;
use nom::{branch::alt, combinator::map, sequence::{preceded, tuple}, IResult};
use crate::{ast::*, utils::kw};

#[derive(Debug, Clone, Display)]
#[display(fmt = "{}?{}:{}", condition, true_expr, false_expr)]
pub struct TernaryExpression {
    pub condition: MetaNode<Expression>,
    pub true_expr: MetaNode<Expression>,
    pub false_expr: MetaNode<Expression>,
}

impl ParseInto for TernaryExpression {
    type Output = Expression;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        alt((
            map(
                tuple((
                    LogicalOr::parse,
                    preceded(kw("?"), Expression::parse),
                    preceded(kw(":"), Expression::parse),
                )),
                |(condition, true_expr, false_expr)| Expression::TernaryExpression(TernaryExpression {
                    condition,
                    true_expr,
                    false_expr,
                }),
            ),
            LogicalOr::parse_inner,
        ))(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test_remains_same;

    use super::*;

    #[test]
    fn test_ternary() {
        test_remains_same::<TernaryExpression, _>("1 ? 2 : 3", "1 ? 2 : 3");
        test_remains_same::<TernaryExpression, _>("1", "1");
    }
}