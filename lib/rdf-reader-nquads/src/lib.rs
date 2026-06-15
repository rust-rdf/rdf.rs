// This is free and unencumbered software released into the public domain.

//! An N-Quads file reader for RDF.rs, a Rust framework for RDF
//! knowledge graphs.
//!
//! # Examples
//!
//! ```rust,no_run
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use tokio::fs::File;
//! let file = File::open("example.nq").await?;
//!
//! use rdf_reader_nquads::NquadsReader;
//! let reader = NquadsReader::open(file).await?;
//!
//! use futures::StreamExt;
//! reader
//!     .into_stream()
//!     .for_each(|quad| async move {
//!         eprintln!("{:?}", quad);
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

    mod quad;
    pub use quad::*;

    mod reader;
    pub use reader::*;
}
#[cfg(feature = "oxrdf")]
pub use oxrdf::*;
