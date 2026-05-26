// This is free and unencumbered software released into the public domain.

use super::{GMonth, GYear};

/// Rust type for representing values of the `xsd:gYearMonth` datatype.
pub type GYearMonth = (GYear, GMonth);
