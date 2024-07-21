use nom::{multi::many0, sequence::tuple};

use crate::ast::*;

#[derive(Debug, Clone)]
pub struct AssignExpr {
    pub left: Box<Expression>,
    pub right: Vec<(MetaNode<AssignOp>, MetaNode<Expression>)>,
}

impl Display for AssignExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.left)?;
        for (op, expr) in &self.right {
            write!(f, "{}{}", op, expr)?;
        }
        Ok(())
    }
}

impl ParseInto for AssignExpr
{
    type Output = Expression;

    fn parse_inner(input: Span) -> IResult<Span, Self::Output> {
        map(
            tuple((
                alt((
                    AnonymousFunction::parse_inner,
                    TernaryExpression::parse_inner,
                )),
                many0(tuple((AssignOp::parse, Expression::parse))),
            )),
            |(left, right)| match right.len() {
                0 => left,
                _ => {
                    Self::Output::from(AssignExpr {
                        left: Box::new(left),
                        right,
                    })
                }
            }
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test_remains_same;

    use super::*;

    #[test]
    fn test_assign() {
        test_remains_same::<AssignExpr, _>("a = 1", "a = 1");
    }
}