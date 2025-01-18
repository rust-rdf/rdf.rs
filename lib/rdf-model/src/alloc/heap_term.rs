// This is free and unencumbered software released into the public domain.

extern crate alloc;

use crate::{Term, TermKind};
use alloc::{
    borrow::Cow,
    string::{String, ToString},
    vec::Vec,
};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HeapTerm {
    Iri(String),
    BNode(String),
    Literal(String),
    LiteralWithDatatype(String, String),
    LiteralWithLanguage(String, String),
}

impl HeapTerm {
    pub fn iri(value: impl AsRef<str>) -> Self {
        Self::Iri(String::from(value.as_ref()))
    }

    pub fn bnode(id: impl AsRef<str>) -> Self {
        Self::BNode(String::from(id.as_ref()))
    }

    pub fn literal(value: impl AsRef<str>) -> Self {
        Self::Literal(String::from(value.as_ref()))
    }

    pub fn literal_with_language(value: impl AsRef<str>, language: impl AsRef<str>) -> Self {
        Self::LiteralWithLanguage(
            String::from(value.as_ref()),
            String::from(language.as_ref()),
        )
    }

    pub fn literal_with_datatype(value: impl AsRef<str>, datatype: impl AsRef<str>) -> Self {
        Self::LiteralWithDatatype(
            String::from(value.as_ref()),
            String::from(datatype.as_ref()),
        )
    }
}

impl Term for HeapTerm {
    fn kind(&self) -> TermKind {
        match self {
            Self::Iri(_) => TermKind::Iri,
            Self::BNode(_) => TermKind::BNode,
            Self::Literal(_)
            | Self::LiteralWithLanguage(_, _)
            | Self::LiteralWithDatatype(_, _) => TermKind::Literal,
        }
    }

    fn as_str(&self) -> Cow<str> {
        match self {
            Self::Iri(s) => Cow::Borrowed(s),
            Self::BNode(s) => Cow::Borrowed(s),
            Self::Literal(s)
            | Self::LiteralWithLanguage(s, _)
            | Self::LiteralWithDatatype(s, _) => Cow::Borrowed(s),
        }
    }
}

impl From<&dyn Term> for HeapTerm {
    fn from(term: &dyn Term) -> Self {
        match term.kind() {
            TermKind::Iri => Self::iri(term.as_str()),
            TermKind::BNode => Self::bnode(term.as_str()),
            TermKind::Literal => Self::literal(term.as_str()), // TODO
        }
    }
}

impl From<&str> for HeapTerm {
    fn from(value: &str) -> Self {
        Self::literal(value)
    }
}

impl From<String> for HeapTerm {
    fn from(value: String) -> Self {
        Self::literal(&value)
    }
}
