// This is free and unencumbered software released into the public domain.

/// A value of the `xsd:gDay` datatype.
///
/// This raw alias does not validate the XSD range `1..=31` or store a timezone.
/// Use [`crate::PrimitiveValue::g_day`] for checked value construction.
///
/// See: <https://www.w3.org/TR/xmlschema-2/#gDay>
pub type GDay = u8;
