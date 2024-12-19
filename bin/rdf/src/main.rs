// This is free and unencumbered software released into the public domain.

#![deny(unsafe_code)]
#![allow(unused)]

mod commands;
mod exit;

use crate::exit::ExitCode;
use clientele::{
    crates::{
        camino::Utf8PathBuf,
        clap::{Parser, Subcommand},
    },
    StandardOptions,
};

/// RDF.rs Command-Line Interface (CLI)
#[derive(Debug, Parser)]
#[command(name = "RDF.rs")]
#[command(arg_required_else_help = true)]
struct Options {
    #[clap(flatten)]
    flags: StandardOptions,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Query the registry of RDF formats
    Format {
        #[command(subcommand)]
        command: FormatCommand,
    },

    Parse {
        /// The input files
        files: Vec<Utf8PathBuf>,
    },
}

#[derive(Debug, Subcommand)]
enum FormatCommand {
    /// List known RDF formats
    List {},
}

pub fn main() -> Result<(), ExitCode> {
    // Load environment variables from `.env`:
    clientele::dotenv().ok();

    // Expand wildcards and @argfiles:
    let args = clientele::args_os()?;

    // Parse command-line options:
    let options = Options::parse_from(args);

    if options.flags.version {
        println!("RDF.rs {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    if options.flags.license {
        println!("This is free and unencumbered software released into the public domain.");
        return Ok(());
    }

    // Configure verbose/debug output:
    if options.flags.verbose > 0 || options.flags.debug {
        // TODO: configure tracing
    }

    use commands::*;
    match options.command.unwrap() {
        Command::Format { command } => match command {
            FormatCommand::List {} => format::list::list(&options.flags),
        },
        Command::Parse { files } => parse::parse(files, &options.flags),
    }
}
