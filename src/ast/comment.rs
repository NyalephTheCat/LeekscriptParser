use super::{CommentOrWhitespace, Span};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_until},
    character::complete::multispace1,
    combinator::{map, recognize},
    multi::many0,
    sequence::{pair, tuple},
    IResult,
};

pub fn parse_whitespace(input: Span) -> IResult<Span, CommentOrWhitespace> {
    map(multispace1, |s: Span| {
        CommentOrWhitespace::Whitespace(s.fragment().to_string())
    })(input)
}

pub fn single_line_comment(i: Span) -> IResult<Span, CommentOrWhitespace> {
    map(recognize(pair(tag("//"), is_not("\n\r"))), |s: Span| {
        CommentOrWhitespace::SingleLineComment(s.fragment().to_string())
    })(i)
}

pub fn multi_line_comment(i: Span) -> IResult<Span, CommentOrWhitespace> {
    map(
        recognize(tuple((tag("/*"), take_until("*/"), tag("*/")))),
        |s: Span| CommentOrWhitespace::MultiLineComment(s.fragment().to_string()),
    )(i)
}

pub fn parse_comment_or_whitespace(i: Span) -> IResult<Span, Vec<CommentOrWhitespace>> {
    many0(alt((
        single_line_comment,
        multi_line_comment,
        parse_whitespace,
    )))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_comment_or_whitespace() {
        let input = "   // single line comment\n/* multi\nline\ncomment */   ";
        let result = parse_comment_or_whitespace(input.into());
        assert!(result.is_ok());
        let (remaining, parsed) = result.unwrap();
        assert!(remaining.is_empty());
        assert_eq!(
            parsed,
            vec![
                CommentOrWhitespace::Whitespace("   ".to_string()),
                CommentOrWhitespace::SingleLineComment("// single line comment".to_string()),
                CommentOrWhitespace::Whitespace("\n".to_string()),
                CommentOrWhitespace::MultiLineComment("/* multi\nline\ncomment */".to_string()),
                CommentOrWhitespace::Whitespace("   ".to_string()),
            ]
        );
    }
}
