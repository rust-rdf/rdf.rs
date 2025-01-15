// This is free and unencumbered software released into the public domain.

//! ```rust
//! # use rdf_borsh::*;
//! let reader = BorshReader::new(Box::new(std::io::stdin()));
//! let writer = BorshWriter::new(Box::new(std::io::stdout()));
//! ```

#![no_std]
#![deny(unsafe_code)]

mod borsh_dataset;
pub use borsh_dataset::*;

mod borsh_quad;
pub use borsh_quad::*;

mod borsh_reader;
pub use borsh_reader::*;

mod borsh_statement;
pub use borsh_statement::*;

mod borsh_term;
pub use borsh_term::*;

mod borsh_term_id;
pub use borsh_term_id::*;

mod borsh_triple;
pub use borsh_triple::*;

mod borsh_writer;
pub use borsh_writer::*;

mod parse;
pub use parse::parse_dataset;
pub(crate) use parse::*;
