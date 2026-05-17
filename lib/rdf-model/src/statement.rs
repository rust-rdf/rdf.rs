// This is free and unencumbered software released into the public domain.

use crate::{HeapTerm, Term, providers};

/// An RDF statement.
///
/// See: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-statement
pub trait Statement {
    type Term: Term;

    // TODO: associated type defaults (https://github.com/rust-lang/rust/issues/29661)
    //type Term: Term = &dyn Term;

    fn subject(&self) -> &Self::Term;

    fn predicate(&self) -> &Self::Term;

    fn object(&self) -> &Self::Term;

    fn context(&self) -> Option<&Self::Term> {
        None
    }
}

impl core::fmt::Debug for dyn Statement<Term = HeapTerm> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Statement")
            .field("subject", &self.subject().as_str())
            .field("predicate", &self.predicate().as_str())
            .field("object", &self.object().as_str())
            .field("context", &self.context().map(|t| t.as_str()))
            .finish()
    }
}
