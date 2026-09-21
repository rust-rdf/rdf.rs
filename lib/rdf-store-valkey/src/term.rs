// This is free and unencumbered software released into the public domain.

use alloc::{borrow::Cow, string::ToString};
use rdf_hash::TermHash;
use rdf_model::{DEFAULT_GRAPH_URN, DefaultGraph, HeapTerm, Term, TermKind};
use serde_json::Value;

/// A term that can be stored in Valkey.
/// The default-graph marker is distinguished internally from its external URN.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ValkeyTerm(pub(crate) Value, pub(crate) bool);

impl From<DefaultGraph> for ValkeyTerm {
    fn from(_: DefaultGraph) -> Self {
        Self(Value::String(DEFAULT_GRAPH_URN.into()), true)
    }
}

impl<T> From<&T> for ValkeyTerm
where
    T: Clone + Into<Self>,
{
    fn from(t: &T) -> Self {
        t.clone().into()
    }
}

impl From<HeapTerm> for ValkeyTerm {
    fn from(input: HeapTerm) -> Self {
        let default_graph = input.is_default_graph();
        ValkeyTerm(input.into_json(), default_graph)
    }
}

impl TryFrom<ValkeyTerm> for HeapTerm {
    type Error = ();

    fn try_from(input: ValkeyTerm) -> Result<Self, Self::Error> {
        if input.is_default_graph() {
            Ok(HeapTerm::DefaultGraph)
        } else {
            HeapTerm::try_from(input.0)
        }
    }
}

impl TryFrom<&ValkeyTerm> for HeapTerm {
    type Error = ();

    fn try_from(input: &ValkeyTerm) -> Result<Self, Self::Error> {
        if input.is_default_graph() {
            Ok(HeapTerm::DefaultGraph)
        } else {
            HeapTerm::try_from(&input.0)
        }
    }
}

impl TryFrom<&ValkeyTerm> for TermHash {
    type Error = ();

    fn try_from(input: &ValkeyTerm) -> Result<Self, Self::Error> {
        HeapTerm::try_from(input).map(TermHash::from)
    }
}

impl Term for ValkeyTerm {
    fn kind(&self) -> TermKind {
        if self.1 {
            return TermKind::DefaultGraph;
        }
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
