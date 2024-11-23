// This is free and unencumbered software released into the public domain.

extern crate std;

use crate::{OpenOptions, Reader};
use std::{boxed::Box, io::Result, path::Path};

#[stability::unstable]
pub fn open_path(
    _input_path: impl AsRef<Path>,
    _options: Option<OpenOptions>,
) -> Result<Box<dyn Reader>> {
    todo!() // TODO
}
