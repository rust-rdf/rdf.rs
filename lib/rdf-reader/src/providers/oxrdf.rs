// This is free and unencumbered software released into the public domain.

extern crate std;

use crate::{Reader, ReaderOptions};
use core::error::Error;
use oxrdfio::{RdfParser, ReaderQuadParser};
use rdf_format::Format;
use rdf_model::{
    Countable, Enumerable, MaybeDurable, MaybeIndexed, MaybeMutable, Source, Statement,
};
use std::{boxed::Box, io::Read};

pub struct OxrdfReader<R: Read> {
    format: Format,
    parser: ReaderQuadParser<R>,
}

impl<R: Read> Iterator for OxrdfReader<R> {
    type Item = Result<Box<dyn Statement>, Box<dyn Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.parser.next() {
            Some(Err(error)) => Some(Err(error.into())),
            Some(Ok(_quad)) => None, // TODO
            None => None,
        }
    }
}

impl<R: Read> OxrdfReader<R> {
    pub fn new(reader: R, options: ReaderOptions) -> Self {
        let format = options.format.expect("format must provided");
        let parser = RdfParser::from_format(format.try_into().expect("format must be compatible"))
            .for_reader(reader);
        Self { format, parser }
    }
}

impl<R: Read> Reader for OxrdfReader<R> {
    fn format(&self) -> Format {
        self.format
    }
}

impl<R: Read> Source for OxrdfReader<R> {}

impl<R: Read> Enumerable for OxrdfReader<R> {}

impl<R: Read> Countable for OxrdfReader<R> {
    fn count(&self) -> usize {
        0 // TODO
    }
}

impl<R: Read> MaybeDurable for OxrdfReader<R> {}

impl<R: Read> MaybeIndexed for OxrdfReader<R> {}

impl<R: Read> MaybeMutable for OxrdfReader<R> {}
