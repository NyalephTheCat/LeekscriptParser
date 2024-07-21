use nom::{combinator::map, sequence::pair, IResult};
use derive_more::Display;
use crate::ast::*;

#[derive(Debug, Clone, Display)]
#[display(fmt = "{}{}", expression, semi)]
pub struct ExpressionStatement {
    pub expression: MetaNode<Expression>,
    pub semi: MetaNode<Semi>,
}

impl ParseInto for ExpressionStatement {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        map(pair(Expression::parse, Semi::parse), |(expression, semi)| ExpressionStatement {
            expression,
            semi,
        })(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test_remains_same;

    use super::*;

    #[test]
    fn test_expression_statement() {
        test_remains_same::<ExpressionStatement, _>("1;", "1;");
        test_remains_same::<ExpressionStatement, _>("1 + 1;", "1 + 1;");
    }

    #[test]
    fn test_expression_statement_without_semi() {
        test_remains_same::<ExpressionStatement, _>("1", "1");
        test_remains_same::<ExpressionStatement, _>("1 + 1", "1 + 1");
    }
}