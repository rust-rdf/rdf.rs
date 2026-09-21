// This is free and unencumbered software released into the public domain.

use crate::{
    ValkeyError, ValkeyQuad, ValkeyTerm, ValkeyTripleId, ValkeyTripleKey, ValkeyTriplePattern,
};
use rdf_model::{HeapTriple, Quad, QuadPattern, Statement, TripleSlot};
use serde_json::Value;

/// A triple statement (S, P, O) in Valkey.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ValkeyTriple(pub(crate) ValkeyTripleId, pub(crate) Quad<ValkeyTerm>);

impl ValkeyTriple {
    pub fn with_context(self, g: impl Into<Option<ValkeyTerm>>) -> ValkeyQuad {
        ValkeyQuad(self, None).with_context(g)
    }

    pub fn id(&self) -> &ValkeyTripleId {
        &self.0
    }

    pub fn key(&self) -> ValkeyTripleKey {
        ValkeyTripleKey::from(self.id())
    }
}

impl Statement for ValkeyTriple {
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
        None
    }

    fn to_quad_pattern(&self) -> QuadPattern<Self::Term> {
        self.to_triple_pattern().to_quad_pattern()
    }
}

impl<T> From<&T> for ValkeyTriple
where
    T: Clone + Into<Self>,
{
    fn from(t: &T) -> Self {
        t.clone().into()
    }
}

impl From<ValkeyQuad> for ValkeyTriple {
    fn from(input: ValkeyQuad) -> Self {
        input.0
    }
}

impl From<HeapTriple> for ValkeyTriple {
    fn from(input: HeapTriple) -> Self {
        let (s, p, o) = input.into_inner();
        let id = ValkeyTripleId::from(((&s).into(), (&p).into(), (&o).into()));
        Self(id, Quad::new(s.into(), p.into(), o.into(), None))
    }
}

impl From<ValkeyTriple> for Value {
    fn from(input: ValkeyTriple) -> Self {
        let (s, p, o, _) = input.1.into_inner();
        let mut output = serde_json::Map::new();
        output.insert("s".into(), s.0);
        output.insert("p".into(), p.0);
        output.insert("o".into(), o.0);
        Value::Object(output)
    }
}

impl From<&ValkeyTriple> for Value {
    fn from(input: &ValkeyTriple) -> Self {
        input.clone().into()
    }
}

/// Projects a subject/predicate/object-bound pattern to a graphless triple.
impl TryFrom<ValkeyTriplePattern> for ValkeyTriple {
    type Error = ValkeyError;

    fn try_from(input: ValkeyTriplePattern) -> Result<Self, Self::Error> {
        let (Some(s), Some(p), Some(o), _) = input.matcher.into_inner() else {
            return Err(ValkeyError::UnboundPattern);
        };
        Ok(Self(input.glob, Quad::new(s, p, o, None)))
    }
}

impl TryFrom<(ValkeyTripleId, Value)> for ValkeyTriple {
    type Error = ValkeyError;

    fn try_from((id, input): (ValkeyTripleId, Value)) -> Result<Self, Self::Error> {
        match input {
            Value::Object(mut input) => {
                let s = ValkeyTerm(
                    input.remove("s").ok_or_else(|| {
                        ValkeyError::InvalidTripleTerm(id.clone(), TripleSlot::Subject)
                    })?,
                    false,
                );
                let p = ValkeyTerm(
                    input.remove("p").ok_or_else(|| {
                        ValkeyError::InvalidTripleTerm(id.clone(), TripleSlot::Predicate)
                    })?,
                    false,
                );
                let o = ValkeyTerm(
                    input.remove("o").ok_or_else(|| {
                        ValkeyError::InvalidTripleTerm(id.clone(), TripleSlot::Object)
                    })?,
                    false,
                );
                Ok(Self(id, Quad::new(s, p, o, None)))
            },
            _ => Err(ValkeyError::InvalidTriple(id)),
        }
    }
}
