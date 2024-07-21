use nom::{branch::alt, combinator::map, IResult};
use derive_more::Display;
use crate::ast::*;

#[derive(Debug, Clone, Display)]
pub enum ClassMember {
    Constructor(Constructor),
    Method(Method),
    Field(Field),
}

impl ParseInto for ClassMember {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        alt((
            map(Constructor::parse_inner, ClassMember::Constructor),
            map(Method::parse_inner, ClassMember::Method),
            map(Field::parse_inner, ClassMember::Field),
        ))(input)
    }
}