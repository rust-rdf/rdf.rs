// This is free and unencumbered software released into the public domain.

use crate::traits::Enumerable;

/// An RDF document.
///
/// See: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-document
pub trait Document: Enumerable {}

impl core::fmt::Debug for dyn Document {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Document").finish()
    }
}
