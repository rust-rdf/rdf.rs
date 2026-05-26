// This is free and unencumbered software released into the public domain.

//! RDF.rs is a Rust framework for working with RDF knowledge graphs.
//!
//! ```rust
//! # use rdf_store_sqlite::*;
//! ```

#![no_std]
#![deny(unsafe_code)]
#![allow(unused_imports)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub use rdf_store::*;

mod error;
pub use error::*;

mod schema;
pub use schema::*;

mod store;
pub use store::*;

mod transaction;
pub use transaction::*;
