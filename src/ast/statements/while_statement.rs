use nom::{sequence::{preceded, tuple}, combinator::map, IResult};
use crate::{ast::*, utils::kw};
use derive_more::Display;

#[derive(Debug, Clone, Display)]
#[display(fmt = "while{}{}", condition, block)]
pub struct WhileStatement {
    pub condition: MetaNode<Expression>,
    pub block: MetaNode<Statement>,
}

impl ParseInto for WhileStatement {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        map(
            tuple((
                preceded(kw("while"), Expression::parse),
                Statement::parse,
            )),
            |(condition, block)| WhileStatement {
                condition,
                block,
            },
        )(input)
    }
}