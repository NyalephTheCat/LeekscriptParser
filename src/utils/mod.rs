use std::fmt::{Debug, Display};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::alphanumeric1,
    combinator::{not, recognize},
    sequence::pair,
    IResult,
};

use crate::ast::{ParseInto, Span};

pub fn kw<'a>(keyword: &'a str) -> impl FnMut(Span<'a>) -> IResult<Span<'a>, Span<'a>> {
    recognize(pair(tag(keyword), not(alt((alphanumeric1, tag("_"))))))
}

pub fn test_remains_same<T: ParseInto<Output = U>, U: Display + Debug>(input: &str, expected: &str) {
    let input = Span::new_extra(input, "test_input");
    let parse_result = T::parse_inner(input);

    assert!(parse_result.is_ok(), "Expected successful parse for {}, got {:?}", input, parse_result);
    let (rem, result) = parse_result.unwrap();

    assert!(rem.is_empty(), "Expected no remaining input for {}, got {:?}", input, rem);
    assert_eq!(result.to_string(), expected, "Expected {}, got {}", expected, result);
}