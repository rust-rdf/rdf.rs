// This is free and unencumbered software released into the public domain.

use super::{GMonth, GYear};

/// A value of the `xsd:gYearMonth` datatype.
///
/// See: <https://www.w3.org/TR/xmlschema-2/#gYearMonth>
pub type GYearMonth = (GYear, GMonth);
