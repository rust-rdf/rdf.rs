// This is free and unencumbered software released into the public domain.

extern crate alloc;

use crate::{Term, TermKind};
use alloc::{
    borrow::{Cow, ToOwned},
    string::String,
};

/// A clone-on-write term.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CowTerm<'a> {
    Iri(Cow<'a, str>),
    BNode(Cow<'a, str>),
    Literal(Cow<'a, str>),
    LiteralWithDatatype(Cow<'a, str>, Cow<'a, str>),
    LiteralWithLanguage(Cow<'a, str>, Cow<'a, str>),
}

impl<'a> CowTerm<'a> {
    pub const fn iri_static(value: &'static str) -> Self {
        Self::Iri(Cow::Borrowed(value))
    }

    pub const fn iri(value: Cow<'a, str>) -> Self {
        Self::Iri(value)
    }

    pub const fn bnode_static(id: &'static str) -> Self {
        Self::BNode(Cow::Borrowed(id))
    }

    pub const fn bnode(id: Cow<'a, str>) -> Self {
        Self::BNode(id)
    }

    pub const fn literal_static(value: &'static str) -> Self {
        Self::Literal(Cow::Borrowed(value))
    }

    pub const fn literal(value: Cow<'a, str>) -> Self {
        Self::Literal(value)
    }

    pub const fn literal_with_language(value: Cow<'a, str>, language: Cow<'a, str>) -> Self {
        Self::LiteralWithLanguage(value, language)
    }

    pub const fn literal_with_datatype(value: Cow<'a, str>, datatype: Cow<'a, str>) -> Self {
        Self::LiteralWithDatatype(value, datatype)
    }
}

impl<'a> Term for CowTerm<'a> {
    fn kind(&self) -> TermKind {
        match self {
            Self::Iri(_) => TermKind::Iri,
            Self::BNode(_) => TermKind::BNode,
            Self::Literal(_)
            | Self::LiteralWithLanguage(_, _)
            | Self::LiteralWithDatatype(_, _) => TermKind::Literal,
        }
    }

    fn as_str(&self) -> &str {
        match self {
            Self::Iri(s) => s,
            Self::BNode(s) => s,
            Self::Literal(s)
            | Self::LiteralWithLanguage(s, _)
            | Self::LiteralWithDatatype(s, _) => s,
        }
    }
}

impl<'a> From<&'a dyn Term> for CowTerm<'a> {
    fn from(term: &'a dyn Term) -> Self {
        match term.kind() {
            TermKind::Iri => Self::iri(Cow::from(term.as_str())),
            TermKind::BNode => Self::bnode(Cow::from(term.as_str())),
            TermKind::Literal => Self::literal(Cow::from(term.as_str())), // TODO
        }
    }
}

impl<'a> From<&'a str> for CowTerm<'a> {
    fn from(value: &'a str) -> Self {
        Self::literal(Cow::from(value))
    }
}

impl<'a> From<String> for CowTerm<'a> {
    fn from(value: String) -> Self {
        Self::Literal(Cow::from(value))
    }
}

impl<'a> From<&'a String> for CowTerm<'a> {
    fn from(value: &'a String) -> Self {
        Self::Literal(Cow::from(value))
    }
}
