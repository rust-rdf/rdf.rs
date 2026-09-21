// This is free and unencumbered software released into the public domain.

#[cfg(feature = "alloc")]
use crate::HeapTerm;
use crate::{DEFAULT_GRAPH, DefaultGraph, Quad, QuadPattern, Term, Triple, TriplePattern};

/// An RDF statement.
///
/// See: <https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-statement>
pub trait Statement {
    type Term: Term + Clone;

    // TODO: associated type defaults (https://github.com/rust-lang/rust/issues/29661)
    //type Term: Term = &dyn Term;

    /// Whether the statement exposes a named graph rather than the default or
    /// an absent graph. Pattern graph constraints use `StatementPattern` instead.
    fn has_context(&self) -> bool {
        self.context().is_some()
    }

    fn subject(&self) -> &Self::Term;

    fn predicate(&self) -> &Self::Term;

    fn object(&self) -> &Self::Term;

    fn context(&self) -> Option<&Self::Term> {
        None
    }

    fn to_triple(&self) -> Triple<Self::Term> {
        Triple::new(
            self.subject().clone(),
            self.predicate().clone(),
            self.object().clone(),
        )
    }

    fn to_triple_pattern(&self) -> TriplePattern<Self::Term> {
        TriplePattern::new(
            Some(self.subject().clone()),
            Some(self.predicate().clone()),
            Some(self.object().clone()),
        )
    }

    fn to_quad(&self) -> Quad<Self::Term> {
        Quad::new(
            self.subject().clone(),
            self.predicate().clone(),
            self.object().clone(),
            self.context().cloned(),
        )
    }

    /// Produces an exact graph constraint, using the singleton for a default
    /// context. Graphless triples override this to preserve their wildcard graph.
    fn to_quad_pattern(&self) -> QuadPattern<Self::Term>
    where
        Self::Term: From<DefaultGraph>,
    {
        QuadPattern::new(
            Some(self.subject().clone()),
            Some(self.predicate().clone()),
            Some(self.object().clone()),
            Some(
                self.context()
                    .cloned()
                    .unwrap_or_else(|| DEFAULT_GRAPH.into()),
            ),
        )
    }
}

#[cfg(feature = "alloc")]
impl core::fmt::Debug for dyn Statement<Term = HeapTerm> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Statement")
            .field("subject", &self.subject().value_str())
            .field("predicate", &self.predicate().value_str())
            .field("object", &self.object().value_str())
            .field("context", &self.context().map(|t| t.value_str()))
            .finish()
    }
}
