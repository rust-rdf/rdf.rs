// This is free and unencumbered software released into the public domain.

extern crate alloc;

use rdf_format::Format;
use rdf_model::Source;

pub trait Reader: Source {
    fn format(&self) -> Format;
}

impl core::fmt::Debug for dyn Reader {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Reader")
            .field("format", &self.format())
            .finish()
    }
}
