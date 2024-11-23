// This is free and unencumbered software released into the public domain.

//! ```rust
//! # use rdf_reader::*;
//! ```

#![no_std]
#![deny(unsafe_code)]

#[cfg(feature = "std")]
mod open_path;
#[cfg(feature = "std")]
pub use open_path::*;

#[cfg(feature = "std")]
mod open_url;
#[cfg(feature = "std")]
pub use open_url::*;

mod reader;
pub use reader::*;

mod reader_options;
pub use reader_options::*;

mod providers {
    #[cfg(feature = "oxrdf")]
    mod oxrdf;
    #[cfg(feature = "oxrdf")]
    pub use oxrdf::*;
}
