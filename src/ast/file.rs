use nom::{combinator::{eof, map}, multi::many0, sequence::{pair, terminated}, IResult};

use crate::ast::*;

#[derive(Debug, Clone)]
pub struct File {
    pub statements: Vec<MetaNode<GlobalStatement>>,
    pub eof: MetaNode<Empty>
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for statement in &self.statements {
            write!(f, "{}", statement)?;
        }
        write!(f, "{}", self.eof)
    }
}

impl ParseInto for File {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        map(
            terminated(pair(many0(GlobalStatement::parse), Empty::parse), eof),
            |(statements, eof)| File { statements, eof },
        )(input)
    }
}