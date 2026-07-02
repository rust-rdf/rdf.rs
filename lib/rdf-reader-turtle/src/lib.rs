// This is free and unencumbered software released into the public domain.

//! A Turtle file reader for RDF.rs, a Rust framework for RDF
//! knowledge graphs.
//!
//! # Examples
//!
//! ```rust,no_run
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use tokio::fs::File;
//! let file = File::open("example.ttl").await?;
//!
//! use rdf_reader_turtle::TurtleReader;
//! let reader = TurtleReader::from(file);
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
//!
//! See: <https://www.w3.org/TR/rdf12-turtle/>

#![no_std]
#![deny(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

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

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
