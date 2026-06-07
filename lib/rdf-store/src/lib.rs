// This is free and unencumbered software released into the public domain.

//! An in-memory storage adapter for RDF.rs, a Rust framework for RDF
//! knowledge graphs.
//!
//! # Examples
//!
//! ```rust
//! use rdf_store::{HeapStore, HeapTransaction};
//! ```

#![no_std]
#![deny(unsafe_code)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
mod heap {
    mod store;
    pub use store::*;
    mod transaction;
    pub use transaction::*;
}
#[cfg(feature = "alloc")]
pub use heap::*;

mod store;
pub use store::*;

mod store_options;
pub use store_options::*;

mod read_transaction;
pub use read_transaction::*;

mod write_transaction;
pub use write_transaction::*;
