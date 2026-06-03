// This is free and unencumbered software released into the public domain.

use crate::{StatementPattern, Term, TriplePattern};

/// A quad statement pattern.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QuadPattern<T: Term> {
    pub(crate) s: Option<T>,
    pub(crate) p: Option<T>,
    pub(crate) o: Option<T>,
    pub(crate) g: Option<T>,
}

impl<T: Term> Default for QuadPattern<T> {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl<T: Term> QuadPattern<T> {
    pub const EMPTY: QuadPattern<T> = Self::empty();

    pub const fn empty() -> Self {
        Self::new(None, None, None, None)
    }

    pub const fn new(s: Option<T>, p: Option<T>, o: Option<T>, g: Option<T>) -> Self {
        Self { s, p, o, g }
    }

    pub const fn with_subject(s: T) -> Self {
        Self::new(Some(s), None, None, None)
    }

    pub const fn with_predicate(p: T) -> Self {
        Self::new(None, Some(p), None, None)
    }

    pub const fn with_object(o: T) -> Self {
        Self::new(None, None, Some(o), None)
    }

    pub const fn with_context(g: T) -> Self {
        Self::new(None, None, None, Some(g))
    }

    pub fn is_empty(&self) -> bool {
        self.s.is_none() && self.p.is_none() && self.o.is_none() && self.g.is_none()
    }

    pub fn is_constant(&self) -> bool {
        self.s.is_some() && self.p.is_some() && self.o.is_some() && self.g.is_some()
    }

    pub fn is_variable(&self) -> bool {
        !self.is_constant()
    }

    pub fn into_inner(self) -> (Option<T>, Option<T>, Option<T>, Option<T>) {
        (self.s, self.p, self.o, self.g)
    }
}

impl<T: Term + Clone> QuadPattern<T> {
    pub fn to_triple_pattern(&self) -> TriplePattern<T> {
        TriplePattern::new(self.s.clone(), self.p.clone(), self.o.clone())
    }
}

impl<T: Term + Clone> StatementPattern for QuadPattern<T> {
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
        self.g.as_ref()
    }
}

impl<T: Term> From<(T, T, T)> for QuadPattern<T> {
    fn from((s, p, o): (T, T, T)) -> Self {
        Self::from((Some(s), Some(p), Some(o)))
    }
}

impl<T: Term> From<(Option<T>, Option<T>, Option<T>)> for QuadPattern<T> {
    fn from((s, p, o): (Option<T>, Option<T>, Option<T>)) -> Self {
        Self { s, p, o, g: None }
    }
}

impl<T: Term + Clone> From<(&T, &T, &T)> for QuadPattern<T> {
    fn from((s, p, o): (&T, &T, &T)) -> Self {
        Self::from((Some(s), Some(p), Some(o)))
    }
}

impl<T: Term + Clone> From<(Option<&T>, Option<&T>, Option<&T>)> for QuadPattern<T> {
    fn from((s, p, o): (Option<&T>, Option<&T>, Option<&T>)) -> Self {
        Self {
            s: s.cloned(),
            p: p.cloned(),
            o: o.cloned(),
            g: None,
        }
    }
}

impl<T: Term> From<(T, T, T, T)> for QuadPattern<T> {
    fn from((s, p, o, g): (T, T, T, T)) -> Self {
        Self::from((Some(s), Some(p), Some(o), Some(g)))
    }
}

impl<T: Term> From<(Option<T>, Option<T>, Option<T>, Option<T>)> for QuadPattern<T> {
    fn from((s, p, o, g): (Option<T>, Option<T>, Option<T>, Option<T>)) -> Self {
        Self { s, p, o, g }
    }
}

impl<T: Term + Clone> From<(&T, &T, &T, &T)> for QuadPattern<T> {
    fn from((s, p, o, g): (&T, &T, &T, &T)) -> Self {
        Self::from((Some(s), Some(p), Some(o), Some(g)))
    }
}

impl<T: Term + Clone> From<(Option<&T>, Option<&T>, Option<&T>, Option<&T>)> for QuadPattern<T> {
    fn from((s, p, o, g): (Option<&T>, Option<&T>, Option<&T>, Option<&T>)) -> Self {
        Self {
            s: s.cloned(),
            p: p.cloned(),
            o: o.cloned(),
            g: g.cloned(),
        }
    }
}
