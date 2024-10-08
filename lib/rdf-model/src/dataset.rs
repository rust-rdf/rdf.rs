// This is free and unencumbered software released into the public domain.

use crate::traits::Enumerable;

/// An RDF dataset.
///
/// See: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-dataset
pub trait Dataset: Enumerable {}
