// This is free and unencumbered software released into the public domain.

extern crate alloc;

use rdf_format::Format;

pub trait Writer {
    fn format(&self) -> Format;
}

impl core::fmt::Debug for dyn Writer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Writer")
            .field("format", &self.format())
            .finish()
    }
}
