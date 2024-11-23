// This is free and unencumbered software released into the public domain.

extern crate std;

use crate::{Reader, ReaderOptions};
use oxrdfio::{RdfParser, ReaderQuadParser};
use rdf_model::{MaybeDurable, MaybeIndexed, MaybeMutable, Source};
use std::io::Read;

pub struct OxrdfReader<R: Read> {
    parser: ReaderQuadParser<R>,
}

impl<R: Read> OxrdfReader<R> {
    pub fn new(reader: R, options: ReaderOptions) -> Self {
        let format = options
            .format
            .expect("format must provided")
            .try_into()
            .expect("format must be compatible");
        let parser = RdfParser::from_format(format).for_reader(reader);
        Self { parser }
    }
}

impl<R: Read> Reader for OxrdfReader<R> {}

impl<R: Read> Source for OxrdfReader<R> {}

impl<R: Read> MaybeDurable for OxrdfReader<R> {}

impl<R: Read> MaybeIndexed for OxrdfReader<R> {}

impl<R: Read> MaybeMutable for OxrdfReader<R> {}
