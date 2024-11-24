// This is free and unencumbered software released into the public domain.

use crate::traits::Enumerable;

/// An RDF graph.
///
/// See: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-graph
pub trait Graph: Enumerable {}

impl core::fmt::Debug for dyn Graph {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Graph").finish()
    }
}
