// This is free and unencumbered software released into the public domain.

use crate::traits::{Enumerable, MaybeDurable, MaybeIndexed, MaybeMutable};

/// An RDF source.
///
/// See: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-source
pub trait Source: Enumerable + MaybeDurable + MaybeIndexed + MaybeMutable {}

impl core::fmt::Debug for dyn Source {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Source").finish()
    }
}
