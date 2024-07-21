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
    use crate::utils::test_remains_same;

    use super::*;

    #[test]
    fn test_simple_type() {
        test_remains_same::<Type, _>("String", "String");
    }

    #[test]
    fn test_generic_type() {
        test_remains_same::<Type, _>("Vec<String>", "Vec<String>");
    }

    #[test]
    fn test_multiple_generics() {
        test_remains_same::<Type, _>("HashMap<String, String>", "HashMap<String, String>");
    }

    #[test]
    fn test_nullable_type() {
        test_remains_same::<Type, _>("String?", "String?");
    }

    #[test]
    fn test_alternative_type() {
        test_remains_same::<Type, _>("String|Vec<String>", "String|Vec<String>");
    }

    #[test]
    fn test_complex_type() {
        test_remains_same::<Type, _>("HashMap<String, String>|Vec<String>|String?", "HashMap<String, String>|Vec<String>|String?");
    }

    #[test]
    fn test_complex_type_with_generics() {
        test_remains_same::<Type, _>("HashMap<String, String>|Vec<String>|String?|Vec<HashMap<String, String>>", "HashMap<String, String>|Vec<String>|String?|Vec<HashMap<String, String>>");
    }
}