use nom::{combinator::{map, opt}, sequence::tuple, IResult};
use derive_more::Display;
use crate::{ast::*, utils::kw};

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub expression: Option<MetaNode<Expression>>,
    pub semi: MetaNode<Semi>,
}

impl std::fmt::Display for ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "return")?;
        if let Some(expression) = &self.expression {
            write!(f, "{}", expression)?;
        }
        write!(f, "{}", self.semi)
    }
}

impl ParseInto for ReturnStatement {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        map(
            tuple((
                kw("return"),
                opt(Expression::parse),
                Semi::parse,
            )),
            |(_, expression, semi)| ReturnStatement {
                expression,
                semi,
            },
        )(input)
    }
}

#[derive(Debug, Clone, Display)]
pub enum BreakStatement {
    #[display(fmt = "break{}", _0)]
    Break(MetaNode<Semi>),
    #[display(fmt = "continue{}", _0)]
    Continue(MetaNode<Semi>),
}

impl ParseInto for BreakStatement {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        map(
            tuple((
                kw("break"),
                Semi::parse,
            )),
            |(_, semi)| BreakStatement::Break(semi),
        )(input)
        .or_else(|_| {
            map(
                tuple((
                    kw("continue"),
                    Semi::parse,
                )),
                |(_, semi)| BreakStatement::Continue(semi),
            )(input)
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test_remains_same;

    use super::*;

    #[test]
    fn test_return_statement() {
        test_remains_same::<ReturnStatement, _>("return;", "return;");
        test_remains_same::<ReturnStatement, _>("return 1;", "return 1;");
    }

    #[test]
    fn test_break_statement() {
        test_remains_same::<BreakStatement, _>("break;", "break;");
        test_remains_same::<BreakStatement, _>("continue;", "continue;");
    }
}