use std::fmt::{Debug, Display};
use derive_builder::Builder;
use nom::combinator::map;

pub type Span<'a> = nom_locate::LocatedSpan<&'a str>;

pub trait ParseInto: Sized where <Self as ParseInto>::Output: From<Self> {
    type Output;

    fn parse<'a>(input: Span<'a>) -> IResult<Span<'a>, MetaNode<Self::Output>> {
        let (input, pre_comments) = comment::parse_comment_or_whitespace(input)?;
        let (input, node) = Self::parse_inner(input)?;
        let (input, post_comments) = comment::parse_comment_or_whitespace(input)?;
        Ok((
            input,
            MetaNode {
                node: Box::new(node),
                pre_comments,
                post_comments,
            },
        ))
    }

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output>;
}

impl<T, U, V> ParseInto for (T, U) where 
T: ParseInto<Output = V>,
U: ParseInto<Output = V>,
V: From<(T, U)>,
{
    type Output = T::Output;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        alt((
            T::parse_inner,
            U::parse_inner,
        ))(input)
    }
}

#[derive(Clone, Builder)]
#[builder(setter(into))]
pub struct MetaNode<Node>
{
    pub node: Box<Node>,
    #[builder(default)] pub pre_comments: Vec<CommentOrWhitespace>,
    #[builder(default)] pub post_comments: Vec<CommentOrWhitespace>,
}

impl<Node> PartialEq for MetaNode<Node> where Node: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
    }
}

impl<'a, Node: ParseInto> Debug for MetaNode<Node> where Node: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{:#?}", self.node)
        } else {
            write!(f, "{:?}", self.node)
        }
    }
}

impl<'a, Node: ParseInto> Display for MetaNode<Node> where Node: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.pre_comments
                .iter()
                .map(|c| format!("{}", c))
                .collect::<Vec<_>>()
                .join(""),
            self.node,
            self.post_comments
                .iter()
                .map(|c| format!("{}", c))
                .collect::<Vec<_>>()
                .join("")
        )
    }
}

impl<Node: PartialEq> PartialEq<Node> for MetaNode<Node> {
    fn eq(&self, other: &Node) -> bool {
        self.node.as_ref() == other
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CommentOrWhitespace {
    SingleLineComment(String),
    MultiLineComment(String),
    Whitespace(String),
}

impl Display for CommentOrWhitespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommentOrWhitespace::SingleLineComment(s) => write!(f, "{}", s),
            CommentOrWhitespace::MultiLineComment(s) => write!(f, "{}", s),
            CommentOrWhitespace::Whitespace(s) => write!(f, "{}", s),
        }
    }
}

mod comment;

pub mod literals;
pub mod expressions;
pub mod statements;
pub mod file;
pub mod types;
pub mod class;

pub use literals::*;
pub use expressions::*;
use nom::{branch::alt, IResult};
pub use statements::*;
pub use file::*;
pub use types::*;
pub use class::*;