use derive_more::Display;
use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::{digit1, hex_digit1, oct_digit1},
    combinator::{consumed, map, opt},
    multi::many0,
    sequence::{pair, tuple},
    IResult,
};

use crate::ast::*;

#[derive(Debug, Clone, Display, PartialEq)]
#[display(fmt = "{}", raw)]
pub struct NumberLiteral {
    pub value: NumberValue,
    pub format: NumberFormat,
    pub raw: String, // This is for the display implementation
}

impl ParseInto for NumberLiteral {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        parse_number(input)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum NumberValue {
    Integer(i64),
    Float(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum NumberFormat {
    Decimal,
    Hexadecimal,
    Octal,
    Binary,
}

fn parse_digits<'a>(input: Span<'a>) -> IResult<Span<'a>, String> {
    map(
        pair(
            digit1,
            map(many0(pair(tag("_"), |s: Span<'a>| digit1(s))), |pairs| {
                pairs
                    .iter()
                    .map(|(_, s)| s.fragment().to_string())
                    .collect::<String>()
            }),
        ),
        |(first, rest)| format!("{}{}", first.fragment(), rest),
    )(input)
}

fn parse_exponent<'a>(input: Span<'a>) -> IResult<Span<'a>, String> {
    map(
        pair(|s: Span<'a>| is_a("eE")(s), pair(opt(is_a("+-")), digit1)),
        |(e, (sign, digits))| {
            format!(
                "{}{}{}",
                e.fragment(),
                sign.map_or("", |x| x.fragment()),
                digits.fragment()
            )
        },
    )(input)
}

fn parse_integer(input: Span) -> IResult<Span, NumberLiteral> {
    map(consumed(parse_digits), |(raw, digits)| {
        let parsed = digits.parse();

        NumberLiteral {
            value: NumberValue::Integer(parsed.unwrap()),
            format: NumberFormat::Decimal,
            raw: raw.fragment().to_string(),
        }
    })(input)
}

fn parse_float(input: Span) -> IResult<Span, NumberLiteral> {
    alt((
        map(
            consumed(tuple((
                parse_digits,
                tag("."),
                opt(parse_digits),
                opt(parse_exponent),
            ))),
            |(raw, (digits, _, fraction, exponent))| {
                let parsed = format!(
                    "{}.{}{}",
                    digits,
                    fraction.map_or("".to_string(), |s| s),
                    exponent.map_or("".to_string(), |s| s)
                )
                .parse();

                NumberLiteral {
                    value: NumberValue::Float(parsed.unwrap()),
                    format: NumberFormat::Decimal,
                    raw: raw.fragment().to_string(),
                }
            },
        ),
        map(
            consumed(tuple((tag("."), parse_digits, opt(parse_exponent)))),
            |(raw, (_, digits, exponent))| {
                let parsed =
                    format!("0.{}{}", digits, exponent.map_or("".to_string(), |s| s)).parse();

                NumberLiteral {
                    value: NumberValue::Float(parsed.unwrap()),
                    format: NumberFormat::Decimal,
                    raw: raw.fragment().to_string(),
                }
            },
        ),
        map(
            consumed(pair(parse_digits, parse_exponent)),
            |(raw, (digits, exponent))| {
                let parsed = format!("{}{}", digits, exponent).parse();

                NumberLiteral {
                    value: NumberValue::Float(parsed.unwrap()),
                    format: NumberFormat::Decimal,
                    raw: raw.fragment().to_string(),
                }
            },
        ),
    ))(input)
}

fn parse_digits_hex<'a>(input: Span<'a>) -> IResult<Span<'a>, String> {
    map(
        pair(
            hex_digit1,
            map(many0(pair(tag("_"), |s: Span<'a>| hex_digit1(s))), |pairs| {
                pairs
                    .iter()
                    .map(|(_, s)| s.fragment().to_string())
                    .collect::<String>()
            }),
        ),
        |(first, rest)| format!("{}{}", first.fragment(), rest),
    )(input)
}

fn parse_hex(input: Span) -> IResult<Span, NumberLiteral> {
    map(
        consumed(map(
            pair(alt((tag("0x"), tag("0X"))), parse_digits_hex),
            |(_, digits)| digits,
        )),
        |(raw, digits)| {
            let parsed = i64::from_str_radix(&digits, 16);

            NumberLiteral {
                value: NumberValue::Integer(parsed.unwrap()),
                format: NumberFormat::Hexadecimal,
                raw: raw.fragment().to_string(),
            }
        },
    )(input)
}

fn parse_digits_octal<'a>(input: Span<'a>) -> IResult<Span<'a>, String> {
    map(
        pair(
            oct_digit1,
            map(many0(pair(tag("_"), |s: Span<'a>| oct_digit1(s))), |pairs| {
                pairs
                    .iter()
                    .map(|(_, s)| s.fragment().to_string())
                    .collect::<String>()
            }),
        ),
        |(first, rest)| format!("{}{}", first.fragment(), rest),
    )(input)
}

fn parse_octal(input: Span) -> IResult<Span, NumberLiteral> {
    map(
        consumed(map(
            pair(alt((tag("0o"), tag("0O"))), parse_digits_octal),
            |(_, digits)| digits,
        )),
        |(raw, digits)| {
            let parsed = i64::from_str_radix(&digits, 8);

            NumberLiteral {
                value: NumberValue::Integer(parsed.unwrap()),
                format: NumberFormat::Octal,
                raw: raw.fragment().to_string(),
            }
        },
    )(input)
}

