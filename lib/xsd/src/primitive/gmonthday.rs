// This is free and unencumbered software released into the public domain.

use super::{GDay, GMonth};

/// A value of the `xsd:gMonthDay` datatype.
///
/// See: <https://www.w3.org/TR/xmlschema-2/#gMonthDay>
pub type GMonthDay = (GMonth, GDay);
