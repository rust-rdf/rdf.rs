// This is free and unencumbered software released into the public domain.

use crate::traits::Enumerable;

/// An RDF document.
///
/// See: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-document
pub trait Document: Enumerable {}
