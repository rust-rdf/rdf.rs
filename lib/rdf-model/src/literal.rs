// This is free and unencumbered software released into the public domain.

/// An RDF literal.
///
/// See: https://www.w3.org/TR/rdf12-concepts/#dfn-literal
#[stability::unstable]
pub trait Literal {}

impl core::fmt::Debug for dyn Literal {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Literal").finish()
    }
}
