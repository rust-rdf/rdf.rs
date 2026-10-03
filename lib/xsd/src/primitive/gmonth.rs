// This is free and unencumbered software released into the public domain.

/// A value of the `xsd:gMonth` datatype.
///
/// This raw alias does not validate the XSD range `1..=12` or store a timezone.
/// Use [`crate::PrimitiveValue::g_month`] for checked value construction.
///
/// See: <https://www.w3.org/TR/xmlschema-2/#gMonth>
pub type GMonth = u8;
