// This is free and unencumbered software released into the public domain.

//! An in-memory storage adapter for RDF.rs, a Rust framework for RDF
//! knowledge graphs.
//!
//! # Features
//!
//! `alloc` enables the transaction traits and their allocation-based defaults
//! without `std`. The heap backend and its Tokio synchronization dependency
//! require `std`. `oxrdf` also implies `std`. With all features disabled, only
//! allocation-independent options remain. Disable defaults before selecting a
//! tier; `--no-default-features --features alloc` does not expose `HeapStore`.
//!
//! # Examples
//!
//! ```rust
//! # #[cfg(feature = "std")]
//! use rdf_store::{HeapStore, HeapTransaction};
//! ```

#![no_std]
#![deny(unsafe_code)]
//#![allow(unused_imports)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
mod heap {
    mod error;
    pub use error::*;
    mod store;
    pub use store::*;
    mod transaction;
    pub use transaction::*;
}
#[cfg(feature = "std")]
pub use heap::*;

#[cfg(feature = "alloc")]
mod store;
#[cfg(feature = "alloc")]
pub use store::*;

mod store_options;
pub use store_options::*;

#[cfg(feature = "alloc")]
mod read_transaction;
#[cfg(feature = "alloc")]
pub use read_transaction::*;

#[cfg(feature = "alloc")]
mod write_transaction;
#[cfg(feature = "alloc")]
pub use write_transaction::*;

#[doc = include_str!("../README.md")]
#[cfg(all(doctest, feature = "std"))]
pub struct ReadmeDoctests;
