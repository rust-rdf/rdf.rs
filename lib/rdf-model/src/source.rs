// This is free and unencumbered software released into the public domain.

use crate::traits::{Durable, Mutable};

/// An RDF source.
///
/// See: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-source
pub trait Source: Durable + Mutable {}
