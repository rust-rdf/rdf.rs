// This is free and unencumbered software released into the public domain.

use crate::{HeapTerm, Statement, Term};

/// A heap-allocated triple statement.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HeapTriple {
    s: HeapTerm,
    p: HeapTerm,
    o: HeapTerm,
}

impl Statement for HeapTriple {
    type Term = HeapTerm;

    fn subject(&self) -> &Self::Term {
        &self.s
    }

    fn predicate(&self) -> &Self::Term {
        &self.p
    }

    fn object(&self) -> &Self::Term {
        &self.o
    }
}

impl From<(HeapTerm, HeapTerm, HeapTerm)> for HeapTriple {
    fn from((s, p, o): (HeapTerm, HeapTerm, HeapTerm)) -> Self {
        Self { s, p, o }
    }
}

impl From<(&HeapTerm, &HeapTerm, &HeapTerm)> for HeapTriple {
    fn from((s, p, o): (&HeapTerm, &HeapTerm, &HeapTerm)) -> Self {
        Self {
            s: s.clone(),
            p: p.clone(),
            o: o.clone(),
        }
    }
}
