// This is free and unencumbered software released into the public domain.

extern crate alloc;

use crate::TermKind;
use alloc::borrow::Cow;

/// An RDF term.
///
/// See: <https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-term>
pub trait Term {
    fn kind(&self) -> TermKind;

    fn is_variable(&self) -> bool {
        false // TODO
    }

    fn is_constant(&self) -> bool {
        !self.is_variable()
    }

    fn is_iri(&self) -> bool {
        self.kind() == TermKind::Iri
    }

    fn is_bnode(&self) -> bool {
        self.kind() == TermKind::BNode
    }

    fn is_literal(&self) -> bool {
        self.kind() == TermKind::Literal
    }

    #[cfg(feature = "alloc")]
    fn value_str(&self) -> Cow<'_, str>;
}

impl core::fmt::Debug for dyn Term {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Term").finish()
    }
}
