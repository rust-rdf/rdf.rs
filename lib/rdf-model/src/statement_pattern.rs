// This is free and unencumbered software released into the public domain.

use crate::{HeapTerm, QuadPattern, Term, TriplePattern};

/// An RDF statement pattern.
///
/// See: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-statement
pub trait StatementPattern {
    type Term: Term + Clone;

    fn matches(
        &self,
        subject: impl Term,
        predicate: impl Term,
        object: impl Term,
        context: Option<impl Term>,
    ) -> bool {
        if let Some(s) = self.subject() {
            if s.value_str() != subject.value_str() {
                return false;
            }
        }
        if let Some(p) = self.predicate() {
            if p.value_str() != predicate.value_str() {
                return false;
            }
        }
        if let Some(o) = self.object() {
            if o.value_str() != object.value_str() {
                return false;
            }
        }
        if let Some(c) = self.context() {
            if c.value_str() != context.unwrap().value_str() {
                // FIXME
                return false;
            }
        }
        true
    }

    /// Whether the pattern has a constant subject.
    fn has_subject(&self) -> bool {
        self.subject().is_some()
    }

    /// Whether the pattern has a constant predicate.
    fn has_predicate(&self) -> bool {
        self.predicate().is_some()
    }

    /// Whether the pattern has a constant object.
    fn has_object(&self) -> bool {
        self.object().is_some()
    }

    /// Whether the pattern has a constant context (graph).
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

    fn to_triple_pattern(&self) -> TriplePattern<Self::Term> {
        TriplePattern::new(
            self.subject().cloned(),
            self.predicate().cloned(),
            self.object().cloned(),
        )
    }

    fn to_quad_pattern(&self) -> QuadPattern<Self::Term> {
        QuadPattern::new(
            self.subject().cloned(),
            self.predicate().cloned(),
            self.object().cloned(),
            self.context().cloned(),
        )
    }
}

// impl core::fmt::Debug for dyn StatementPattern<Term = HeapTerm> {
//     fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
//         f.debug_struct("StatementPattern")
//             .field("subject", &self.subject().map(|t| t.as_str()))
//             .field("predicate", &self.predicate().map(|t| t.as_str()))
//             .field("object", &self.object().map(|t| t.as_str()))
//             .field("context", &self.context().map(|t| t.as_str()))
//             .finish()
//     }
// }
