// This is free and unencumbered software released into the public domain.

/// An RDF vocabulary.
///
/// See: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-vocabulary
pub trait Vocabulary {}

impl core::fmt::Debug for dyn Vocabulary {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vocabulary").finish()
    }
}
