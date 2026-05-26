// This is free and unencumbered software released into the public domain.

use crate::{Term, TermKind};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AnyTerm;

impl Term for AnyTerm {
    fn kind(&self) -> TermKind {
        todo!() // FIXME
    }

    #[cfg(feature = "alloc")]
    fn value_str(&self) -> alloc::borrow::Cow<'_, str> {
        alloc::borrow::Cow::default()
    }
}
