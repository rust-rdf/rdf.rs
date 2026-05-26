// This is free and unencumbered software released into the public domain.

use super::{GDay, GMonth};

/// Rust type for representing values of the `xsd:gMonthDay` datatype.
pub type GMonthDay = (GMonth, GDay);
