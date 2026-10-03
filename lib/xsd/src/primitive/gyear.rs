// This is free and unencumbered software released into the public domain.

/// A bounded, timezone-free year for `xsd:gYear`.
///
/// This alias stores any `i32` year (`-2147483648..=2147483647`), including zero
/// as in XSD 1.1. Negative years retain their signed numeric value without an
/// era adjustment. It has no timezone field and cannot represent arbitrarily
/// large XSD years. Available without allocation or date/time features.
///
/// The alias itself formats and compares as a Rust integer. Wrap it in
/// [`crate::PrimitiveValue::GYear`] for XSD formatting: at least four digits,
/// a minus sign for negative years, and no leading plus sign. Within that
/// variant, comparison follows the stored integer, not timezone-aware XSD
/// temporal comparison. Original lexical spelling is not retained.
///
/// [`crate::parse_g_year`] and [`crate::parse`] support timezone-free XSD lexical
/// forms, validating year spelling and range. Timezone suffixes return errors
/// rather than being discarded.
/// Explicit JSON conversion (`serde`) emits the formatted year as an untagged
/// string. Derived Serde serialization instead preserves the `GYear` variant
/// and integer field and supports structural round trips.
///
/// ```
/// let year: xsd::primitive::GYear = -1;
/// assert_eq!(year.to_string(), "-1");
/// assert_eq!(xsd::PrimitiveValue::GYear(year).to_string(), "-0001");
/// assert_eq!(xsd::PrimitiveValue::GYear(0).to_string(), "0000");
/// ```
///
/// See: <https://www.w3.org/TR/xmlschema-2/#gYear>
pub type GYear = i32;
