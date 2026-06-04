// This is free and unencumbered software released into the public domain.

use crate::{CowTerm, HeapTerm, Quad, QuadPattern, Statement, Term, TriplePattern};

pub type TripleSlot = crate::StatementSlot;

/// A triple statement.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Triple<T: Term> {
    pub(crate) s: T,
    pub(crate) p: T,
    pub(crate) o: T,
}

impl<T: Term> Triple<T> {
    pub const fn new(s: T, p: T, o: T) -> Self {
        Self { s, p, o }
    }

    pub fn with_subject(self, s: impl Into<T>) -> Self {
        Self {
            s: s.into(),
            ..self
        }
    }

    pub fn with_predicate(self, p: impl Into<T>) -> Self {
        Self {
            p: p.into(),
            ..self
        }
    }

    pub fn with_object(self, o: impl Into<T>) -> Self {
        Self {
            o: o.into(),
            ..self
        }
    }

    pub fn with_context(self, g: impl Into<Option<T>>) -> Quad<T> {
        Quad::new(self.s, self.p, self.o, g.into())
    }

    pub fn into_inner(self) -> (T, T, T) {
        (self.s, self.p, self.o)
    }
}

impl<T: Term + Clone> Statement for Triple<T> {
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

impl<T: Term + Clone> Triple<T> {
    pub fn to_triple_pattern(&self) -> TriplePattern<T> {
        TriplePattern::new(
            Some(self.s.clone()),
            Some(self.p.clone()),
            Some(self.o.clone()),
        )
    }

    pub fn to_quad(&self) -> Quad<T> {
        Quad::new(self.s.clone(), self.p.clone(), self.o.clone(), None)
    }

    pub fn to_quad_pattern(&self) -> QuadPattern<T> {
        QuadPattern::new(
            Some(self.s.clone()),
            Some(self.p.clone()),
            Some(self.o.clone()),
            None,
        )
    }
}

impl From<Triple<CowTerm<'_>>> for Triple<HeapTerm> {
    fn from(input: Triple<CowTerm<'_>>) -> Self {
        Self::new(input.s.into(), input.p.into(), input.o.into())
    }
}

impl<'a> From<&'a Triple<CowTerm<'a>>> for Triple<HeapTerm> {
    fn from(input: &'a Triple<CowTerm<'a>>) -> Self {
        Self::new((&input.s).into(), (&input.p).into(), (&input.o).into())
    }
}

impl From<Quad<CowTerm<'_>>> for Triple<HeapTerm> {
    fn from(input: Quad<CowTerm<'_>>) -> Self {
        Self::new(input.s.into(), input.p.into(), input.o.into())
    }
}

impl<'a> From<&'a Quad<CowTerm<'a>>> for Triple<HeapTerm> {
    fn from(input: &'a Quad<CowTerm<'a>>) -> Self {
        Self::new((&input.s).into(), (&input.p).into(), (&input.o).into())
    }
}

impl<'a> From<Triple<HeapTerm>> for Triple<CowTerm<'a>> {
    fn from(input: Triple<HeapTerm>) -> Self {
        Self::new(input.s.into(), input.p.into(), input.o.into())
    }
}

impl<'a> From<&'a Triple<HeapTerm>> for Triple<CowTerm<'a>> {
    fn from(input: &'a Triple<HeapTerm>) -> Self {
        Self::new((&input.s).into(), (&input.p).into(), (&input.o).into())
    }
}

impl<'a> From<Quad<HeapTerm>> for Triple<CowTerm<'a>> {
    fn from(input: Quad<HeapTerm>) -> Self {
        Self::new(input.s.into(), input.p.into(), input.o.into())
    }
}

impl<'a> From<&'a Quad<HeapTerm>> for Triple<CowTerm<'a>> {
    fn from(input: &'a Quad<HeapTerm>) -> Self {
        Self::new((&input.s).into(), (&input.p).into(), (&input.o).into())
    }
}

impl<T: Term> From<(T, T, T)> for Triple<T> {
    fn from((s, p, o): (T, T, T)) -> Self {
        Self::new(s, p, o)
    }
}

impl<T: Term + Clone> From<(&T, &T, &T)> for Triple<T> {
    fn from((s, p, o): (&T, &T, &T)) -> Self {
        Self::new(s.clone(), p.clone(), o.clone())
    }
}

impl<T: Term> TryFrom<TriplePattern<T>> for Triple<T> {
    type Error = ();

    fn try_from(input: TriplePattern<T>) -> Result<Self, Self::Error> {
        if !input.is_constant() {
            return Err(());
        }
        Ok(Self::new(
            input.s.unwrap(),
            input.p.unwrap(),
            input.o.unwrap(),
        ))
    }
}

impl<T: Term> TryFrom<QuadPattern<T>> for Triple<T> {
    type Error = ();

    fn try_from(input: QuadPattern<T>) -> Result<Self, Self::Error> {
        if !input.is_constant() {
            return Err(());
        }
        Ok(Self::new(
            input.s.unwrap(),
            input.p.unwrap(),
            input.o.unwrap(),
        ))
    }
}
