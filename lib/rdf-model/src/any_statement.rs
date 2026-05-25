// This is free and unencumbered software released into the public domain.

use crate::{AnyTerm, StatementPattern, Term};
use core::marker::PhantomData;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AnyStatement<T: Term + Clone = AnyTerm>(PhantomData<T>);

impl<T: Term + Clone> AnyStatement<T> {
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T: Term + Clone> Default for AnyStatement<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Term + Clone> StatementPattern for AnyStatement<T> {
    type Term = T;
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
impl<T: Term + Clone> TryFrom<crate::CowTriplePattern<'_>> for AnyStatement<T> {
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
