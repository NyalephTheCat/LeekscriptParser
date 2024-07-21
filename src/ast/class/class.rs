use nom::{branch::alt, bytes::complete::tag, combinator::{map, opt}, multi::many1, sequence::{delimited, preceded, tuple}, IResult};

use crate::{ast::*, utils::kw};

#[derive(Debug, Clone)]
pub struct Class {
    pub name: MetaNode<Identifier>,
    pub extends: Option<MetaNode<Identifier>>,
    pub body: MetaNode<ClassBody>,
}

impl std::fmt::Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "class{}", self.name)?;
        if let Some(extends) = &self.extends {
            write!(f, "extends{}", extends)?;
        }
        write!(f, "{}", self.body)
    }
}

impl ParseInto for Class {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        map(
            tuple((
                preceded(kw("class"), Identifier::parse),
                opt(preceded(kw("extends"), Identifier::parse)),
                ClassBody::parse,
            )),
            |(name, extends, body)| Class {
                name,
                extends,
                body,
            },
        )(input)
    }
}

#[derive(Debug, Clone)]
pub enum ClassBody {
    Empty(MetaNode<Empty>), // Holds any whitespace or comments in an empty class body
    Members(Vec<MetaNode<ClassMember>>),
}

impl std::fmt::Display for ClassBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClassBody::Empty(e) => write!(f, "{{{}}}", e),
            ClassBody::Members(members) => {
                write!(f, "{{")?;
                for member in members {
                    write!(f, "{}", member)?;
                }
                write!(f, "}}")
            }
        }
    }
}

impl ParseInto for ClassBody {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        delimited(
            tag("{"),
            alt((
                map(many1(ClassMember::parse), ClassBody::Members),
                map(Empty::parse, ClassBody::Empty),
            )),
            tag("}"),
        )(input)
    }
}

#[derive(Debug, Clone)]
pub struct Empty;
impl Display for Empty {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
impl ParseInto for Empty {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        Ok((input, Empty))
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test_remains_same;

    use super::*;

    #[test]
    fn test_empty() {
        test_remains_same::<Class, _>("class A {}", "class A {}");
    }

    #[test]
    fn test_members() {
        test_remains_same::<Class, _>(
            "class A { a = 1; b = 2; }",
            "class A { a = 1; b = 2; }",
        );
    }

    #[test]
    fn test_extends() {
        test_remains_same::<Class, _>("class A extends B {}", "class A extends B {}");
    }

    #[test]
    fn test_members_extends() {
        test_remains_same::<Class, _>(
            "class A extends B { a = 1; b = 2; }",
            "class A extends B { a = 1; b = 2; }",
        );
    }

    #[test]
    fn test_empty_extends() {
        test_remains_same::<Class, _>("class A extends B {}", "class A extends B {}");
    }

    #[test]
    fn test_empty_members() {
        test_remains_same::<Class, _>("class A { }", "class A { }");
    }

    #[test]
    fn test_method() {
        test_remains_same::<Class, _>("class A { a() {} }", "class A { a() {} }");
        test_remains_same::<Class, _>("class A { Number a() {} }", "class A { Number a() {} }");
        test_remains_same::<Class, _>(
            "class A { public Number a() {} }",
            "class A { public Number a() {} }",
        );
        test_remains_same::<Class, _>("class A { public a() {} }", "class A { public a() {} }");
        test_remains_same::<Class, _>(
            "class A { public a(Number a) {} }",
            "class A { public a(Number a) {} }",
        );
        test_remains_same::<Class, _>(
            "class A { public a(Number a, String b) {} }",
            "class A { public a(Number a, String b) {} }",
        );
        test_remains_same::<Class, _>(
            "class A { public a(Number a, String b) { return; } }",
            "class A { public a(Number a, String b) { return; } }",
        );
    }
}