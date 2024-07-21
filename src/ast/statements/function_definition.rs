use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    multi::{many1, separated_list0},
    sequence::{delimited, pair},
    IResult,
};

use derive_more::Display;
use crate::{ast::*, utils::kw};

use super::MetaNode;

#[derive(Debug, Clone)]
pub enum BlockStatement {
    Empty(MetaNode<Empty>),
    StatementList(Vec<MetaNode<Statement>>),
}

impl Display for BlockStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockStatement::Empty(e) => write!(f, "{{{}}}", e),
            BlockStatement::StatementList(list) => {
                write!(f, "{{")?;
                for statement in list {
                    write!(f, "{}", statement)?;
                }
                write!(f, "}}")
            }
        }
    }
}

impl ParseInto for BlockStatement {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        delimited(
            tag("{"),
            alt((
                map(many1(Statement::parse), BlockStatement::StatementList),
                map(Empty::parse, BlockStatement::Empty),
            )),
            tag("}"),
        )(input)
    }
}

#[derive(Debug, Clone)]
pub struct Parameters {
    pub params: Vec<MetaNode<Parameter>>,
}
impl std::fmt::Display for Parameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.params.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(","))
    }
}
impl ParseInto for Parameters {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        map(
            delimited(
                tag("("),
                separated_list0(tag(","), Parameter::parse),
                tag(")"),
            ),
            |params| Parameters { params },
        )(input)
    }
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub type_: Option<MetaNode<Type>>,
    pub name: MetaNode<Identifier>,
}
impl std::fmt::Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(type_) = &self.type_ {
            write!(f, "{}", type_)?;
        }
        write!(f, "{}", self.name)
    }
}
impl ParseInto for Parameter {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        use nom::sequence::pair;

        alt((
            map(pair(Type::parse, Identifier::parse), |(type_, name)| Parameter {
                type_: Some(type_),
                name,
            }),
            map(Identifier::parse, |name| Parameter {
                type_: None,
                name,
            }),
        ))(input)
    }
}

#[derive(Debug, Clone, Display)]
pub struct Arrow(String);
impl ParseInto for Arrow {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        map(alt((tag("->"), tag("=>"))), |s:Span| Arrow(s.to_string()))(input)
    }
}

#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub name: MetaNode<Identifier>,
    pub args: MetaNode<Parameters>,
    pub return_type: Option<(MetaNode<Arrow>, MetaNode<Type>)>,
    pub body: MetaNode<BlockStatement>,
}
impl std::fmt::Display for FunctionDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "function{}{}{}{}",
            self.name,
            self.args,
            self.return_type
                .as_ref()
                .map_or("".to_string(), |(arrow, t)| format!("{}{}", arrow, t)),
            self.body
        )
    }
}
impl ParseInto for FunctionDefinition {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        use nom::sequence::tuple;
        map(
            tuple((
                kw("function"),
                Identifier::parse,
                Parameters::parse,
                opt(pair(Arrow::parse, Type::parse)),
                BlockStatement::parse,
            )),
            |(_, name, args, return_type, body)| FunctionDefinition {
                name,
                args,
                return_type,
                body,
            },
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test_remains_same;

    use super::*;

    #[test]
    fn test_empty() {
        test_remains_same::<BlockStatement, _>("{}", "{}");
        test_remains_same::<BlockStatement, _>("{ }", "{ }");
    }

    #[test]
    fn test_single() {
        test_remains_same::<BlockStatement, _>("{1}", "{1}");
    }

    #[test]
    fn test_multiple() {
        test_remains_same::<BlockStatement, _>("{1;2}", "{1;2}");
    }

    #[test]
    fn test_params() {
        test_remains_same::<Parameters, _>("()", "()");
        test_remains_same::<Parameters, _>("(a)", "(a)");
        test_remains_same::<Parameters, _>("(a, b)", "(a, b)");
    }

    #[test]
    fn test_params_with_type() {
        test_remains_same::<Parameter, _>("String a", "String a");
    }

    #[test]
    fn test_arrow() {
        test_remains_same::<Arrow, _>("->", "->");
        test_remains_same::<Arrow, _>("=>", "=>");
    }

    #[test]
    fn test_function() {
        test_remains_same::<FunctionDefinition, _>("function a(){}", "function a(){}");
        test_remains_same::<FunctionDefinition, _>("function a() -> String {}", "function a() -> String {}");
    }
}