// This is free and unencumbered software released into the public domain.

extern crate std;

use crate::Reader;
use std::{boxed::Box, io::Result, path::PathBuf};

#[stability::unstable]
pub fn open_path(_path: &PathBuf) -> Result<Box<dyn Reader>> {
    todo!() // TODO
}
