// This is free and unencumbered software released into the public domain.

//! ```rust
//! # use rdf_writer::*;
//! ```

#![no_std]
#![deny(unsafe_code)]
#![allow(unused_imports)]

mod writer;
pub use writer::*;

mod writer_options;
pub use writer_options::*;

mod providers {
    #[cfg(feature = "oxrdf")]
    mod oxrdf;
    #[cfg(feature = "oxrdf")]
    pub use oxrdf::*;
    #[cfg(feature = "sophia")]
    mod sophia;
    #[cfg(feature = "sophia")]
    pub use sophia::*;
}
