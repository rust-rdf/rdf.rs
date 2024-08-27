// This is free and unencumbered software released into the public domain.

use crate::traits::Enumerable;

/// An RDF graph.
///
/// See: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-graph
pub trait Graph: Enumerable {}
