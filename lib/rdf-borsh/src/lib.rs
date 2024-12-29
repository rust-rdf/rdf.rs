// This is free and unencumbered software released into the public domain.

//! ```rust
//! # use rdf_borsh::*;
//! ```

#![no_std]
#![deny(unsafe_code)]

mod reader;
pub use reader::*;

mod writer;
pub use writer::*;
