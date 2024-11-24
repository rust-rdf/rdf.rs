// This is free and unencumbered software released into the public domain.

use crate::traits::Enumerable;

/// An RDF dataset.
///
/// See: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-dataset
pub trait Dataset: Enumerable {}

impl core::fmt::Debug for dyn Dataset {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dataset").finish()
    }
}
