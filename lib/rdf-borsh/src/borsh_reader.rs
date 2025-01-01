// This is free and unencumbered software released into the public domain.

extern crate alloc;

use alloc::boxed::Box;
use borsh::io::Read;
use core::error::Error;
use rdf_model::{
    Countable, Enumerable, MaybeDurable, MaybeIndexed, MaybeMutable, Source, Statement,
};
use rdf_reader::{Format, Reader};

pub struct BorshReader {
    #[allow(unused)]
    source: Box<dyn Read>,
}

impl BorshReader {
    pub fn new(source: Box<dyn Read>) -> Self {
        Self { source }
    }
}

impl Reader for BorshReader {
    fn format(&self) -> Format {
        todo!() // TODO
    }
}

impl Source for BorshReader {}
impl Enumerable for BorshReader {}
impl MaybeDurable for BorshReader {}
impl MaybeIndexed for BorshReader {}
impl MaybeMutable for BorshReader {}

impl Countable for BorshReader {
    fn count(&self) -> usize {
        0 // TODO
    }
}

impl Iterator for BorshReader {
    type Item = Result<Box<dyn Statement>, Box<dyn Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        None // TODO
    }
}
