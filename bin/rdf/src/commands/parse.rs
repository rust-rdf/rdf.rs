// This is free and unencumbered software released into the public domain.

use crate::{StandardOptions, Utf8PathBuf, exit::ExitCode};

pub fn parse(mut input_paths: Vec<Utf8PathBuf>, flags: &StandardOptions) -> Result<(), ExitCode> {
    // input_paths.dedup();

    // for input_path in input_paths {
    //     let reader = rdf_reader::open_path(&input_path, None)?;
    //     if flags.debug {
    //         eprintln!("{:?}", reader);
    //     }

    //     for statement in reader {
    //         if flags.debug {
    //             eprintln!("{:?}", statement);
    //         } else if flags.verbose > 0 {
    //             println!("{:?}", statement); // TODO
    //         }
    //     }
    // }

    Ok(())
}
