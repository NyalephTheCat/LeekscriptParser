use crate::ast::*;
use combinator::opt;
use multi::many0;
use combinator::map;
use nom::*;
use sequence::tuple;

#[derive(Clone)]
pub struct BinExpr<Op: ParseInto, Expr: ParseInto> {
    marker: std::marker::PhantomData::<Expr>,

    pub left: Box<Expression>,
    pub right: Vec<(MetaNode<Op>, MetaNode<Expression>)>,
}

impl<Op: Debug + ParseInto, Expr: Debug + ParseInto> Debug for BinExpr<Op, Expr> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Display the debug representation of the struct without the marker
        f.debug_struct("BinExpr")
            .field("left", &self.left)
            .field("right", &self.right)
            .finish()
    }
}

impl<Op: ParseInto + Display, Expr: ParseInto> Display for BinExpr<Op, Expr> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.left)?;
        for (op, expr) in &self.right {
            write!(f, "{}{}", op, expr)?;
        }
        Ok(())
    }
}

impl<Op, Expr> ParseInto for BinExpr<Op, Expr> where
Op: ParseInto<Output = Op>,
Expr: ParseInto<Output = Expression>,
Expression: From<BinExpr<Op, Expr>>,
{
    type Output = Expression;

    fn parse_inner(input: Span) -> IResult<Span, Self::Output> {
        map(
            tuple((
                Expr::parse_inner,
                many0(tuple((Op::parse, Expr::parse))),
            )),
            |(left, right)| match right.len() {
                0 => left,
                _ => {
                    Self::Output::from(BinExpr {
                        marker: std::marker::PhantomData,
                        left: Box::new(left),
                        right,
                    })
                }
            }
        )(input)
    }
}

#[derive(Clone)]
pub struct UnaryLeft<Op: ParseInto, Expr: ParseInto> {
    marker: std::marker::PhantomData::<Expr>,

    pub op: MetaNode<Op>,
    pub expr: Box<Expression>,
}

impl<Op: Debug + ParseInto, Expr: Debug + ParseInto> Debug for UnaryLeft<Op, Expr> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Display the debug representation of the struct without the marker
        f.debug_struct("UnaryLeft")
            .field("op", &self.op)
            .field("expr", &self.expr)
            .finish()
    }
}

impl<Op: ParseInto + Display, Expr: ParseInto> Display for UnaryLeft<Op, Expr> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.op, self.expr)
    }
}

impl<Op: ParseInto, Expr: ParseInto> ParseInto for UnaryLeft<Op, Expr> where
Op: ParseInto<Output = Op>,
Expr: ParseInto<Output = Expression>,
Expression: From<UnaryLeft<Op, Expr>>,
{
    type Output = Expression;

    fn parse_inner(input: Span) -> IResult<Span, Self::Output> {
        map(
            tuple((
                opt(Op::parse),
                Expr::parse_inner,
            )),
            |(op, expr)| match op {
                Some(op) => Self::Output::from(UnaryLeft {
                    marker: std::marker::PhantomData,
                    op: op,
                    expr: Box::new(expr),
                }),
                None => expr,
            }
        )(input)
    }
}

#[derive(Clone)]
pub struct UnaryRight<Op: ParseInto, Expr: ParseInto> {
    marker: std::marker::PhantomData::<Expr>,

    pub expr: Box<Expression>,
    pub op: MetaNode<Op>,
}

impl<Op: Debug + ParseInto, Expr: Debug + ParseInto> Debug for UnaryRight<Op, Expr> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Display the debug representation of the struct without the marker
        f.debug_struct("UnaryRight")
            .field("expr", &self.expr)
            .field("op", &self.op)
            .finish()
    }
}

impl<Op: ParseInto + Display, Expr: ParseInto> Display for UnaryRight<Op, Expr> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.expr, self.op)
    }
}

impl<Op: ParseInto, Expr: ParseInto> ParseInto for UnaryRight<Op, Expr> where
Op: ParseInto<Output = Op>,
Expr: ParseInto<Output = Expression>,
Expression: From<UnaryRight<Op, Expr>>,
{
    type Output = Expression;

    fn parse_inner(input: Span) -> IResult<Span, Self::Output> {
        map(
            tuple((
                Expr::parse_inner,
                opt(Op::parse),
            )),
            |(expr, op)| match op {
                Some(op) => Self::Output::from(UnaryRight {
                    marker: std::marker::PhantomData,
                    expr: Box::new(expr),
                    op: op,
                }),
                None => expr,
            }
        )(input)
    }
}