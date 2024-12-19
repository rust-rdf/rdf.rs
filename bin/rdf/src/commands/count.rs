// This is free and unencumbered software released into the public domain.

use crate::{exit::ExitCode, StandardOptions, Utf8PathBuf};

pub fn count(mut input_paths: Vec<Utf8PathBuf>, flags: &StandardOptions) -> Result<(), ExitCode> {
    let mut count = 0usize;

    input_paths.dedup();

    for input_path in input_paths {
        let reader = rdf_reader::open_path(&input_path, None)?;
        if flags.debug {
            eprintln!("{:?}", reader);
        }

        count += reader.count();
    }

    println!("{}", count);
    Ok(())
}
