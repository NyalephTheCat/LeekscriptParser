
use derive_more::Display;
use nom::{
    branch::alt, character::complete::{anychar, char, one_of}, combinator::{map, not}, multi::{many0}, sequence::{pair, preceded}, IResult
};

use crate::ast::*;

#[derive(Debug, Clone, Display, PartialEq)]
#[display(fmt = "{}{}{}", quote_type, value, quote_type)]
pub struct StringLiteral {
    pub value: String,
    pub quote_type: QuoteType,
}

#[derive(Debug, Clone, Copy, Display, PartialEq)]
pub enum QuoteType {
    #[display(fmt = "'")] Single,
    #[display(fmt = "\"")] Double,
}

impl ParseInto for StringLiteral {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        // Get the quote type
        let (input, quote_type) = alt((
            map(char('\''), |_| QuoteType::Single),
            map(char('"'), |_| QuoteType::Double),
        ))(input)?;

        // Get the string value
        let (input, value) = many0(
            eat_char(quote_type)
        )(input)?;

        // Eat the closing quote
        let (input, _) = char(match quote_type {
            QuoteType::Single => '\'',
            QuoteType::Double => '"',
        })(input)?;
        

        Ok((input, StringLiteral { value: value.join(""), quote_type }))
    }
}

// Escape sequences:
// - \b: backspace
// - \f: form feed
// - \n: line feed
// - \r: carriage return
// - \t: tab
// - \v: vertical tab
// - \\: backslash
// - \': single quote
// - \": double quote
// - \0: null character
fn eat_char<'a>(quote: QuoteType) -> impl Fn(Span<'a>) -> IResult<Span<'a>, String> {
    move |input: Span<'a>| {
        // Should eat the escape character and the character it escapes or the next character
        let (input, chars) = alt((
            map(
                pair(char('\\'), anychar), 
                |(_, c)| format!("\\{}", c)),
            map(preceded(not(match quote {
                QuoteType::Single => one_of("'\\"),
                QuoteType::Double => one_of("\"\\"),
            }), anychar), |c| c.to_string()),
        ))(input)?;

        Ok((input, chars))
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test_remains_same;

    use super::*;

    #[test]
    fn test_single_quote() {
        test_remains_same::<StringLiteral, _>("'hello'", "'hello'");
    }

    #[test]
    fn test_double_quote() {
        test_remains_same::<StringLiteral, _>("\"hello\"", "\"hello\"");
    }

    #[test]
    fn test_empty() {
        test_remains_same::<StringLiteral, _>("''", "''");
        test_remains_same::<StringLiteral, _>("\"\"", "\"\"");
    }

    #[test]
    fn test_escape() {
        test_remains_same::<StringLiteral, _>("'\\''", "'\\''");
        test_remains_same::<StringLiteral, _>("\"\\\"\"", "\"\\\"\"");
    }

    #[test]
    fn test_escape_escape() {
        test_remains_same::<StringLiteral, _>("'\\\\'", "'\\\\'");
        test_remains_same::<StringLiteral, _>("\"\\\\\"", "\"\\\\\"");
    }

    #[test]
    fn test_arbitrary_escape() {
        test_remains_same::<StringLiteral, _>("'\\a'", "'\\a'");
    }
}