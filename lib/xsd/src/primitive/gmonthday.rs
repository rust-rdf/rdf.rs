// This is free and unencumbered software released into the public domain.

use super::{GDay, GMonth};

/// A value of the `xsd:gMonthDay` datatype.
///
/// This raw `(month, day)` pair does not validate calendar fields or store a
/// timezone. Use [`crate::PrimitiveValue::g_month_day`] for checked construction,
/// including support for February 29 without assuming a particular year.
///
/// See: <https://www.w3.org/TR/xmlschema-2/#gMonthDay>
pub type GMonthDay = (GMonth, GDay);
