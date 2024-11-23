// This is free and unencumbered software released into the public domain.

extern crate std;

use crate::{Reader, ReaderOptions};
use rdf_format::Format;
use std::{boxed::Box, io::Result, string::String};

#[stability::unstable]
pub fn open_url(_input_url: &String, options: Option<ReaderOptions>) -> Result<Box<dyn Reader>> {
    let options = options.unwrap_or_default();
    let _input_format = options.format.unwrap_or_else(|| Format::Turtle);

    todo!() // TODO
}
