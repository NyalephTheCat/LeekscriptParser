use crate::{ast::*, utils::kw};
use derive_more::Display;
use function_definition::Arrow;
use nom::{
    branch::alt,
    combinator::{map, opt},
    sequence::pair,
    IResult,
};

#[derive(Debug, Clone, Display)]
pub enum AnonymousFunction {
    ArrowFunction(ArrowFunction),
    Function(AnonymousFuncDec),
}

impl ParseInto for AnonymousFunction {
    type Output = Expression;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        map(
            alt((
                map(ArrowFunction::parse_inner, AnonymousFunction::ArrowFunction),
                map(AnonymousFuncDec::parse_inner, AnonymousFunction::Function),
            )),
            |f| Expression::AnonyFunc(f),
        )(input)
    }
}

#[derive(Debug, Clone)]
pub struct ArrowFunction {
    pub args: MetaNode<AnonymousParameter>,
    pub arrow: MetaNode<Arrow>,
    pub return_type: Option<MetaNode<Type>>,
    pub body: MetaNode<FunctionBodyOrExpression>,
}

impl std::fmt::Display for ArrowFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            self.args,
            self.arrow,
            self.return_type
                .as_ref()
                .map_or("".to_string(), |t| format!("{}", t)),
            self.body
        )
    }
}

impl ParseInto for ArrowFunction {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        use nom::sequence::tuple;

        map(
            tuple((
                AnonymousParameter::parse,
                Arrow::parse,
                alt((
                    map(pair(Type::parse, FunctionBodyOrExpression::parse), |(t, f)| { (Some(t), f) }),
                    map(FunctionBodyOrExpression::parse, |f| (None, f)),
                ))
            )),
            |(args, arrow, (return_type, body))| ArrowFunction {
                args,
                arrow,
                return_type,
                body,
            },
        )(input)
    }
}

#[derive(Debug, Clone, Display)]
pub enum AnonymousParameter {
    SingleParam(MetaNode<Parameter>),
    Parameters(MetaNode<Parameters>),
}

impl ParseInto for AnonymousParameter {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        alt((
            map(Parameters::parse, AnonymousParameter::Parameters),
            map(Parameter::parse, AnonymousParameter::SingleParam),
        ))(input)
    }
}

#[derive(Debug, Clone, Display)]
pub enum FunctionBodyOrExpression {
    BlockStatement(MetaNode<BlockStatement>),
    Expression(MetaNode<Expression>),
}

impl ParseInto for FunctionBodyOrExpression {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        alt((
            map(
                BlockStatement::parse,
                FunctionBodyOrExpression::BlockStatement,
            ),
            map(Expression::parse, FunctionBodyOrExpression::Expression),
        ))(input)
    }
}

#[derive(Debug, Clone)]
pub struct AnonymousFuncDec {
    pub args: MetaNode<Parameters>,
    pub return_type: Option<(MetaNode<Arrow>, MetaNode<Type>)>,
    pub body: MetaNode<BlockStatement>,
}
impl std::fmt::Display for AnonymousFuncDec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "function{}", self.args)?;
        if let Some((arrow, return_type)) = &self.return_type {
            write!(f, "{}{}", arrow, return_type)?;
        }
        write!(f, "{}", self.body)
    }
}
impl ParseInto for AnonymousFuncDec {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        use nom::sequence::tuple;

        map(
            tuple((
                kw("function"),
                Parameters::parse,
                opt(pair(Arrow::parse, Type::parse)),
                BlockStatement::parse,
            )),
            |(_, args, return_type, body)| AnonymousFuncDec {
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
    fn test_arrow_one_arg() {
        test_remains_same::<AnonymousFunction, _>("(a) => a", "(a) => a");
        test_remains_same::<AnonymousFunction, _>("a => a", "a => a");
        test_remains_same::<AnonymousFunction, _>("a => { return 1 }", "a => { return 1 }");
    }

    #[test]
    fn test_arrow_no_args() {
        test_remains_same::<AnonymousFunction, _>("() => 1", "() => 1");
        test_remains_same::<AnonymousFunction, _>("() => { return 1 }", "() => { return 1 }");
    }

    #[test]
    fn test_arrow_multiple_args() {
        test_remains_same::<AnonymousFunction, _>("(a, b) => a", "(a, b) => a");
        test_remains_same::<AnonymousFunction, _>("(a, b) => { return a }", "(a, b) => { return a }");
    }

    #[test]
    fn test_arrow_typed() {
        test_remains_same::<AnonymousFunction, _>("String a => a", "String a => a");
        test_remains_same::<AnonymousFunction, _>("(String a) => a", "(String a) => a");
        test_remains_same::<AnonymousFunction, _>("(String a) => { return a }", "(String a) => { return a }");
        test_remains_same::<AnonymousFunction, _>("(String a) => String a", "(String a) => String a");
        
        test_remains_same::<AnonymousFunction, _>("(String a, String b) => a", "(String a, String b) => a");
        test_remains_same::<AnonymousFunction, _>("(String a, String b) => { return a }", "(String a, String b) => { return a }");
        test_remains_same::<AnonymousFunction, _>("(String a, String b) => String a", "(String a, String b) => String a");
    }

    #[test]
    fn test_function() {
        test_remains_same::<AnonymousFunction, _>("function(){}", "function(){}");
        test_remains_same::<AnonymousFunction, _>("function(a){}", "function(a){}");
        test_remains_same::<AnonymousFunction, _>("function(a, b){}", "function(a, b){}");
    }

    #[test]
    fn test_function_typed() {
        test_remains_same::<AnonymousFunction, _>("function() => String {}", "function() => String {}");
        test_remains_same::<AnonymousFunction, _>("function(a) => String {}", "function(a) => String {}");
        test_remains_same::<AnonymousFunction, _>("function(a, b) => String {}", "function(a, b) => String {}");
        test_remains_same::<AnonymousFunction, _>("function(String a, String b) => String {}", "function(String a, String b) => String {}");
    }
}