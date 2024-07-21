
use nom::{branch::alt, bytes::complete::tag, character::complete::char, combinator::{map, value}, IResult};
use derive_more::{Display, From};
use crate::{ast::*, utils::kw};

/* Types for binary and unary operations in order of priority */
pub type LogicalOr = BinExpr<LogicalOrOp, LogicalXorExpr>;
pub type LogicalXorExpr = BinExpr<LogicalXorOp, LogicalAndExpr>;
pub type LogicalAndExpr = BinExpr<LogicalAndOp, RelationExpr>;
pub type RelationExpr = BinExpr<RelationOp, InstanceOfExpr>;
pub type InstanceOfExpr = BinExpr<InstanceOfOp, ShiftExpr>;
pub type ShiftExpr = BinExpr<ShiftOp, BinOrExpr>;
pub type BinOrExpr = BinExpr<BinOrOp, BinXorExpr>;
pub type BinXorExpr = BinExpr<BinXorOp, BinAndExpr>;
pub type BinAndExpr = BinExpr<BinAndOp, AddExpr>;
pub type AddExpr = BinExpr<AddOp, MultExpr>;
pub type MultExpr = BinExpr<MultOp, TypeConversionExpr>;
pub type PreUpdateExpr = UnaryLeft<UpdateOp, PostUpdateExpr>;
pub type PostUpdateExpr = UnaryRight<UpdateOp, UnaryExpr>;
pub type UnaryExpr = UnaryLeft<UnaryOp, Member>;

#[derive(Debug, Clone, Display, From)]
pub enum Expression {
    Assign(AssignExpr),
    AnonyFunc(AnonymousFunction),
    TernaryExpression(TernaryExpression),
    LogicalOr(LogicalOr),
    LogicalXor(LogicalXorExpr),
    LogicalAnd(LogicalAndExpr),
    Relation(RelationExpr),
    InstanceOf(InstanceOfExpr),
    Shift(ShiftExpr),
    BinOr(BinOrExpr),
    BinXor(BinXorExpr),
    BinAnd(BinAndExpr),
    Add(AddExpr),
    Mult(MultExpr),
    TypeConversion(TypeConversionExpr),
    PreUpdate(PreUpdateExpr),
    PostUpdate(PostUpdateExpr),
    Unary(UnaryExpr),
    Member(Member),
    Primary(PrimaryExpr),
}

impl ParseInto for Expression {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        alt((
            AnonymousFunction::parse_inner,
            AssignExpr::parse_inner,
        ))(input)
    }
}

#[derive(Debug, Clone, Display)]
pub enum UnaryOp {
    Not(String),
    #[display(fmt = "+")] Plus,
    #[display(fmt = "-")] Neg,
    #[display(fmt = "~")] BitNot,
    #[display(fmt = "typeof")] Typeof,
    #[display(fmt = "new")] New,
}

impl ParseInto for UnaryOp {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        alt((
            map(alt((tag("!"), kw("not"))), |not| UnaryOp::Not(not.to_string())),
            value(UnaryOp::Plus, char('+')),
            value(UnaryOp::Neg, char('-')),
            value(UnaryOp::BitNot, char('~')),
            value(UnaryOp::Typeof, kw("typeof")),
            value(UnaryOp::New, kw("new")),
        ))(input)
    }
}

#[derive(Debug, Clone, Display)]
pub enum UpdateOp {
    #[display(fmt = "++")] Inc,
    #[display(fmt = "--")] Dec,
}

impl ParseInto for UpdateOp {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        alt((
            value(UpdateOp::Inc, tag("++")),
            value(UpdateOp::Dec, tag("--")),
        ))(input)
    }
}

#[derive(Debug, Clone, Display)]
pub enum MultOp {
    #[display(fmt = "*")] Mul,
    #[display(fmt = "/")] Div,
    #[display(fmt = "%")] Mod,
}

impl ParseInto for MultOp {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        alt((
            value(MultOp::Mul, char('*')),
            value(MultOp::Div, char('/')),
            value(MultOp::Mod, char('%')),
        ))(input)
    }
}

#[derive(Debug, Clone, Display)]
pub enum AddOp {
    #[display(fmt = "+")] Add,
    #[display(fmt = "-")] Sub,
}

