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

    fn context(&self) -> Option<&Self::Term> {
        self.g.as_ref()
    }
}

impl From<(HeapTerm, HeapTerm, HeapTerm)> for HeapQuad {
    fn from((s, p, o): (HeapTerm, HeapTerm, HeapTerm)) -> Self {
        Self { s, p, o, g: None }
    }
}

impl From<(&HeapTerm, &HeapTerm, &HeapTerm)> for HeapQuad {
    fn from((s, p, o): (&HeapTerm, &HeapTerm, &HeapTerm)) -> Self {
        Self {
            s: s.clone(),
            p: p.clone(),
            o: o.clone(),
            g: None,
        }
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

impl From<(&HeapTerm, &HeapTerm, &HeapTerm, &HeapTerm)> for HeapQuad {
    fn from((s, p, o, g): (&HeapTerm, &HeapTerm, &HeapTerm, &HeapTerm)) -> Self {
        Self {
            s: s.clone(),
            p: p.clone(),
            o: o.clone(),
            g: Some(g.clone()),
        }
    }
}
