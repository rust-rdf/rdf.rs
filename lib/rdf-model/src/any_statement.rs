// This is free and unencumbered software released into the public domain.

use crate::{AnyTerm, QuadPattern, Statement, StatementPattern, Term};
#[cfg(feature = "alloc")]
use crate::{CowTerm, EMPTY_COW_QUAD_PATTERN, EMPTY_HEAP_QUAD_PATTERN, HeapTerm};
use core::{borrow::Borrow, marker::PhantomData};

pub type AnyQuad = AnyStatement;
pub type AnyTriple = AnyStatement;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AnyStatement;

impl StatementPattern for AnyStatement {
    type Term = AnyTerm;
}

impl AnyStatement {
    /// Matches any terms in any graph, without requiring cross-type equality.
    pub fn matches(&self, _: impl Term, _: impl Term, _: impl Term, _: Option<impl Term>) -> bool {
        true
    }

    /// Matches any statement without inspecting or cloning its terms.
    pub fn matches_statement(&self, _: &(impl Statement + ?Sized)) -> bool {
        true
    }

    /// Produces an unconstrained pattern without constructing a graph marker.
    pub fn to_quad_pattern(&self) -> QuadPattern<AnyTerm> {
        QuadPattern::empty()
    }
}

impl<T: Term> From<AnyStatement> for crate::TriplePattern<T> {
    fn from(_: AnyStatement) -> Self {
        Self::empty()
    }
}

impl<T: Term> From<AnyStatement> for QuadPattern<T> {
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

#[cfg(feature = "alloc")]
impl Borrow<QuadPattern<CowTerm<'static>>> for AnyStatement {
    fn borrow(&self) -> &QuadPattern<CowTerm<'static>> {
        &EMPTY_COW_QUAD_PATTERN
    }
}

#[cfg(feature = "alloc")]
impl Borrow<QuadPattern<HeapTerm>> for AnyStatement {
    fn borrow(&self) -> &QuadPattern<HeapTerm> {
        &EMPTY_HEAP_QUAD_PATTERN
    }
}
