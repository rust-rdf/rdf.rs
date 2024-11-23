// This is free and unencumbered software released into the public domain.

extern crate std;

use crate::{providers, Reader, ReaderOptions};
use rdf_format::Format;
use std::{
    boxed::Box,
    fs::File,
    io::{BufReader, Read, Result},
    path::Path,
};

#[stability::unstable]
pub fn open_path(
    input_path: impl AsRef<Path>,
    options: Option<ReaderOptions>,
) -> Result<Box<dyn Reader>> {
    let options = options.unwrap_or_default();
    let input_format = options.format.unwrap_or_else(|| Format::Turtle); // TODO: from file extension
    let input_file = File::open(&input_path)?;
    let input_reader = BufReader::new(input_file);

    for_reader(
        input_reader,
        ReaderOptions {
            format: Some(input_format),
            ..options
        },
    )
}

#[stability::unstable]
pub fn for_reader<R: Read + 'static>(reader: R, options: ReaderOptions) -> Result<Box<dyn Reader>> {
    let input_format = options.format.expect("format must be specified");
    Ok(match input_format {
        #[cfg(feature = "oxrdf")]
        Format::Notation3
        | Format::NQuads
        | Format::NTriples
        | Format::RdfXml
        | Format::TriG
        | Format::Turtle => Box::new(providers::OxrdfReader::new(reader, options)),

        _ => unimplemented!(),
    })
}
