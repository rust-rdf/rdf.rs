// This is free and unencumbered software released into the public domain.

use super::{GMonth, GYear};

/// A value of the `xsd:gYearMonth` datatype.
///
/// This raw `(year, month)` pair stores no timezone and does not validate the
/// month. Use [`crate::PrimitiveValue::g_year_month`] for checked construction.
/// The year range is `i32::MIN..=i32::MAX`, including zero as in XSD 1.1.
///
/// See: <https://www.w3.org/TR/xmlschema-2/#gYearMonth>
pub type GYearMonth = (GYear, GMonth);
