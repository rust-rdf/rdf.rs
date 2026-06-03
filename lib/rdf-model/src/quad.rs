// This is free and unencumbered software released into the public domain.

use crate::{CowTerm, HeapTerm, QuadPattern, Statement, Term, Triple, TriplePattern};

/// A quad statement.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Quad<T: Term> {
    pub(crate) s: T,
    pub(crate) p: T,
    pub(crate) o: T,
    pub(crate) g: Option<T>,
}

impl<T: Term> Quad<T> {
    pub const fn new(s: T, p: T, o: T, g: Option<T>) -> Self {
        Self { s, p, o, g }
    }

    pub fn into_inner(self) -> (T, T, T, Option<T>) {
        (self.s, self.p, self.o, self.g)
    }
}

impl<T: Term + Clone> Statement for Quad<T> {
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

    fn context(&self) -> Option<&Self::Term> {
        self.g.as_ref()
    }
}

impl<T: Term + Clone> Quad<T> {
    pub fn to_triple(&self) -> Triple<T> {
        Triple::new(self.s.clone(), self.p.clone(), self.o.clone())
    }

    pub fn to_triple_pattern(&self) -> TriplePattern<T> {
        TriplePattern::new(
            Some(self.s.clone()),
            Some(self.p.clone()),
            Some(self.o.clone()),
        )
    }

    pub fn to_quad_pattern(&self) -> QuadPattern<T> {
        QuadPattern::new(
            Some(self.s.clone()),
            Some(self.p.clone()),
            Some(self.o.clone()),
            self.g.clone(),
        )
    }
}

impl From<Triple<CowTerm<'_>>> for Quad<HeapTerm> {
    fn from(input: Triple<CowTerm<'_>>) -> Self {
        Self::new(input.s.into(), input.p.into(), input.o.into(), None)
    }
}

impl<'a> From<&'a Triple<CowTerm<'a>>> for Quad<HeapTerm> {
    fn from(input: &'a Triple<CowTerm<'a>>) -> Self {
        Self::new(
            (&input.s).into(),
            (&input.p).into(),
            (&input.o).into(),
            None,
        )
    }
}

impl From<Quad<CowTerm<'_>>> for Quad<HeapTerm> {
    fn from(input: Quad<CowTerm<'_>>) -> Self {
        Self::new(
            input.s.into(),
            input.p.into(),
            input.o.into(),
            input.g.map(|g| g.into()),
        )
    }
}

impl<'a> From<&'a Quad<CowTerm<'a>>> for Quad<HeapTerm> {
    fn from(input: &'a Quad<CowTerm<'a>>) -> Self {
        Self::new(
            (&input.s).into(),
            (&input.p).into(),
            (&input.o).into(),
            input.g.as_ref().map(|g| g.into()),
        )
    }
}

impl<'a> From<Triple<HeapTerm>> for Quad<CowTerm<'a>> {
    fn from(input: Triple<HeapTerm>) -> Self {
        Self::new(input.s.into(), input.p.into(), input.o.into(), None)
    }
}

impl<'a> From<&'a Triple<HeapTerm>> for Quad<CowTerm<'a>> {
    fn from(input: &'a Triple<HeapTerm>) -> Self {
        Self::new(
            (&input.s).into(),
            (&input.p).into(),
            (&input.o).into(),
            None,
        )
    }
}

impl<'a> From<Quad<HeapTerm>> for Quad<CowTerm<'a>> {
    fn from(input: Quad<HeapTerm>) -> Self {
        Self::new(input.s.into(), input.p.into(), input.o.into(), None)
    }
}

impl<'a> From<&'a Quad<HeapTerm>> for Quad<CowTerm<'a>> {
    fn from(input: &'a Quad<HeapTerm>) -> Self {
        Self::new(
            (&input.s).into(),
            (&input.p).into(),
            (&input.o).into(),
            input.g.as_ref().map(|g| g.into()),
        )
    }
}

impl<T: Term> From<(T, T, T)> for Quad<T> {
    fn from((s, p, o): (T, T, T)) -> Self {
        Self::new(s, p, o, None)
    }
}

impl<T: Term + Clone> From<(&T, &T, &T)> for Quad<T> {
    fn from((s, p, o): (&T, &T, &T)) -> Self {
        Self::new(s.clone(), p.clone(), o.clone(), None)
    }
}

impl<T: Term> From<(T, T, T, T)> for Quad<T> {
    fn from((s, p, o, g): (T, T, T, T)) -> Self {
        Self::new(s, p, o, Some(g))
    }
}

impl<T: Term> From<(T, T, T, Option<T>)> for Quad<T> {
    fn from((s, p, o, g): (T, T, T, Option<T>)) -> Self {
        Self::new(s, p, o, g)
    }
}

impl<T: Term + Clone> From<(&T, &T, &T, &T)> for Quad<T> {
    fn from((s, p, o, g): (&T, &T, &T, &T)) -> Self {
        Self::new(s.clone(), p.clone(), o.clone(), Some(g.clone()))
    }
}

impl<T: Term + Clone> From<(&T, &T, &T, &Option<T>)> for Quad<T> {
    fn from((s, p, o, g): (&T, &T, &T, &Option<T>)) -> Self {
        Self::new(s.clone(), p.clone(), o.clone(), g.clone())
    }
}

impl<T: Term + Clone> From<(&T, &T, &T, Option<&T>)> for Quad<T> {
    fn from((s, p, o, g): (&T, &T, &T, Option<&T>)) -> Self {
        Self::new(s.clone(), p.clone(), o.clone(), g.cloned())
    }
}
