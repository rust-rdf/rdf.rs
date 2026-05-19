// This is free and unencumbered software released into the public domain.

use crate::{Quad, QuadPattern, StatementPattern, Term};

/// A triple statement pattern.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TriplePattern<T: Term> {
    s: Option<T>,
    p: Option<T>,
    o: Option<T>,
}

impl<T: Term> TriplePattern<T> {
    pub const fn empty() -> Self {
        Self::new(None, None, None)
    }

    pub const fn new(s: Option<T>, p: Option<T>, o: Option<T>) -> Self {
        Self { s, p, o }
    }

    pub const fn with_subject(s: T) -> Self {
        Self::new(Some(s), None, None)
    }

    pub const fn with_predicate(p: T) -> Self {
        Self::new(None, Some(p), None)
    }

    pub const fn with_object(o: T) -> Self {
        Self::new(None, None, Some(o))
    }
}

impl<T: Term + Clone> TriplePattern<T> {
    pub fn to_quad_pattern(&self) -> QuadPattern<T> {
        QuadPattern::new(self.s.clone(), self.p.clone(), self.o.clone(), None)
    }
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
