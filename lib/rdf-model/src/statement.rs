// This is free and unencumbered software released into the public domain.

/// An RDF statement.
///
/// See: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-statement
pub trait Statement {}

impl core::fmt::Debug for dyn Statement {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Statement").finish()
    }
}
