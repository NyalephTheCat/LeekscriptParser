use nom::{bytes::complete::tag, combinator::{map, opt, peek}, multi::separated_list1, sequence::{pair, preceded, terminated, tuple}, IResult};

use crate::{ast::*, utils::kw};

use super::MetaNode;

#[derive(Debug, Clone)]
pub struct GlobalDefinition {
    pub type_: Option<MetaNode<Type>>,
    pub values: Vec<(MetaNode<Identifier>, Option<MetaNode<Expression>>)>,
    pub semi: MetaNode<Semi>,
}

impl Display for GlobalDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "global")?;
        if let Some(type_) = &self.type_ {
            write!(f, "{}", type_)?;
        }

        for (i, (name, value)) in self.values.iter().enumerate() {
            write!(f, "{}", name)?;

            if let Some(value) = value {
                write!(f, "={}", value)?;
            }

            if i < self.values.len() - 1 {
                write!(f, ",")?;
            }
        }

        write!(f, "{}", self.semi)
    }
}

impl ParseInto for GlobalDefinition {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        map(
            tuple((
                kw("global"),
                opt(terminated(Type::parse, peek(Identifier::parse))), // Lookahead to see if we have a type
                separated_list1(tag(","), pair(Identifier::parse,
                    opt(preceded(tag("="), Expression::parse)))),
                Semi::parse,
            )),
            |(_, type_, values, semi)| GlobalDefinition {
                type_,
                values,
                semi,
            },
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test_remains_same;

    use super::*;

    #[test]
    fn test_global_definition() {
        test_remains_same::<GlobalDefinition, _>("global a;", "global a;");
        test_remains_same::<GlobalDefinition, _>("global Number a;", "global Number a;");
        test_remains_same::<GlobalDefinition, _>("global a=1;", "global a=1;");
        test_remains_same::<GlobalDefinition, _>("global Number a=1;", "global Number a=1;");
    }

    #[test]
    fn test_global_definition_with_type() {
        test_remains_same::<GlobalDefinition, _>("global Number a;", "global Number a;");
        test_remains_same::<GlobalDefinition, _>("global Number a=1;", "global Number a=1;");
    }

    #[test]
    fn test_multiple_var_declaration() {
        test_remains_same::<GlobalDefinition, _>("global a,b;", "global a,b;");
        test_remains_same::<GlobalDefinition, _>("global a,b=1;", "global a,b=1;");
        test_remains_same::<GlobalDefinition, _>("global Number a,b;", "global Number a,b;");
        test_remains_same::<GlobalDefinition, _>("global Number a,b=1;", "global Number a,b=1;");
    }
}