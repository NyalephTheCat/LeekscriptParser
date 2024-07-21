use derive_more::Display;
use nom::{combinator::{map, opt}, sequence::pair, IResult};
use crate::{ast::*, utils::kw};

#[derive(Debug, Clone, Display)]
#[display(fmt = "{}{}{}", expression, as_, type_)]
pub struct TypeConversionExpr {
    pub expression: Box<Expression>,
    pub as_: MetaNode<As>,
    pub type_: MetaNode<Type>,
}

impl ParseInto for TypeConversionExpr {
    type Output = Expression;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        map(
            pair(
                PreUpdateExpr::parse_inner,
                opt(pair(As::parse, Type::parse)),
            ),
            |(expression, type_)| match type_ {
                Some((as_, type_)) => Expression::TypeConversion(TypeConversionExpr {
                    expression: Box::new(expression),
                    as_,
                    type_,
                }),
                None => expression,
            },
    )(input)
    }
}

#[derive(Debug, Clone, Display)]
#[display(fmt = "as")]
pub struct As;

impl ParseInto for As {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        map(kw("as"), |_| As)(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test_remains_same;

    use super::*;

    #[test]
    fn test_type_conversion() {
        test_remains_same::<TypeConversionExpr, _>("1 as number", "1 as number");
        test_remains_same::<TypeConversionExpr, _>("1", "1");
    }
}