impl ParseInto for AddOp {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        alt((
            value(AddOp::Add, char('+')),
            value(AddOp::Sub, char('-')),
        ))(input)
    }
}

#[derive(Debug, Clone, Display)]
#[display(fmt = "&")]
pub struct BinAndOp;

impl ParseInto for BinAndOp {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        value(BinAndOp, char('&'))(input)
    }
}

#[derive(Debug, Clone, Display)]
#[display(fmt = "^")]
pub struct BinXorOp;

impl ParseInto for BinXorOp {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        value(BinXorOp, char('^'))(input)
    }
}

#[derive(Debug, Clone, Display)]
#[display(fmt = "|")]
pub struct BinOrOp;

impl ParseInto for BinOrOp {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        value(BinOrOp, char('|'))(input)
    }
}

#[derive(Debug, Clone, Display)]
pub enum ShiftOp {
    #[display(fmt = "<<")] Left,
    #[display(fmt = ">>")] Right,
    #[display(fmt = ">>>")] UnsignedRight,
}

impl ParseInto for ShiftOp {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        alt((
            value(ShiftOp::Left, tag("<<")),
            value(ShiftOp::UnsignedRight, tag(">>>")),
            value(ShiftOp::Right, tag(">>")),
        ))(input)
    }
}

#[derive(Debug, Clone, Display)]
#[display(fmt = "instanceof")]
pub struct InstanceOfOp;

impl ParseInto for InstanceOfOp {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        value(InstanceOfOp, kw("instanceof"))(input)
    }
}

#[derive(Debug, Clone, Display)]
pub enum RelationOp {
    #[display(fmt = "<")] Less,
    #[display(fmt = "<=")] LessEq,
    #[display(fmt = ">")] Greater,
    #[display(fmt = ">=")] GreaterEq,
    #[display(fmt = "==")] Eq,
    #[display(fmt = "!=")] NotEq,
}

impl ParseInto for RelationOp {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        alt((
            value(RelationOp::LessEq, tag("<=")),
            value(RelationOp::GreaterEq, tag(">=")),
            value(RelationOp::Less, char('<')),
            value(RelationOp::Greater, char('>')),
            value(RelationOp::Eq, tag("==")),
            value(RelationOp::NotEq, tag("!=")),
        ))(input)
    }
}

#[derive(Debug, Clone, Display)]
pub enum LogicalAndOp {
    And(String),
}

impl ParseInto for LogicalAndOp {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        map(alt((kw("and"), tag("&&"))), |and| LogicalAndOp::And(and.to_string()))(input)
    }
}

#[derive(Debug, Clone, Display)]
pub enum LogicalXorOp {
    Xor(String),
}

impl ParseInto for LogicalXorOp {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        map(kw("xor"), |xor| LogicalXorOp::Xor(xor.to_string()))(input)
    }
}

#[derive(Debug, Clone, Display)]
pub enum LogicalOrOp {
    Or(String),
}

impl ParseInto for LogicalOrOp {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        map(alt((kw("or"), tag("||"))), |or| LogicalOrOp::Or(or.to_string()))(input)
    }
}

#[derive(Debug, Clone, Display)]
pub enum AssignOp {
    #[display(fmt = "=")] Assign,
    #[display(fmt = "+=")] Add,
    #[display(fmt = "-=")] Sub,
    #[display(fmt = "*=")] Mul,
    #[display(fmt = "/=")] Div,
    #[display(fmt = "%=")] Mod,
    #[display(fmt = "<<=")] Left,
    #[display(fmt = ">>=")] Right,
    #[display(fmt = ">>>=")] UnsignedRight,
    #[display(fmt = "&=")] And,
    #[display(fmt = "^=")] Xor,
    #[display(fmt = "|=")] Or,
}

