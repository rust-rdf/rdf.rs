// This is free and unencumbered software released into the public domain.

//! A Jelly file writer for RDF.rs, a Rust framework for RDF
//! knowledge graphs.
//!
//! # Examples
//!
//! ```rust,no_run
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use tokio::fs::File;
//! let file = File::create("output.jelly").await?;
//!
//! use rdf_writer_jelly::JellyWriter;
//! let writer = JellyWriter::from(file);
//!
//! writer.finish().await?;
//! # Ok(())
//! # }
//! ```
//!
//! See: <https://jelly-rdf.github.io>

#![no_std]
#![deny(unsafe_code)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod error;
pub use error::*;

mod writer;
pub use writer::*;
