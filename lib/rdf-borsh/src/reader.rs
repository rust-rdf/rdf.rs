// This is free and unencumbered software released into the public domain.

extern crate alloc;

use alloc::boxed::Box;
use core::error::Error;

pub struct Reader {}

impl rdf_reader::Reader for Reader {
    fn format(&self) -> rdf_reader::Format {
        todo!() // TODO
    }
}

impl rdf_model::Source for Reader {}
impl rdf_model::Enumerable for Reader {}
impl rdf_model::MaybeDurable for Reader {}
impl rdf_model::MaybeIndexed for Reader {}
impl rdf_model::MaybeMutable for Reader {}

impl rdf_model::Countable for Reader {
    fn count(&self) -> usize {
        0 // TODO
    }
}

impl Iterator for Reader {
    type Item = Result<Box<dyn rdf_model::Statement>, Box<dyn Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        None // TODO
    }
}
