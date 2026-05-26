// This is free and unencumbered software released into the public domain.

extern crate alloc;

use crate::{BaseDirection, CowTerm, Datatype, Term, TermKind};
use alloc::{
    borrow::Cow,
    string::{String, ToString},
};
use xsd::{ParseError, PrimitiveValue, Type, Value, primitives::Boolean};

type Language = String; // TODO

/// A heap-allocated term.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
// #[cfg_attr(
//     feature = "borsh",
//     derive(borsh::BorshSerialize, borsh::BorshDeserialize)
// )]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HeapTerm {
    /// An IRI or IRI reference term.
    Iri(String),

    /// A blank node (aka bnode) term.
    BNode(String),

    /// A plain string term.
    /// The term's datatype is `xsd:string`.
    String(String),

    /// A language-tagged string term.
    /// The term's datatype is either `rdf:langString` or `rdf:dirLangString`.
    TaggedString(String, Language, Option<BaseDirection>),

    /// A typed value term.
    TypedValue(Value),

    /// A typed literal term that could not be instantiated as a typed value.
    /// The term is either an ill-typed literal and/or an unsupported datatype.
    TypedLiteral(String, Datatype),
}

impl HeapTerm {
    pub fn iri(value: impl Into<String>) -> Self {
        Self::Iri(value.into())
    }

    pub fn bnode(id: impl Into<String>) -> Self {
        Self::BNode(id.into())
    }

    pub fn string(value: impl Into<String>) -> Self {
        Self::String(value.into())
    }

    pub fn tagged_string(value: impl Into<String>, tag: impl Into<Language>) -> Self {
        Self::TaggedString(value.into(), tag.into(), None)
    }

    pub fn typed_value(value: impl Into<Value>) -> Self {
        Self::TypedValue(value.into())
    }

    pub fn typed_literal(literal: impl Into<String>, datatype: impl Into<Datatype>) -> Self {
        Self::TypedLiteral(literal.into(), datatype.into())
    }

    pub fn kind(&self) -> TermKind {
        match self {
            Self::Iri(_) => TermKind::Iri,
            Self::BNode(_) => TermKind::BNode,
            Self::String(_)
            | Self::TaggedString(_, _, _)
            | Self::TypedValue(_)
            | Self::TypedLiteral(_, _) => TermKind::Literal,
        }
    }

    pub fn value_str(&self) -> Cow<'_, str> {
        Cow::Borrowed(match self {
            Self::Iri(s) => s.as_str(),
            Self::BNode(s) => s.as_str(),
            Self::String(s) | Self::TaggedString(s, _, _) | Self::TypedLiteral(s, _) => s.as_str(),
            Self::TypedValue(Value::Primitive(PrimitiveValue::Boolean(Boolean::FALSE))) => "false",
            Self::TypedValue(Value::Primitive(PrimitiveValue::Boolean(Boolean::TRUE))) => "true",
            Self::TypedValue(Value::Primitive(
                PrimitiveValue::String(s) | PrimitiveValue::AnyUri(s),
            )) => s.as_str(),
            Self::TypedValue(v) => return Cow::Owned(v.to_string()),
        })
    }
}

impl Term for HeapTerm {
    fn kind(&self) -> TermKind {
        self.kind()
    }

    fn value_str(&self) -> Cow<'_, str> {
        self.value_str()
    }
}

impl Term for &HeapTerm {
    fn kind(&self) -> TermKind {
        (*self).kind()
    }

    fn value_str(&self) -> Cow<'_, str> {
        (*self).value_str()
    }
}

impl From<&dyn Term> for HeapTerm {
    fn from(term: &dyn Term) -> Self {
        match term.kind() {
            TermKind::Iri => Self::iri(term.value_str()),
            TermKind::BNode => Self::bnode(term.value_str()),
            TermKind::Literal => Self::string(term.value_str()), // FIXME
        }
    }
}

impl From<CowTerm<'_>> for HeapTerm {
    fn from(input: CowTerm<'_>) -> Self {
        Self::from(&input)
    }
}

impl From<&CowTerm<'_>> for HeapTerm {
    fn from(input: &CowTerm<'_>) -> Self {
        match input {
            CowTerm::Iri(s) => HeapTerm::Iri(s.to_string()),
            CowTerm::BNode(s) => HeapTerm::BNode(s.to_string()),
            CowTerm::String(s) => HeapTerm::String(s.to_string()),
            CowTerm::TaggedString(s, lang, dir) => {
                HeapTerm::TaggedString(s.to_string(), lang.clone(), dir.clone())
            }
            CowTerm::TypedValue(v) => HeapTerm::TypedValue(v.clone()),
            CowTerm::TypedLiteral(s, dt) => HeapTerm::TypedLiteral(s.to_string(), dt.clone()),
        }
    }
}

impl From<&str> for HeapTerm {
    fn from(value: &str) -> Self {
        Self::string(value)
    }
}

impl From<String> for HeapTerm {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&String> for HeapTerm {
    fn from(value: &String) -> Self {
        Self::String(value.clone())
    }
}

impl From<Boolean> for HeapTerm {
    fn from(value: Boolean) -> Self {
        Self::TypedValue(value.into())
    }
}

impl From<(String, Language)> for HeapTerm {
    fn from((literal, language): (String, Language)) -> Self {
        Self::TaggedString(literal, language, None)
    }
}

impl From<(String, Datatype)> for HeapTerm {
    fn from((literal, datatype): (String, Datatype)) -> Self {
        let result = match &datatype {
            Datatype::Xsd(t) => xsd::parse(&literal, t),
            _ => Err(ParseError),
        };
        match result {
            Ok(value) => Self::TypedValue(value),
            Err(_) => Self::TypedLiteral(literal, datatype),
        }
    }
}

impl From<(String, Option<Datatype>, Option<Language>)> for HeapTerm {
    fn from((literal, datatype, language): (String, Option<Datatype>, Option<Language>)) -> Self {
        match (language, datatype) {
            (None, None) => Self::string(literal),
            (None, Some(datatype)) => Self::from((literal, datatype)),
            (Some(language), None) => Self::tagged_string(literal, language),
            (Some(language), Some(_)) => Self::tagged_string(literal, language), // TODO
        }
    }
}
