// This is free and unencumbered software released into the public domain.

use crate::{HeapTerm, Statement, Term};

/// A heap-allocated quad statement.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HeapQuad {
    s: HeapTerm,
    p: HeapTerm,
    o: HeapTerm,
    g: Option<HeapTerm>,
}

impl Statement for HeapQuad {
    fn subject(&self) -> &dyn Term {
        &self.s
    }

    fn predicate(&self) -> &dyn Term {
        &self.p
    }

    fn object(&self) -> &dyn Term {
        &self.o
    }

    fn context(&self) -> Option<&dyn Term> {
        self.g.as_ref().map(|g| g as &dyn Term)
    }
}

impl From<(HeapTerm, HeapTerm, HeapTerm)> for HeapQuad {
    fn from((s, p, o): (HeapTerm, HeapTerm, HeapTerm)) -> Self {
        Self { s, p, o, g: None }
    }
}

impl From<(HeapTerm, HeapTerm, HeapTerm, HeapTerm)> for HeapQuad {
    fn from((s, p, o, g): (HeapTerm, HeapTerm, HeapTerm, HeapTerm)) -> Self {
        Self {
            s,
            p,
            o,
            g: Some(g),
        }
    }
}
