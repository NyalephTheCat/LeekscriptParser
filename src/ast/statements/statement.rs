use nom::{branch::alt, combinator::map, IResult};
use derive_more::Display;

use crate::ast::*;

#[derive(Debug, Clone, Display)]
pub enum Statement {
    ExpressionStatement(ExpressionStatement),
    VarDeclaration(VarDeclarationStatement),
    Block(BlockStatement),
    IfStatement(IfStatement),
    DoWhileStatement(DoWhileStatement),
    WhileStatement(WhileStatement),
    ForStatement(ForStatement),
    ReturnStatement(ReturnStatement),
    BreakStatement(BreakStatement),
    Empty(Semi),
}

impl ParseInto for Statement {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        alt((
            map(BlockStatement::parse_inner, Statement::Block),
            map(VarDeclarationStatement::parse_inner, Statement::VarDeclaration),
            map(IfStatement::parse_inner, Statement::IfStatement),
            map(DoWhileStatement::parse_inner, Statement::DoWhileStatement),
            map(WhileStatement::parse_inner, Statement::WhileStatement),
            map(ForStatement::parse_inner, Statement::ForStatement),
            map(ReturnStatement::parse_inner, Statement::ReturnStatement),
            map(BreakStatement::parse_inner, Statement::BreakStatement),
            map(ExpressionStatement::parse_inner, Statement::ExpressionStatement),
            map(Semi::must_parse, Statement::Empty),
        ))(input)
    }
}

#[derive(Debug, Clone, Display)]
pub enum GlobalStatement {
    IncludeStatement(IncludeStatement),
    Statement(Statement), // Normal statements
    GlobalDefinition(GlobalDefinition),
    FunctionDefinition(FunctionDefinition),
    ClassDefinition(Class),
}

impl ParseInto for GlobalStatement {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        alt((
            map(IncludeStatement::parse_inner, GlobalStatement::IncludeStatement),
            map(Class::parse_inner, GlobalStatement::ClassDefinition),
            map(GlobalDefinition::parse_inner, GlobalStatement::GlobalDefinition),
            map(Statement::parse_inner, GlobalStatement::Statement),
            map(FunctionDefinition::parse_inner, GlobalStatement::FunctionDefinition),
        ))(input)
    }
}