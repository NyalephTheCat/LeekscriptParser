use derive_builder::Builder;
use nom::{bytes::complete::tag, combinator::opt, multi::separated_list1, sequence::{delimited, preceded, tuple}};

use crate::ast::*;

#[derive(Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
pub struct Type {
    pub type_: MetaNode<Identifier>,
    #[builder(default = "Vec::new()")] pub generics: Vec<MetaNode<Type>>,
    #[builder(default)] pub alternative: Option<MetaNode<Type>>,
    #[builder(default = "false")] pub nullable: bool,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.type_)?;
    
        if !self.generics.is_empty() {
            write!(f, "<")?;
            for (i, generic) in self.generics.iter().enumerate() {
                if i > 0 {
                    write!(f, ",")?;
                }
                write!(f, "{}", generic)?;
            }
            write!(f, ">")?;
        }
        
        if let Some(alternative) = &self.alternative {
            write!(f, "|{}", alternative)?;
        }
        
        if self.nullable {
            write!(f, "?")?;
        }
        Ok(())
    }
}

impl ParseInto for Type {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        map(
            tuple((
                Identifier::parse,
                opt(delimited(
                    tag("<"),
                    separated_list1(tag(","), Type::parse),
                    tag(">"),
                )),
                opt(preceded(tag("|"), Type::parse)),
                opt(tag("?")),
            )),
            |(type_, generics, alternative, nullable)| Type {
                type_: type_,
                generics: generics.unwrap_or_default(),
                alternative,
                nullable: nullable.is_some(),
            }
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parses_self(input: &str) {
        parses_to(input, input);
    }

    fn parses_to(input: &str, expected: &str) {
        let (remaining, result) = Type::parse(Span::new(input)).unwrap();
        let output = result.to_string();
        
        // Try and parse again to ensure the output is the same
        let (_, result) = Type::parse(Span::new(&output)).unwrap();
        let output2 = result.to_string();

        assert_eq!(output, expected);
        assert_eq!(output2, expected);

        assert!(remaining.is_empty());
    }

    #[test]
    fn test_simple() {
        parses_self("Foo");
    }

    #[test]
    fn test_generics() {
        parses_self("Foo<Bar>");
        parses_self("Foo<Bar, Baz>");
        parses_self("Foo<Bar, Baz, Qux>");
    }

    #[test]
    fn test_alternative() {
        parses_self("Foo|Bar");
        parses_self("Foo<Bar>|Baz");
        parses_self("Foo<Bar, Baz>|Qux");
    }

    #[test]
    fn test_nullable() {
        parses_self("Foo?");
        parses_self("Foo<Bar>?");
        parses_self("Foo<Bar, Baz>?");
        parses_self("Foo<Bar, Baz>|Qux?");
    }

    #[test]
    fn test_complex() {
        parses_self("Foo<Bar, Baz>|Qux<Bar, Baz>?");
    }
}