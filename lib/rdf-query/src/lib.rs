// This is free and unencumbered software released into the public domain.

//! ```rust
//! # use rdf_query::*;
//! ```

#![no_std]
#![deny(unsafe_code)]

pub mod graph_pattern;
pub mod matcher;
pub mod pattern;
pub mod query;
pub mod solution;
pub mod solutions;
pub mod variable;

mod traits {
    pub mod queryable;
}
