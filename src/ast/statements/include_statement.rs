use nom::{combinator::map, sequence::{delimited, tuple}, IResult};
use derive_more::Display;
use crate::{ast::*, utils::kw};

#[derive(Debug, Clone, Display)]
#[display(fmt = "{}({}){}", include_kw, path, semi)]
pub struct IncludeStatement {
    pub include_kw: MetaNode<IncludeKw>,
    pub path: MetaNode<StringLiteral>,
    pub semi: MetaNode<Semi>,
}

impl ParseInto for IncludeStatement {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        map(tuple((
            IncludeKw::parse, 
            delimited(
                kw("("),
                StringLiteral::parse,
                kw(")"),
            ),
            Semi::parse
        )), |(include_kw, path, semi)| {
            IncludeStatement {
                include_kw,
                path, semi
            }
        })(input)
    }
}

#[derive(Debug, Clone, Display)]
#[display(fmt = "include")]
pub struct IncludeKw;

impl ParseInto for IncludeKw {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        map(kw("include"), |_| IncludeKw)(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test_remains_same;

    use super::*;

    #[test]
    fn test_include_statement() {
        test_remains_same::<IncludeStatement, _>("include(\"file\");", "include(\"file\");");
    }
}