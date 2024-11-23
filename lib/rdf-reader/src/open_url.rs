// This is free and unencumbered software released into the public domain.

extern crate std;

use crate::Reader;
use std::{boxed::Box, io::Result, string::String};

#[stability::unstable]
pub fn open_url(_url: &String) -> Result<Box<dyn Reader>> {
    todo!() // TODO
}
