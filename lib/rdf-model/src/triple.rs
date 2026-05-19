// This is free and unencumbered software released into the public domain.

use crate::{Statement, Term};

/// A triple statement.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Triple<T: Term> {
    s: T,
    p: T,
    o: T,
}

impl<T: Term> Statement for Triple<T> {
    type Term = T;

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

impl<T: Term> From<(T, T, T)> for Triple<T> {
    fn from((s, p, o): (T, T, T)) -> Self {
        Self { s, p, o }
    }
}

impl<T: Term + Clone> From<(&T, &T, &T)> for Triple<T> {
    fn from((s, p, o): (&T, &T, &T)) -> Self {
        Self {
            s: s.clone(),
            p: p.clone(),
            o: o.clone(),
        }
    }
}
