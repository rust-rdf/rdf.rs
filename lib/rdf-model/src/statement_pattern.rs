// This is free and unencumbered software released into the public domain.

use crate::{DEFAULT_GRAPH, DefaultGraph, QuadPattern, Statement, Term, TriplePattern};

/// An RDF statement pattern.
///
/// See: <https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-statement>
pub trait StatementPattern {
    type Term: Term + Clone;

    /// Matches borrowed terms using their declared complete-term equality.
    ///
    /// No lexical coercion, conversion, or allocation is performed. Candidate
    /// terms must be explicitly comparable to the pattern's term type. Unbound
    /// slots match any term. An unbound context matches any graph; a default-graph
    /// marker matches `None` or another default-graph marker. Other contexts match
    /// only an equal named-graph term. Missing contexts never cause a panic.
    fn matches<T: Term + ?Sized>(
        &self,
        subject: &T,
        predicate: &T,
        object: &T,
        context: Option<&T>,
    ) -> bool
    where
        Self::Term: PartialEq<T>,
    {
        if let Some(s) = self.subject() {
            if s != subject {
                return false;
            }
        }
        if let Some(p) = self.predicate() {
            if p != predicate {
                return false;
            }
        }
        if let Some(o) = self.object() {
            if o != object {
                return false;
            }
        }
        if self.is_default_graph() {
            return context.is_none_or(Term::is_default_graph);
        }
        match self.context() {
            None => true,
            Some(graph) => {
                context.is_some_and(|actual| !actual.is_default_graph() && graph == actual)
            },
        }
    }

    /// Matches a complete statement without cloning its terms or allocating.
    fn matches_statement<S: Statement + ?Sized>(&self, statement: &S) -> bool
    where
        Self::Term: PartialEq<S::Term>,
    {
        self.matches(
            statement.subject(),
            statement.predicate(),
            statement.object(),
            statement.context(),
        )
    }

    /// Whether this pattern selects only the default graph.
    ///
    /// Normally indicated by the singleton in [`Self::context`]. Concrete quads
    /// also select their exact graph and override this for an implicit `None`.
    fn is_default_graph(&self) -> bool {
        self.context().is_some_and(Term::is_default_graph)
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
        self.is_default_graph() || self.context().is_some()
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

    /// Returns the explicit graph constraint: a marker for the default graph or
    /// a term for a named graph. `None` means a wildcard unless
    /// [`Self::is_default_graph`] indicates an implicit default-graph statement.
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

    /// Copies every constraint, including an implicit default graph, into an
    /// owned pattern. The term type must support the default-graph singleton.
    fn to_quad_pattern(&self) -> QuadPattern<Self::Term>
    where
        Self::Term: From<DefaultGraph>,
    {
        QuadPattern::new(
            self.subject().cloned(),
            self.predicate().cloned(),
            self.object().cloned(),
            if self.is_default_graph() {
                Some(DEFAULT_GRAPH.into())
            } else {
                self.context().cloned()
            },
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
