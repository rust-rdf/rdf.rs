// This is free and unencumbered software released into the public domain.

/// A value of the `xsd:string` datatype.
///
/// See: <https://www.w3.org/TR/xmlschema-2/#string>
#[cfg(feature = "alloc")]
pub use ::alloc::string::String;

/// A value of the `xsd:string` datatype.
///
/// See: <https://www.w3.org/TR/xmlschema-2/#string>
#[cfg(not(feature = "alloc"))]
pub type String = &'static str;
