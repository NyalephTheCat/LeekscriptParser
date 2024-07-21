use crate::{ast::*, utils::kw};
use nom::{branch::alt, bytes::complete::tag, combinator::{map, opt}, sequence::{delimited, preceded, terminated, tuple}, IResult};
use derive_more::Display;

#[derive(Debug, Clone, Display)]
#[display(fmt = "for{}{}", for_header, block)]
pub struct ForStatement {
    pub for_header: MetaNode<ForHeader>,
    pub block: MetaNode<Statement>,
}

impl ParseInto for ForStatement {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        let (input, for_statement) = map(
            tuple((
                preceded(kw("for"), ForHeader::parse),
                Statement::parse,
            )),
            |(for_header, block)| ForStatement {
                for_header,
                block,
            },
        )(input)?;

        Ok((input, for_statement))
    }
}

#[derive(Debug, Clone, Display)]
#[display(fmt = "({})")]
pub enum ForHeader {
    ForIter(ForIter),
    ForIn(ForIn),
}

impl ParseInto for ForHeader {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        delimited(
            tag("("), 
            alt((
                map(ForIter::parse_inner, ForHeader::ForIter),
                map(ForIn::parse_inner, ForHeader::ForIn),
            )), 
            tag(")")
        )(input)
    }
}

#[derive(Debug, Clone)]
pub enum VarDecOrExpr {
    VarDeclaration(VarDeclaration),
    Expression(Expression),
}

impl std::fmt::Display for VarDecOrExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VarDecOrExpr::VarDeclaration(a) => write!(f, "{}", a),
            VarDecOrExpr::Expression(a) => write!(f, "{}", a),
        }
    }
}

impl ParseInto for VarDecOrExpr {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        alt((
            map(VarDeclaration::parse_inner, VarDecOrExpr::VarDeclaration),
            map(Expression::parse_inner, VarDecOrExpr::Expression),
        ))(input)
    }
}

#[derive(Debug, Clone)]
pub struct ForIter {
    pub init: Option<MetaNode<VarDecOrExpr>>,
    pub condition: Option<MetaNode<Expression>>,
    pub increment: Option<MetaNode<Expression>>,
}

impl std::fmt::Display for ForIter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let init = match &self.init {
            Some(init) => format!("{}", init),
            None => "".to_string(),
        };
        let condition = match &self.condition {
            Some(condition) => format!("{}", condition),
            None => "".to_string(),
        };
        let increment = match &self.increment {
            Some(increment) => format!("{}", increment),
            None => "".to_string(),
        };
        write!(f, "{};{};{}", init, condition, increment)
    }
}

impl ParseInto for ForIter {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        map(
            tuple((
                opt(VarDecOrExpr::parse),
                preceded(tag(";"), opt(Expression::parse)),
                preceded(tag(";"), opt(Expression::parse)),
            )),
            |(init, condition, increment)| ForIter {
                init,
                condition,
                increment,
            },
        )(input)
    }
}

#[derive(Debug, Clone)]
pub struct ForIn {
    pub key: Option<MetaNode<VarDeclaration>>,
    pub var: MetaNode<VarDeclaration>,
    pub iterable: MetaNode<Expression>,
}

impl std::fmt::Display for ForIn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(key) = &self.key {
            write!(f, "{}:", key)?;
        }
        write!(f, "{}in{}", self.var, self.iterable)
    }
}

impl ParseInto for ForIn {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        map(
            tuple((
                opt(terminated(VarDeclaration::parse, tag(":"))),
                VarDeclaration::parse,
                kw("in"),
                Expression::parse,
            )),
            |(key, var, _, iterable)| ForIn {
                key,
                var,
                iterable,
            },
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test_remains_same;

    use super::*;

    #[test]
    fn test_for_in_statement() {
        test_remains_same::<ForStatement, _>("for(var a in b) {}", "for(var a in b) {}");
        test_remains_same::<ForStatement, _>("for(var a: var b in c) { var b = 2; }", "for(var a: var b in c) { var b = 2; }");
    }

    #[test]
    fn test_for_statement() {
        test_remains_same::<ForStatement, _>("for(;;) {}", "for(;;) {}");
        test_remains_same::<ForStatement, _>("for(var a = 1;;) {}", "for(var a = 1;;) {}");
        test_remains_same::<ForStatement, _>("for(var a = 1; a < 10;) {}", "for(var a = 1; a < 10;) {}");
        test_remains_same::<ForStatement, _>("for(var a = 1; a < 10; a++) {}", "for(var a = 1; a < 10; a++) {}");
        test_remains_same::<ForStatement, _>("for(var a = 1; a < 10; a++) { var b = 2; }", "for(var a = 1; a < 10; a++) { var b = 2; }");

        test_remains_same::<ForStatement, _>("for(var a in b) {}", "for(var a in b) {}");
        test_remains_same::<ForStatement, _>("for(var a: var b in c) { var b = 2; }", "for(var a: var b in c) { var b = 2; }");
    }
}