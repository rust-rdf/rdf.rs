// This is free and unencumbered software released into the public domain.

use crate::{
    ValkeyError, ValkeyTerm, ValkeyTriple, ValkeyTripleId, ValkeyTripleKey, ValkeyTriplePattern,
};
use rdf_model::Statement;

/// A quad statement (S, P, O, C) in Valkey.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ValkeyQuad(pub(crate) ValkeyTriple, pub(crate) Option<ValkeyTerm>);

impl ValkeyQuad {
    pub fn with_context(self, g: impl Into<Option<ValkeyTerm>>) -> Self {
        Self(self.0, g.into())
    }

    pub fn id(&self) -> &ValkeyTripleId {
        self.0.id()
    }

    pub fn key(&self) -> ValkeyTripleKey {
        self.0.key()
    }
}

impl Statement for ValkeyQuad {
    type Term = ValkeyTerm;

    fn subject(&self) -> &Self::Term {
        self.0.subject()
    }

    fn predicate(&self) -> &Self::Term {
        self.0.predicate()
    }

    fn object(&self) -> &Self::Term {
        self.0.object()
    }

    fn context(&self) -> Option<&Self::Term> {
        self.1.as_ref()
    }
}

impl From<ValkeyTriple> for ValkeyQuad {
    fn from(input: ValkeyTriple) -> Self {
        Self(input, None)
    }
}

impl TryFrom<ValkeyTriplePattern> for ValkeyQuad {
    type Error = ValkeyError;

    fn try_from(input: ValkeyTriplePattern) -> Result<Self, Self::Error> {
        Ok(Self(input.try_into()?, None))
    }
}
