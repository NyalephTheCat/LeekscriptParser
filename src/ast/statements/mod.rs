use nom::{bytes::complete::tag, combinator::{opt, map}, IResult};

use crate::ast::*;

#[derive(Debug, Clone)]
pub struct Semi(bool);

impl std::fmt::Display for Semi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 {
            write!(f, ";")
        } else {
            Ok(())
        }
    }
}

impl ParseInto for Semi {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        map(opt(tag(";")), |x: Option<_>| Semi(x.is_some()))(input)
    }
}

impl Semi {
    pub fn must_parse(input: Span) -> IResult<Span, Self> {
        map(tag(";"), |_| Semi(true))(input)
    }
}

pub mod expression_statement;
pub mod global_declaration_statement;
pub mod var_declaration;
pub mod function_definition;
pub mod include_statement;
pub mod if_statement;
pub mod do_while_statement;
pub mod while_statement;
pub mod for_statement;
pub mod return_statement;

pub mod statement;

pub use statement::*;
pub use expression_statement::*;
pub use global_declaration_statement::*;
pub use var_declaration::*;
pub use function_definition::*;
pub use include_statement::*;
pub use if_statement::*;
pub use do_while_statement::*;
pub use while_statement::*;
pub use for_statement::*;
pub use return_statement::*;