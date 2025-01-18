// This is free and unencumbered software released into the public domain.

#[cfg(feature = "oxrdf")]
extern crate alloc;

use crate::{providers, Term};

#[cfg(feature = "oxrdf")]
use alloc::boxed::Box;

/// An RDF statement.
///
/// See: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-statement
pub trait Statement {
    fn subject(&self) -> &dyn Term;

    fn predicate(&self) -> &dyn Term;

    fn object(&self) -> &dyn Term;

    fn context(&self) -> Option<&dyn Term> {
        None
    }
}

impl core::fmt::Debug for dyn Statement {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Statement")
            .field("subject", &self.subject().as_str())
            .field("predicate", &self.predicate().as_str())
            .field("object", &self.object().as_str())
            .field("context", &self.context().map(|t| t.as_str()))
            .finish()
    }
}

#[cfg(feature = "oxrdf")]
impl From<oxrdf::Quad> for Box<dyn Statement> {
    fn from(quad: oxrdf::Quad) -> Self {
        Box::new(providers::OxrdfStatement::new(quad))
    }
}
