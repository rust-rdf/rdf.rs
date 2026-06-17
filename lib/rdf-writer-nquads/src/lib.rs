// This is free and unencumbered software released into the public domain.

//! An N-Quads file writer for RDF.rs, a Rust framework for RDF
//! knowledge graphs.
//!
//! # Examples
//!
//! ```rust,no_run
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use tokio::fs::File;
//! let file = File::create("output.nq").await?;
//!
//! use rdf_writer_nquads::NquadsWriter;
//! let writer = NquadsWriter::from(file);
//!
//! writer.finish().await?;
//! # Ok(())
//! # }
//! ```

#![no_std]
#![deny(unsafe_code)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "oxrdf")]
mod oxrdf {
    mod error;
    pub use error::*;

    mod writer;
    pub use writer::*;
}
#[cfg(feature = "oxrdf")]
pub use oxrdf::*;
