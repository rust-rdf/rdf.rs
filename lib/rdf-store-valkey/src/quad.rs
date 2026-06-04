// This is free and unencumbered software released into the public domain.

use crate::{ValkeyError, ValkeyTerm, ValkeyTripleKey};
use alloc::{format, string::String};
use rdf_model::{HeapTriple, Quad, Statement};
use serde_json::Value;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ValkeyQuad(ValkeyTripleKey, Quad<ValkeyTerm>);

impl ValkeyQuad {
    pub fn with_context(self, g: Option<ValkeyTerm>) -> Self {
        Self(self.0, self.1.with_context(g))
    }

    pub fn id(&self) -> &String {
        &self.0.0
    }
}

impl From<HeapTriple> for ValkeyQuad {
    fn from(input: HeapTriple) -> Self {
        let (s, p, o) = input.into_inner();
        let id = format!(
            "{}:{}:{}",
            s.to_b3().to_hex()[0..16].to_lowercase(),
            p.to_b3().to_hex()[0..16].to_lowercase(),
            o.to_b3().to_hex()[0..16].to_lowercase()
        );
        Self(
            ValkeyTripleKey(id),
            Quad::new(
                ValkeyTerm(s.into_json()),
                ValkeyTerm(p.into_json()),
                ValkeyTerm(o.into_json()),
                None,
            ),
        )
    }
}

impl From<ValkeyQuad> for Value {
    fn from(input: ValkeyQuad) -> Self {
        let (s, p, o, _) = input.1.into_inner();
        let mut output = serde_json::Map::new();
        output.insert("s".into(), s.0);
        output.insert("p".into(), p.0);
        output.insert("o".into(), o.0);
        Value::Object(output)
    }
}

impl TryFrom<(ValkeyTripleKey, Value)> for ValkeyQuad {
    type Error = ValkeyError;

    fn try_from((key, input): (ValkeyTripleKey, Value)) -> Result<Self, Self::Error> {
        match input {
            Value::Object(mut input) => {
                let s = ValkeyTerm(
                    input
                        .remove("s")
                        .ok_or_else(|| ValkeyError::InvalidTerm(key.clone()))?,
                );
                let p = ValkeyTerm(
                    input
                        .remove("p")
                        .ok_or_else(|| ValkeyError::InvalidTerm(key.clone()))?,
                );
                let o = ValkeyTerm(
                    input
                        .remove("o")
                        .ok_or_else(|| ValkeyError::InvalidTerm(key.clone()))?,
                );
                Ok(Self(key, Quad::new(s, p, o, None)))
            },
            _ => Err(ValkeyError::InvalidTriple(key)),
        }
    }
}

impl Statement for ValkeyQuad {
    type Term = ValkeyTerm;

    fn subject(&self) -> &Self::Term {
        self.1.subject()
    }

    fn predicate(&self) -> &Self::Term {
        self.1.predicate()
    }

    fn object(&self) -> &Self::Term {
        self.1.object()
    }

    fn context(&self) -> Option<&Self::Term> {
        self.1.context()
    }
}
