use crate::{ast::*, utils::kw};
use nom::{IResult, sequence::tuple, combinator::map};
use derive_more::Display;

#[derive(Debug, Clone, Display)]
#[display(fmt = "{}constructor{}{}", visibility, parameters, body)]
pub struct Constructor {
    pub visibility: MetaNode<Visibility>,
    pub parameters: MetaNode<Parameters>,
    pub body: MetaNode<BlockStatement>,
}

impl ParseInto for Constructor {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        map(
            tuple((
                Visibility::parse,
                kw("constructor"),
                Parameters::parse,
                BlockStatement::parse,
            )),
            |(visibility, _, parameters, body)| Constructor {
                visibility,
                parameters,
                body,
            },
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test_remains_same;

    use super::*;

    #[test]
    fn test_empty() {
        test_remains_same::<Constructor, _>("public constructor() {}", "public constructor() {}");
        test_remains_same::<Constructor, _>("constructor() {}", "constructor() {}");
        test_remains_same::<Constructor, _>("private constructor() {}", "private constructor() {}");
        test_remains_same::<Constructor, _>("protected constructor() {}", "protected constructor() {}");
    }

    #[test]
    fn test_params() {
        test_remains_same::<Constructor, _>("public constructor(String a, Number b) {}", "public constructor(String a, Number b) {}");
        test_remains_same::<Constructor, _>("constructor(String a) {}", "constructor(String a) {}");
    }

    #[test]
    fn test_body() {
        test_remains_same::<Constructor, _>("public constructor() { return; }", "public constructor() { return; }");
        test_remains_same::<Constructor, _>("constructor() { return; }", "constructor() { return; }");
    }
}