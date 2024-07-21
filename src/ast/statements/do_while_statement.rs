use nom::{sequence::{preceded, tuple}, combinator::map, IResult};
use derive_more::Display;
use crate::{ast::*, utils::kw};

#[derive(Debug, Clone, Display)]
#[display(fmt = "do{}while{}{}", statement, condition, semi)]
pub struct DoWhileStatement {
    pub statement: MetaNode<Statement>,
    pub condition: MetaNode<Expression>,
    pub semi: MetaNode<Semi>,
}

impl ParseInto for DoWhileStatement {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        map(
            tuple((
                preceded(kw("do"), Statement::parse),
                preceded(kw("while"), Expression::parse),
                Semi::parse,
            )),
            |(statement, condition, semi)| DoWhileStatement {
                statement,
                condition,
                semi,
            },
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test_remains_same;

    use super::*;

    #[test]
    fn test_do_while() {
        test_remains_same::<DoWhileStatement, _>("do { return; } while true;", "do { return; } while true;");
    }
}