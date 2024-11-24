// This is free and unencumbered software released into the public domain.

/// An RDF node.
#[stability::unstable]
pub trait Node {}

impl core::fmt::Debug for dyn Node {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Node").finish()
    }
}
