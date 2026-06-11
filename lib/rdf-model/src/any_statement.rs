// This is free and unencumbered software released into the public domain.

use crate::{
    AnyTerm, CowTerm, EMPTY_COW_QUAD_PATTERN, EMPTY_HEAP_QUAD_PATTERN, HeapTerm, QuadPattern,
    Statement, StatementPattern, Term,
};
use core::{borrow::Borrow, marker::PhantomData};

pub type AnyQuad = AnyStatement;
pub type AnyTriple = AnyStatement;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AnyStatement;

impl StatementPattern for AnyStatement {
    type Term = AnyTerm;

    fn matches(&self, _: impl Term, _: impl Term, _: impl Term, _: Option<impl Term>) -> bool {
        true
    }
}

#[cfg(feature = "alloc")]
impl From<AnyStatement> for crate::CowTriplePattern<'_> {
    fn from(_: AnyStatement) -> Self {
        Self::empty()
    }
}

#[cfg(feature = "alloc")]
impl From<AnyStatement> for crate::CowQuadPattern<'_> {
    fn from(_: AnyStatement) -> Self {
        Self::empty()
    }
}

#[cfg(feature = "alloc")]
impl From<AnyStatement> for crate::HeapTriplePattern {
    fn from(_: AnyStatement) -> Self {
        Self::empty()
    }
}

#[cfg(feature = "alloc")]
impl From<AnyStatement> for crate::HeapQuadPattern {
    fn from(_: AnyStatement) -> Self {
        Self::empty()
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<crate::CowTriplePattern<'_>> for AnyStatement {
    type Error = ();

    fn try_from(input: crate::CowTriplePattern<'_>) -> Result<Self, Self::Error> {
        if input.is_empty() {
            Ok(Self::default())
        } else {
            Err(())
        }
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<crate::CowQuadPattern<'_>> for AnyStatement {
    type Error = ();

    fn try_from(input: crate::CowQuadPattern<'_>) -> Result<Self, Self::Error> {
        if input.is_empty() {
            Ok(Self::default())
        } else {
            Err(())
        }
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<crate::HeapTriplePattern> for AnyStatement {
    type Error = ();

    fn try_from(input: crate::HeapTriplePattern) -> Result<Self, Self::Error> {
        if input.is_empty() {
            Ok(Self::default())
        } else {
            Err(())
        }
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<crate::HeapQuadPattern> for AnyStatement {
    type Error = ();

    fn try_from(input: crate::HeapQuadPattern) -> Result<Self, Self::Error> {
        if input.is_empty() {
            Ok(Self::default())
        } else {
            Err(())
        }
    }
}

impl Borrow<QuadPattern<CowTerm<'static>>> for AnyStatement {
    fn borrow(&self) -> &QuadPattern<CowTerm<'static>> {
        &EMPTY_COW_QUAD_PATTERN
    }
}

impl Borrow<QuadPattern<HeapTerm>> for AnyStatement {
    fn borrow(&self) -> &QuadPattern<HeapTerm> {
        &EMPTY_HEAP_QUAD_PATTERN
    }
}
