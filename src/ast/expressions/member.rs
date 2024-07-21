use derive_more::Display;
use nom::{branch::alt, bytes::complete::tag, combinator::map, multi::{many0, separated_list0}, sequence::{delimited, pair, preceded}, IResult};
use crate::{ast::*, utils::kw};
use nom::character::complete::char;

#[derive(Debug, Clone, Display)]
#[display(fmt = ".")]
pub struct Dot;
impl ParseInto for Dot {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        map(kw("."), |_| Dot)(input)
    }
}

#[derive(Debug, Clone, Display)]
pub enum IdentifierOrMember {
    Identifier(MetaNode<Identifier>),
    #[display(fmt = "class")]
    Class,
    #[display(fmt = "super")]
    Super,
    #[display(fmt = "this")]
    This,
}

impl ParseInto for IdentifierOrMember {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        alt((
            map(Identifier::parse, IdentifierOrMember::Identifier),
            map(kw("class"), |_| IdentifierOrMember::Class),
            map(kw("super"), |_| IdentifierOrMember::Super),
            map(kw("this"), |_| IdentifierOrMember::This),
        ))(input)
    }
}

#[derive(Debug, Clone)]
pub enum MemberRight {
    Dot(MetaNode<IdentifierOrMember>),
    Bracket(MetaNode<Expression>),
    Call(Vec<MetaNode<Expression>>),
    NotNull,
}

impl Display for MemberRight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemberRight::Dot(ident) => write!(f, ".{}", ident),
            MemberRight::Bracket(expr) => write!(f, "[{}]", expr),
            MemberRight::Call(args) => {
                write!(f, "(")?;
                
                for (i, arg) in args.iter().enumerate() {
                    write!(f, "{}", arg)?;
                    if i < args.len() - 1 {
                        write!(f, ",")?;
                    }
                }

                write!(f, ")")
            },
            MemberRight::NotNull => write!(f, "!"),
        }
    }
}

impl ParseInto for MemberRight {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        alt((
            map(preceded(char('.'), IdentifierOrMember::parse), MemberRight::Dot),
            map(delimited(
                char('['),
                Expression::parse,
                char(']'),
            ), MemberRight::Bracket),
            map(delimited(
                char('('),
                separated_list0(char(','), Expression::parse),
                char(')'),
            ), MemberRight::Call),
            map(tag("!"), |_| MemberRight::NotNull),
        ))(input)
    }
}

#[derive(Debug, Clone)]
pub struct Member {
    pub left: PrimaryExpr,
    pub right: Vec<MetaNode<MemberRight>>,
}

impl Display for Member {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.left)?;
        for right in &self.right {
            write!(f, "{}", right)?;
        }
        Ok(())
    }
}

impl ParseInto for Member {
    type Output = Expression;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        map(
            pair(
                PrimaryExpr::parse_inner,
                many0(MemberRight::parse),
            ),
            |(left, right)| match right.len() {
                0 => left.into(),
                _ => Expression::Member(Member {
                    left,
                    right,
                }),
            },
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test_remains_same;

    use super::*;

    #[test]
    fn test_dot() {
        test_remains_same::<Member, _>("a.b", "a.b");
        test_remains_same::<Member, _>("a.b.c", "a.b.c");
        test_remains_same::<Member, _>("a.b.c.d", "a.b.c.d");
    }

    #[test]
    fn test_access_with_keywords() {
        test_remains_same::<Member, _>("a.class", "a.class");
        test_remains_same::<Member, _>("a.super", "a.super");
        test_remains_same::<Member, _>("a.this", "a.this");
    }

    #[test]
    fn test_bracket() {
        test_remains_same::<Member, _>("a[1]", "a[1]");
        test_remains_same::<Member, _>("a[1][2]", "a[1][2]");
        test_remains_same::<Member, _>("a[1][2][3]", "a[1][2][3]");
    }

    #[test]
    fn test_call() {
        test_remains_same::<Member, _>("a()", "a()");
        test_remains_same::<Member, _>("a(1)", "a(1)");
        test_remains_same::<Member, _>("a(1, 2)", "a(1, 2)");
    }

    #[test]
    fn test_not_null() {
        test_remains_same::<Member, _>("a!", "a!");
        test_remains_same::<Member, _>("a.b!", "a.b!");
        test_remains_same::<Member, _>("a[1]!", "a[1]!");
        test_remains_same::<Member, _>("a(1, 2)!", "a(1, 2)!");
    }

    #[test]
    fn test_complex() {
        test_remains_same::<Member, _>("a.b[1].c(1, 2)!", "a.b[1].c(1, 2)!");
        test_remains_same::<Member, _>("a.b[1].c(1, 2)!.d", "a.b[1].c(1, 2)!.d");
    }

    #[test]
    fn test_no_member() {
        test_remains_same::<Member, _>("a", "a");
        test_remains_same::<Member, _>("1", "1");
        test_remains_same::<Member, _>("'hello'", "'hello'");
    }
}