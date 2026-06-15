// This is free and unencumbered software released into the public domain.

//! An N-Triples file reader for RDF.rs, a Rust framework for RDF
//! knowledge graphs.
//!
//! # Examples
//!
//! ```rust,no_run
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use tokio::fs::File;
//! let file = File::open("example.nt").await?;
//!
//! use rdf_reader_ntriples::NtriplesReader;
//! let reader = NtriplesReader::open(file).await?;
//!
//! use futures::StreamExt;
//! reader
//!     .into_stream()
//!     .for_each(|triple| async move {
//!         eprintln!("{:?}", triple);
//!     })
//!     .await;
//! # Ok(())
//! # }
//! ```

#![no_std]
#![deny(unsafe_code)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "oxrdf")]
mod oxrdf {
    mod error;
    pub use error::*;

    mod reader;
    pub use reader::*;

    mod triple;
    pub use triple::*;
}
#[cfg(feature = "oxrdf")]
pub use oxrdf::*;