impl ParseInto for AssignOp {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        alt((
            value(AssignOp::Assign, char('=')),
            value(AssignOp::Add, tag("+=")),
            value(AssignOp::Sub, tag("-=")),
            value(AssignOp::Mul, tag("*=")),
            value(AssignOp::Div, tag("/=")),
            value(AssignOp::Mod, tag("%=")),
            value(AssignOp::Left, tag("<<=")),
            value(AssignOp::Right, tag(">>=")),
            value(AssignOp::UnsignedRight, tag(">>>=")),
            value(AssignOp::And, tag("&=")),
            value(AssignOp::Xor, tag("^=")),
            value(AssignOp::Or, tag("|=")),
        ))(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test_remains_same;

    use super::*;

    #[test]
    fn test_unary() {
        test_remains_same::<Expression, _>("-1", "-1");
        test_remains_same::<Expression, _>("+1", "+1");
        test_remains_same::<Expression, _>("~1", "~1");
        test_remains_same::<Expression, _>("typeof 1", "typeof 1");
        test_remains_same::<Expression, _>("new 1", "new 1");
        test_remains_same::<Expression, _>("!1", "!1");
        test_remains_same::<Expression, _>("not 1", "not 1");
    }

    #[test]
    fn test_update() {
        test_remains_same::<Expression, _>("1++", "1++");
        test_remains_same::<Expression, _>("1--", "1--");
        test_remains_same::<Expression, _>("++1", "++1");
        test_remains_same::<Expression, _>("--1", "--1");
    }

    #[test]
    fn test_mult() {
        test_remains_same::<Expression, _>("1 * 2", "1 * 2");
        test_remains_same::<Expression, _>("1 / 2", "1 / 2");
        test_remains_same::<Expression, _>("1 % 2", "1 % 2");
    }

    #[test]
    fn test_add() {
        test_remains_same::<Expression, _>("1 + 2", "1 + 2");
        test_remains_same::<Expression, _>("1 - 2", "1 - 2");
    }

    #[test]
    fn test_shift() {
        test_remains_same::<Expression, _>("1 << 2", "1 << 2");
        test_remains_same::<Expression, _>("1 >> 2", "1 >> 2");
        test_remains_same::<Expression, _>("1 >>> 2", "1 >>> 2");
    }

    #[test]
    fn test_relation() {
        test_remains_same::<Expression, _>("1 < 2", "1 < 2");
        test_remains_same::<Expression, _>("1 <= 2", "1 <= 2");
        test_remains_same::<Expression, _>("1 > 2", "1 > 2");
        test_remains_same::<Expression, _>("1 >= 2", "1 >= 2");
        test_remains_same::<Expression, _>("1 == 2", "1 == 2");
        test_remains_same::<Expression, _>("1 != 2", "1 != 2");
    }

    #[test]
    fn test_logical_and() {
        test_remains_same::<Expression, _>("1 and 2", "1 and 2");
        test_remains_same::<Expression, _>("1 && 2", "1 && 2");
    }

    #[test]
    fn test_logical_xor() {
        test_remains_same::<Expression, _>("1 xor 2", "1 xor 2");
    }

    #[test]
    fn test_logical_or() {
        test_remains_same::<Expression, _>("1 or 2", "1 or 2");
        test_remains_same::<Expression, _>("1 || 2", "1 || 2");
    }

    #[test]
    fn test_assign() {
        test_remains_same::<Expression, _>("1 = 2", "1 = 2");
        test_remains_same::<Expression, _>("1 += 2", "1 += 2");
        test_remains_same::<Expression, _>("1 -= 2", "1 -= 2");
        test_remains_same::<Expression, _>("1 *= 2", "1 *= 2");
        test_remains_same::<Expression, _>("1 /= 2", "1 /= 2");
        test_remains_same::<Expression, _>("1 %= 2", "1 %= 2");
        test_remains_same::<Expression, _>("1 <<= 2", "1 <<= 2");
        test_remains_same::<Expression, _>("1 >>= 2", "1 >>= 2");
        test_remains_same::<Expression, _>("1 >>>= 2", "1 >>>= 2");
        test_remains_same::<Expression, _>("1 &= 2", "1 &= 2");
        test_remains_same::<Expression, _>("1 ^= 2", "1 ^= 2");
        test_remains_same::<Expression, _>("1 |= 2", "1 |= 2");
    }

    #[test]
    fn test_expression() {
        let complex_input = r#"1 + 2 * 3 / 4 % 5 and a[b] + [1, 2, 3] or c.d(1, 2, 3) << 4 >> 5 >>> 6"#;
        
        test_remains_same::<Expression, _>(complex_input, complex_input);
    }
}