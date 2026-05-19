// This is free and unencumbered software released into the public domain.

use crate::{StatementPattern, Term};

/// A triple statement pattern.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TriplePattern<T: Term> {
    s: Option<T>,
    p: Option<T>,
    o: Option<T>,
}

impl<T: Term> StatementPattern for TriplePattern<T> {
    type Term = T;

    fn subject(&self) -> Option<&Self::Term> {
        self.s.as_ref()
    }

    fn predicate(&self) -> Option<&Self::Term> {
        self.p.as_ref()
    }

    fn object(&self) -> Option<&Self::Term> {
        self.o.as_ref()
    }

    fn context(&self) -> Option<&Self::Term> {
        None
    }
}

impl<T: Term> From<(T, T, T)> for TriplePattern<T> {
    fn from((s, p, o): (T, T, T)) -> Self {
        Self::from((Some(s), Some(p), Some(o)))
    }
}

impl<T: Term> From<(Option<T>, Option<T>, Option<T>)> for TriplePattern<T> {
    fn from((s, p, o): (Option<T>, Option<T>, Option<T>)) -> Self {
        Self { s, p, o }
    }
}

impl<T: Term + Clone> From<(&T, &T, &T)> for TriplePattern<T> {
    fn from((s, p, o): (&T, &T, &T)) -> Self {
        Self::from((Some(s), Some(p), Some(o)))
    }
}

impl<T: Term + Clone> From<(Option<&T>, Option<&T>, Option<&T>)> for TriplePattern<T> {
    fn from((s, p, o): (Option<&T>, Option<&T>, Option<&T>)) -> Self {
        Self {
            s: s.cloned(),
            p: p.cloned(),
            o: o.cloned(),
        }
    }
}