fn parse_digits_binary<'a>(input: Span<'a>) -> IResult<Span<'a>, String> {
    map(
        pair(
            is_a("01"),
            map(many0(pair(tag("_"), |s: Span<'a>| is_a("01")(s))), |pairs| {
                pairs
                    .iter()
                    .map(|(_, s)| s.fragment().to_string())
                    .collect::<String>()
            }),
        ),
        |(first, rest)| format!("{}{}", first.fragment(), rest),
    )(input)
}

fn parse_binary(input: Span) -> IResult<Span, NumberLiteral> {
    map(
        consumed(map(
            pair(alt((tag("0b"), tag("0B"))), parse_digits_binary),
            |(_, digits)| digits,
        )),
        |(raw, digits)| {
            let parsed = i64::from_str_radix(&digits, 2);

            NumberLiteral {
                value: NumberValue::Integer(parsed.unwrap()),
                format: NumberFormat::Binary,
                raw: raw.fragment().to_string(),
            }
        },
    )(input)
}

fn parse_number(input: Span) -> IResult<Span, NumberLiteral> {
    alt((
        parse_hex,
        parse_octal,
        parse_binary,
        parse_float,
        parse_integer,
    ))(input)
}

#[cfg(test)]
mod tests {
    use crate::utils::test_remains_same;

    use super::*;

    #[test]
    fn test_integer() {
        test_remains_same::<NumberLiteral, _>("0", "0");
        test_remains_same::<NumberLiteral, _>("1", "1");
        test_remains_same::<NumberLiteral, _>("123", "123");
        test_remains_same::<NumberLiteral, _>("1_000", "1_000");
        test_remains_same::<NumberLiteral, _>("1_000_000", "1_000_000");
    }

    #[test]
    fn test_float() {
        test_remains_same::<NumberLiteral, _>("0.0", "0.0");
        test_remains_same::<NumberLiteral, _>("1.0", "1.0");
        test_remains_same::<NumberLiteral, _>("123.0", "123.0");
        test_remains_same::<NumberLiteral, _>("1_000.0", "1_000.0");
        test_remains_same::<NumberLiteral, _>("1_000_000.0", "1_000_000.0");

        test_remains_same::<NumberLiteral, _>("0.0e0", "0.0e0");
        test_remains_same::<NumberLiteral, _>("1.0e0", "1.0e0");
        test_remains_same::<NumberLiteral, _>("123.0e0", "123.0e0");
        test_remains_same::<NumberLiteral, _>("1_000.0e0", "1_000.0e0");
        test_remains_same::<NumberLiteral, _>("1_000_000.0e0", "1_000_000.0e0");

        test_remains_same::<NumberLiteral, _>("0.0e+0", "0.0e+0");
        test_remains_same::<NumberLiteral, _>("1.0e+0", "1.0e+0");
        test_remains_same::<NumberLiteral, _>("123.0e+0", "123.0e+0");
        test_remains_same::<NumberLiteral, _>("1_000.0e+0", "1_000.0e+0");

        test_remains_same::<NumberLiteral, _>("0.0e-0", "0.0e-0");
        test_remains_same::<NumberLiteral, _>("1.0e-0", "1.0e-0");
        test_remains_same::<NumberLiteral, _>("123.0e-0", "123.0e-0");
        test_remains_same::<NumberLiteral, _>("1_000.0e-0", "1_000.0e-0");
    }

    #[test]
    fn test_hex() {
        test_remains_same::<NumberLiteral, _>("0x0", "0x0");
        test_remains_same::<NumberLiteral, _>("0x1", "0x1");
        test_remains_same::<NumberLiteral, _>("0x123", "0x123");
        test_remains_same::<NumberLiteral, _>("0x1_000", "0x1_000");
        test_remains_same::<NumberLiteral, _>("0x1_000_000", "0x1_000_000");

        test_remains_same::<NumberLiteral, _>("0xfff", "0xfff");
        test_remains_same::<NumberLiteral, _>("0X1_fff", "0X1_fff");
    }

    #[test]
    fn test_octal() {
        test_remains_same::<NumberLiteral, _>("0o0", "0o0");
        test_remains_same::<NumberLiteral, _>("0o1", "0o1");
        test_remains_same::<NumberLiteral, _>("0o123", "0o123");
        test_remains_same::<NumberLiteral, _>("0o1_000", "0o1_000");
        test_remains_same::<NumberLiteral, _>("0o1_000_000", "0o1_000_000");

        test_remains_same::<NumberLiteral, _>("0o7", "0o7");
        test_remains_same::<NumberLiteral, _>("0O7", "0O7");
    }

    #[test]
    fn test_binary() {
        test_remains_same::<NumberLiteral, _>("0b0", "0b0");
        test_remains_same::<NumberLiteral, _>("0b1", "0b1");
        test_remains_same::<NumberLiteral, _>("0b101", "0b101");
        test_remains_same::<NumberLiteral, _>("0b1_000", "0b1_000");
        test_remains_same::<NumberLiteral, _>("0b1_000_000", "0b1_000_000");
    }
}