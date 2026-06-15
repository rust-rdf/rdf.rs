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
//! let reader = TurtleReader::open(file).await?;
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

mod error;
pub use error::*;

mod reader;
pub use reader::*;

mod triple;
pub use triple::*;
