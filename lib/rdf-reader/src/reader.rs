// This is free and unencumbered software released into the public domain.

extern crate alloc;

use alloc::boxed::Box;
use rdf_model::{Source, Statement};

pub trait Reader: Source {}

impl Iterator for dyn Reader {
    type Item = Box<dyn Statement>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!() // TODO
    }
}
