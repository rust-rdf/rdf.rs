// This is free and unencumbered software released into the public domain.

extern crate std;

use crate::{OpenOptions, Reader};
use std::{boxed::Box, io::Result, string::String};

#[stability::unstable]
pub fn open_url(_input_url: &String, _options: Option<OpenOptions>) -> Result<Box<dyn Reader>> {
    todo!() // TODO
}
