// This is free and unencumbered software released into the public domain.

//! An HDT file writer for RDF.rs, a Rust framework for RDF
//! knowledge graphs.
//!
//! # Examples
//!
//! ```rust,no_run
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use tokio::fs::File;
//! let file = File::create("output.hdt").await?;
//!
//! use rdf_writer_hdt::HdtWriter;
//! let writer = HdtWriter::from(file);
//!
//! writer.finish().await?;
//! # Ok(())
//! # }
//! ```
//!
//! See: <https://rdfhdt.org>

#![no_std]
#![deny(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod error;
pub use error::*;

mod writer;
pub use writer::*;

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
