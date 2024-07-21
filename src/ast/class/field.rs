use nom::{bytes::complete::tag, combinator::{map, opt, peek}, sequence::{preceded, terminated, tuple}, IResult};

use crate::ast::*;

#[derive(Debug, Clone)]
pub struct Field {
    pub visibility: MetaNode<Visibility>,
    pub type_: Option<MetaNode<Type>>,
    pub name: MetaNode<Identifier>,
    pub value: Option<MetaNode<Expression>>,
    pub semi: MetaNode<Semi>,
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.visibility)?;
        if let Some(type_) = &self.type_ {
            write!(f, "{}", type_)?;
        }
        write!(f, "{}", self.name)?;
        if let Some(value) = &self.value {
            write!(f, "={}", value)?;
        }
        write!(f, "{}", self.semi)?;
        Ok(())
    }
}

impl ParseInto for Field {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        map(
            tuple((
                Visibility::parse,
                opt(terminated(Type::parse, peek(Identifier::parse))),
                Identifier::parse,
                opt(preceded(tag("="), Expression::parse)),
                Semi::parse,
            )),
            |(visibility, type_, name, value, semi)| Field {
                visibility,
                type_,
                name,
                value,
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
    fn test_field() {
        test_remains_same::<Field, _>("public Number a;", "public Number a;");
        test_remains_same::<Field, _>("private Number a = 1;", "private Number a = 1;");
        test_remains_same::<Field, _>("protected Number a = 1;", "protected Number a = 1;");
        test_remains_same::<Field, _>("Number a;", "Number a;");
        test_remains_same::<Field, _>("Number a = 1;", "Number a = 1;");
        test_remains_same::<Field, _>("a;", "a;");
        test_remains_same::<Field, _>("a = 1;", "a = 1;");
        test_remains_same::<Field, _>("a = 1", "a = 1");
        test_remains_same::<Field, _>("static Number a", "static Number a");
    }
}