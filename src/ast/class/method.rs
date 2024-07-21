use nom::{combinator::{opt, peek}, sequence::{terminated, tuple}, IResult};

use crate::ast::*;

#[derive(Debug, Clone)]
pub struct Method {
    pub visibility: MetaNode<Visibility>,
    pub return_type: Option<MetaNode<Type>>,
    pub name: MetaNode<Identifier>,
    pub parameters: MetaNode<Parameters>,
    pub body: MetaNode<BlockStatement>,
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.visibility)?;
        if let Some(return_type) = &self.return_type {
            write!(f, "{}", return_type)?;
        }
        write!(f, "{}", self.name)?;
        write!(f, "{}", self.parameters)?;
        write!(f, "{}", self.body)
    }
}

impl ParseInto for Method {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        let (input, (visibility, return_type, name, parameters, body)) = tuple((
            Visibility::parse,
            opt(terminated(Type::parse, peek(Identifier::parse))),
            Identifier::parse,
            Parameters::parse,
            BlockStatement::parse,
        ))(input)?;

        Ok((input, Method {
            visibility,
            return_type,
            name,
            parameters,
            body,
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test_remains_same;

    use super::*;

    #[test]
    fn test_method() {
        test_remains_same::<Method, _>("a() {}", "a() {}");
        test_remains_same::<Method, _>("Number a() {}", "Number a() {}");
        test_remains_same::<Method, _>("public Number a() {}", "public Number a() {}");
        test_remains_same::<Method, _>("public a() {}", "public a() {}");
        test_remains_same::<Method, _>("public a(Number a) {}", "public a(Number a) {}");
        test_remains_same::<Method, _>("public a(Number a, String b) {}", "public a(Number a, String b) {}");
        test_remains_same::<Method, _>("public a(Number a, String b) { return; }", "public a(Number a, String b) { return; }");
    }
}