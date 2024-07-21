use nom::{bytes::complete::tag, combinator::{map, opt}, sequence::{delimited, preceded, tuple}, IResult};
use derive_more::{From, Display};
use crate::{ast::*, utils::kw};

#[derive(Debug, Clone, Display, From)]
#[display(fmt = "({})", expression)]
pub struct ParenthesizedExpression {
    pub expression: MetaNode<Expression>,
}

impl ParseInto for ParenthesizedExpression {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        map(
            delimited(
                tag("("),
                Expression::parse,
                tag(")"),
            ),
            ParenthesizedExpression::from
        )(input)
    }
}

#[derive(Debug, Clone)]
pub struct IfStatement {
    pub condition: MetaNode<ParenthesizedExpression>,
    pub block: MetaNode<Statement>,
    pub else_block: Option<MetaNode<Statement>>,
}

impl std::fmt::Display for IfStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "if{}{}", self.condition, self.block)?;
        if let Some(else_block) = &self.else_block {
            write!(f, "else{}", else_block)?;
        }
        Ok(())
    }
}

impl ParseInto for IfStatement {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        map(
            tuple((
                preceded(kw("if"), ParenthesizedExpression::parse),
                Statement::parse,
                opt(preceded(kw("else"), Statement::parse)),
            )),
            |(condition, block, else_block)| IfStatement {
                condition,
                block,
                else_block,
            },
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test_remains_same;

    use super::*;

    #[test]
    fn test_if_statement() {
        test_remains_same::<IfStatement, _>("if (a) { b; }", "if (a) { b; }");
        test_remains_same::<IfStatement, _>("if (a) { b; } else { c; }", "if (a) { b; } else { c; }");
    }

    #[test]
    fn test_if_statement_with_else_if() {
        test_remains_same::<IfStatement, _>("if (a) { b; } else if c { d; }", "if (a) { b; } else if c { d; }");
        test_remains_same::<IfStatement, _>("if (a) { b; } else if c { d; } else { e; }", "if (a) { b; } else if c { d; } else { e; }");
    }

    #[test]
    fn test_if_parenthesized_expression() {
        test_remains_same::<IfStatement, _>("if (a) (a + b)", "if (a) (a + b)");
    }
}