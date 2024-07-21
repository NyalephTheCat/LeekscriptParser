use crate::{ast::*, utils::kw};
use derive_more::{Display, From};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt, peek},
    multi::separated_list1,
    sequence::{pair, preceded, terminated, tuple},
    IResult,
};

#[derive(Debug, Clone, From)]
pub struct VarType(Option<MetaNode<Type>>);

impl Display for VarType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(type_) = &self.0 {
            write!(f, "{}", type_)
        } else {
            write!(f, "var")
        }
    }
}

impl ParseInto for VarType {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        map(
            alt((
                map(terminated(Type::parse, peek(Identifier::parse)), Some),
                map(kw("var"), |_| None),
            )),
            VarType::from,
        )(input)
    }
}

#[derive(Debug, Clone)]
pub struct VarDeclaration {
    pub type_: MetaNode<VarType>,
    pub values: Vec<(MetaNode<Identifier>, Option<MetaNode<Expression>>)>,
}

impl Display for VarDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.type_)?;
        for (i, (name, value)) in self.values.iter().enumerate() {
            write!(f, "{}", name)?;

            if let Some(value) = value {
                write!(f, "={}", value)?;
            }

            if i < self.values.len() - 1 {
                write!(f, ",")?;
            }
        }
        Ok(())
    }
}

impl ParseInto for VarDeclaration {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        map(
            tuple((
                VarType::parse,
                separated_list1(
                    tag(","),
                    pair(
                        Identifier::parse,
                        opt(preceded(tag("="), Expression::parse)),
                    ),
                ),
            )),
            |(type_, values)| VarDeclaration { type_, values },
        )(input)
    }
}

#[derive(Debug, Clone, Display, From)]
#[display(fmt = "{}{}", var_declaration, semi)]
pub struct VarDeclarationStatement {
    pub var_declaration: MetaNode<VarDeclaration>,
    pub semi: MetaNode<Semi>,
}

impl ParseInto for VarDeclarationStatement {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        map(
            pair(VarDeclaration::parse, Semi::parse),
            VarDeclarationStatement::from,
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test_remains_same;

    use super::*;

    #[test]
    fn test_var_declaration() {
        test_remains_same::<VarDeclarationStatement, _>("var a;", "var a;");
        test_remains_same::<VarDeclarationStatement, _>("Number a;", "Number a;");
        test_remains_same::<VarDeclarationStatement, _>("var a=1;", "var a=1;");
        test_remains_same::<VarDeclarationStatement, _>("Number a=1;", "Number a=1;");
    }

    #[test]
    fn test_multiple_var_declaration() {
        test_remains_same::<VarDeclarationStatement, _>("var a,b;", "var a,b;");
        test_remains_same::<VarDeclarationStatement, _>("Number a,b;", "Number a,b;");
        test_remains_same::<VarDeclarationStatement, _>("var a,b=1;", "var a,b=1;");
        test_remains_same::<VarDeclarationStatement, _>("Number a,b=1;", "Number a,b=1;");
    }
}
