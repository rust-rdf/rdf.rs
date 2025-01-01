// This is free and unencumbered software released into the public domain.

extern crate alloc;

use core::result::Result;
use rdf_model::Statement;

pub use rdf_format::Format;

pub trait Writer {
    type Error;

    fn format(&self) -> Format;

    fn write_statement(&mut self, statement: &dyn Statement) -> Result<(), Self::Error>;
}

impl core::fmt::Debug for dyn Writer<Error = core::convert::Infallible> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Writer")
            .field("format", &self.format())
            .finish()
    }
}
