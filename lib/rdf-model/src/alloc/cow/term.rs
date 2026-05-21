// This is free and unencumbered software released into the public domain.

extern crate alloc;

use crate::{BaseDirection, Datatype, Term, TermKind};
use alloc::{
    borrow::{Cow, ToOwned},
    string::{String, ToString},
};
use xsd::{PrimitiveValue, Value};

type Language = String; // TODO

/// A clone-on-write term.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
// #[cfg_attr(
//     feature = "borsh",
//     derive(borsh::BorshSerialize, borsh::BorshDeserialize)
// )]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CowTerm<'a> {
    /// An IRI or IRI reference term.
    Iri(Cow<'a, str>),

    /// A blank node (aka bnode) term.
    BNode(Cow<'a, str>),

    /// A plain string term.
    /// The term's datatype is `xsd:string`.
    String(Cow<'a, str>),

    /// A language-tagged string term.
    /// The term's datatype is either `rdf:langString` or `rdf:dirLangString`.
    TaggedString(Cow<'a, str>, Language, Option<BaseDirection>),

    /// A typed value term.
    TypedValue(Value),

    /// A typed literal term that could not be instantiated as a typed value.
    /// The term is either an ill-typed literal and/or an unsupported datatype.
    TypedLiteral(Cow<'a, str>, Datatype),
}

impl<'a> CowTerm<'a> {
    pub const fn static_iri(value: &'static str) -> Self {
        Self::Iri(Cow::Borrowed(value))
    }

    pub const fn static_string(value: &'static str) -> Self {
        Self::String(Cow::Borrowed(value))
    }

    pub fn iri(value: impl Into<Cow<'a, str>>) -> Self {
        Self::Iri(value.into())
    }

    pub fn bnode(id: impl Into<Cow<'a, str>>) -> Self {
        Self::BNode(id.into())
    }

    pub fn string(value: impl Into<Cow<'a, str>>) -> Self {
        Self::String(value.into())
    }

    pub fn tagged_string(value: impl Into<Cow<'a, str>>, tag: impl Into<Language>) -> Self {
        Self::TaggedString(value.into(), tag.into(), None)
    }

    pub fn typed_value(value: impl Into<Value>) -> Self {
        Self::TypedValue(value.into())
    }

    pub fn typed_literal(literal: impl Into<Cow<'a, str>>, datatype: impl Into<Datatype>) -> Self {
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
            Self::Iri(s) => s,
            Self::BNode(s) => s,
            Self::String(s) | Self::TaggedString(s, _, _) | Self::TypedLiteral(s, _) => s,
            Self::TypedValue(Value::Primitive(PrimitiveValue::Boolean(false))) => "false",
            Self::TypedValue(Value::Primitive(PrimitiveValue::Boolean(true))) => "true",
            Self::TypedValue(Value::Primitive(
                PrimitiveValue::String(s) | PrimitiveValue::AnyUri(s),
            )) => s,
            Self::TypedValue(v) => return Cow::Owned(ToString::to_string(&v)),
        })
    }
}

impl<'a> Term for CowTerm<'a> {
    fn kind(&self) -> TermKind {
        self.kind()
    }

    fn value_str(&self) -> Cow<'_, str> {
        self.value_str()
    }
}

impl<'a> Term for &CowTerm<'a> {
    fn kind(&self) -> TermKind {
        (*self).kind()
    }

    fn value_str(&self) -> Cow<'_, str> {
        (*self).value_str()
    }
}

impl<'a> From<&'a dyn Term> for CowTerm<'a> {
    fn from(term: &'a dyn Term) -> Self {
        match term.kind() {
            TermKind::Iri => Self::iri(Cow::from(term.value_str())),
            TermKind::BNode => Self::bnode(Cow::from(term.value_str())),
            TermKind::Literal => Self::string(Cow::from(term.value_str())), // TODO
        }
    }
}

impl<'a> From<&'a str> for CowTerm<'a> {
    fn from(value: &'a str) -> Self {
        Self::string(Cow::from(value))
    }
}

impl<'a> From<String> for CowTerm<'a> {
    fn from(value: String) -> Self {
        Self::String(Cow::from(value))
    }
}

impl<'a> From<&'a String> for CowTerm<'a> {
    fn from(value: &'a String) -> Self {
        Self::String(Cow::from(value))
    }
}
