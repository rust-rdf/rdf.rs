// This is free and unencumbered software released into the public domain.

use alloc::{borrow::Cow, string::ToString};
use rdf_model::{HeapTerm, Term, TermHash, TermKind};
use serde_json::Value;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ValkeyTerm(pub(crate) Value);

impl ValkeyTerm {
    pub fn to_b3(&self) -> TermHash {
        HeapTerm::from(self).to_b3()
    }
}

impl From<HeapTerm> for ValkeyTerm {
    fn from(input: HeapTerm) -> Self {
        ValkeyTerm(input.into_json())
    }
}

impl From<&HeapTerm> for ValkeyTerm {
    fn from(input: &HeapTerm) -> Self {
        input.clone().into()
    }
}

impl From<ValkeyTerm> for HeapTerm {
    fn from(input: ValkeyTerm) -> Self {
        (&input).into()
    }
}

impl From<&ValkeyTerm> for HeapTerm {
    fn from(input: &ValkeyTerm) -> Self {
        match input.0 {
            Value::Null => unimplemented!(),
            Value::Bool(b) => HeapTerm::TypedValue(b.into()),
            Value::Number(ref n) => HeapTerm::TypedValue(n.as_f64().unwrap().into()), // FIXME
            Value::String(ref s) if s.starts_with("_:") => HeapTerm::BNode(s.clone()),
            Value::String(ref s) => HeapTerm::Iri(s.clone()),
            Value::Array(_) => todo!(),  // TODO
            Value::Object(_) => todo!(), // TODO
        }
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
