use nom::{branch::alt, combinator::{map, opt, value}, sequence::tuple, IResult};
use derive_more::Display;
use crate::{ast::*, utils::kw};

#[derive(Debug, Clone, Display)]
pub enum Privacy {
    #[display(fmt = "public")] Public,
    #[display(fmt = "protected")] Protected,
    #[display(fmt = "private")] Private,
}

impl ParseInto for Privacy {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        alt((
            value(Privacy::Public, kw("public")),
            value(Privacy::Protected, kw("protected")),
            value(Privacy::Private, kw("private")),
        ))(input)
    }
}

#[derive(Debug, Clone, Display)]
#[display(fmt = "static")]
pub struct Static;

impl ParseInto for Static {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        value(Static, kw("static"))(input)
    }
}

#[derive(Debug, Clone)]
pub struct Visibility {
    pub privacy: Option<MetaNode<Privacy>>,
    pub static_: Option<MetaNode<Static>>,
}

impl std::fmt::Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(privacy) = &self.privacy {
            write!(f, "{}", privacy)?;
        }
        if let Some(static_) = &self.static_ {
            write!(f, "{}", static_)?;
        }
        Ok(())
    }
}

impl ParseInto for Visibility {
    type Output = Self;

    fn parse_inner<'a>(input: Span<'a>) -> IResult<Span<'a>, Self::Output> {
        map(
            tuple((
                opt(Privacy::parse),
                opt(Static::parse),
            )),
            |(privacy, static_)| Visibility { privacy, static_ },
        )(input)
    }
}