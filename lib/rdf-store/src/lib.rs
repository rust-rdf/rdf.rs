// This is free and unencumbered software released into the public domain.

//! RDF.rs is a Rust framework for working with RDF knowledge graphs.
//!
//! ```rust
//! use rdf_store::*;
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
