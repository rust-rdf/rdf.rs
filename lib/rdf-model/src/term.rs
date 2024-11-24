// This is free and unencumbered software released into the public domain.

extern crate alloc;

use alloc::borrow::Cow;

/// An RDF term.
///
/// See: https://www.w3.org/TR/rdf12-concepts/#dfn-rdf-term
#[stability::unstable]
pub trait Term {
    fn as_str(&self) -> Cow<str>;
}

impl core::fmt::Debug for dyn Term {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Term").finish()
    }
}
