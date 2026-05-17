// This is free and unencumbered software released into the public domain.

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

mod store;
pub use store::*;

mod transaction;
pub use transaction::*;
