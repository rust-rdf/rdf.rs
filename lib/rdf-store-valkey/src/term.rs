// This is free and unencumbered software released into the public domain.

use alloc::{borrow::Cow, string::ToString};
use rdf_model::{HeapTerm, Term, TermHash, TermKind};
use serde_json::Value;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ValkeyTerm(pub(crate) Value);

impl ValkeyTerm {
    pub fn to_b3(&self) -> TermHash {
        HeapTerm::try_from(self).unwrap().to_b3()
    }
}

impl From<HeapTerm> for ValkeyTerm {
    fn from(input: HeapTerm) -> Self {
        ValkeyTerm(input.into_json())
    }
}

impl From<&HeapTerm> for ValkeyTerm {
    fn from(input: &HeapTerm) -> Self {
        ValkeyTerm(input.clone().into_json())
    }
}

impl TryFrom<ValkeyTerm> for HeapTerm {
    type Error = ();

    fn try_from(input: ValkeyTerm) -> Result<Self, Self::Error> {
        HeapTerm::try_from(input.0)
    }
}

impl TryFrom<&ValkeyTerm> for HeapTerm {
    type Error = ();

    fn try_from(input: &ValkeyTerm) -> Result<Self, Self::Error> {
        HeapTerm::try_from(&input.0)
    }
}

impl Term for ValkeyTerm {
    fn kind(&self) -> TermKind {
        match self.0 {
            Value::Null => TermKind::Literal,
            Value::Bool(_) => TermKind::Literal,
            Value::Number(_) => TermKind::Literal,
            Value::String(ref s) if s.starts_with("_:") => TermKind::BNode,
            Value::String(_) => TermKind::Iri,
            Value::Array(_) => TermKind::Literal,
            Value::Object(_) => TermKind::Literal,
        }
    }

    fn value_str(&self) -> Cow<'_, str> {
        match self.0 {
            Value::String(ref s) => Cow::Owned(s.clone()),
            _ => Cow::Owned(self.0.to_string()), // TODO
        }
    }
}
