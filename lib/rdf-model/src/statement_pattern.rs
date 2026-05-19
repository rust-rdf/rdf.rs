// This is free and unencumbered software released into the public domain.

use crate::{HeapTerm, Term};

/// An RDF statement pattern.
///
/// See: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-statement
pub trait StatementPattern {
    type Term: Term;

    fn has_subject(&self) -> bool {
        self.subject().is_some()
    }

    fn has_predicate(&self) -> bool {
        self.predicate().is_some()
    }

    fn has_object(&self) -> bool {
        self.object().is_some()
    }

    fn has_context(&self) -> bool {
        self.context().is_some()
    }

    fn subject(&self) -> Option<&Self::Term> {
        None
    }

    fn predicate(&self) -> Option<&Self::Term> {
        None
    }

    fn object(&self) -> Option<&Self::Term> {
        None
    }

    fn context(&self) -> Option<&Self::Term> {
        None
    }
}

impl core::fmt::Debug for dyn StatementPattern<Term = HeapTerm> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("StatementPattern")
            .field("subject", &self.subject().map(|t| t.as_str()))
            .field("predicate", &self.predicate().map(|t| t.as_str()))
            .field("object", &self.object().map(|t| t.as_str()))
            .field("context", &self.context().map(|t| t.as_str()))
            .finish()
    }
}
