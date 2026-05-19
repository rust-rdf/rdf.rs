// This is free and unencumbered software released into the public domain.

extern crate alloc;

use crate::{Term, TermKind};
use alloc::string::{String, ToString};

/// A heap-allocated term.
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
    pub fn iri(value: impl ToString) -> Self {
        Self::Iri(value.to_string())
    }

    pub fn bnode(id: impl ToString) -> Self {
        Self::BNode(id.to_string())
    }

    pub fn literal(value: impl ToString) -> Self {
        Self::Literal(value.to_string())
    }

    pub fn literal_with_language(value: impl ToString, language: impl ToString) -> Self {
        Self::LiteralWithLanguage(value.to_string(), language.to_string())
    }

    pub fn literal_with_datatype(value: impl ToString, datatype: impl ToString) -> Self {
        Self::LiteralWithDatatype(value.to_string(), datatype.to_string())
    }

    pub fn kind(&self) -> TermKind {
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
            Self::Iri(s) => s.as_str(),
            Self::BNode(s) => s.as_str(),
            Self::Literal(s)
            | Self::LiteralWithLanguage(s, _)
            | Self::LiteralWithDatatype(s, _) => s.as_str(),
        }
    }

    // fn as_str(&self) -> Cow<'_, str> {
    //     match self {
    //         Self::Iri(s) => Cow::Borrowed(s),
    //         Self::BNode(s) => Cow::Borrowed(s),
    //         Self::Literal(s)
    //         | Self::LiteralWithLanguage(s, _)
    //         | Self::LiteralWithDatatype(s, _) => Cow::Borrowed(s),
    //     }
    // }
}

impl Term for HeapTerm {
    fn kind(&self) -> TermKind {
        self.kind()
    }

    fn as_str(&self) -> &str {
        self.as_str()
    }
}

impl Term for &HeapTerm {
    fn kind(&self) -> TermKind {
        (*self).kind()
    }

    fn as_str(&self) -> &str {
        (*self).as_str()
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
        Self::Literal(value)
    }
}

impl From<&String> for HeapTerm {
    fn from(value: &String) -> Self {
        Self::Literal(value.clone())
    }
}
