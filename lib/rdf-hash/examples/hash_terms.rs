// This is free and unencumbered software released into the public domain.

use rdf_hash::TermHash;
use rdf_model::{SAMPLE_LITERAL, SAMPLE_RESOURCE};

pub fn main() -> Result<(), Box<dyn core::error::Error>> {
    let hash1: TermHash = SAMPLE_RESOURCE.into();
    eprintln!("{}", hash1);

    let hash2: TermHash = SAMPLE_LITERAL.into();
    eprintln!("{}", hash2);

    Ok(())
}
