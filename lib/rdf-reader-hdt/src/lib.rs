// This is free and unencumbered software released into the public domain.

//! An HDT file reader for RDF.rs, a Rust framework for RDF
//! knowledge graphs.
//!
//! # Examples
//!
//! ```rust,no_run
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use tokio::fs::File;
//! let file = File::open("example.hdt").await?;
//!
//! use rdf_reader_hdt::HdtReader;
//! let reader = HdtReader::try_from(file).await?;
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

mod reader;
pub use reader::*;

mod triple;
pub use triple::*;

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
