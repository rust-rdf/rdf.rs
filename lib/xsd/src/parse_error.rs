// This is free and unencumbered software released into the public domain.

/// An error encountered when parsing an `xsd:boolean` literal.
pub type ParseBooleanError = ParseError;

/// An error encountered when parsing an `xsd:double` literal.
pub type ParseDoubleError = core::num::ParseFloatError;

/// An error encountered when parsing an `xsd:float` literal.
pub type ParseFloatError = core::num::ParseFloatError;

/// An error encountered when parsing an `xsd:integer` literal.
pub type ParseIntegerError = core::num::ParseIntError;

/// An error encountered when parsing an `xsd:dateTime` literal.
#[cfg(feature = "jiff")]
pub type ParseDateTimeError = jiff::Error;

/// An error encountered when parsing an `xsd:duration` literal.
#[cfg(feature = "jiff")]
pub type ParseDurationError = jiff::Error;

/// An error encountered when parsing an XSD literal.
#[derive(Debug)]
pub struct ParseError;

impl core::fmt::Display for ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "parse error")
    }
}

impl core::error::Error for ParseError {}

/// An error encountered when parsing an `xsd:decimal` literal.
#[cfg(feature = "rust_decimal")]
#[derive(Debug)]
pub struct ParseDecimalError(rust_decimal::Error);

/// An error encountered when parsing an `xsd:decimal` literal.
#[cfg(not(feature = "rust_decimal"))]
#[derive(Debug)]
pub struct ParseDecimalError;

impl core::fmt::Display for ParseDecimalError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        #[cfg(feature = "rust_decimal")]
        return self.0.fmt(f);
        #[cfg(not(feature = "rust_decimal"))]
        return write!(f, "parse decimal error");
    }
}

impl core::error::Error for ParseDecimalError {}

#[cfg(feature = "rust_decimal")]
impl From<rust_decimal::Error> for ParseDecimalError {
    fn from(input: rust_decimal::Error) -> Self {
        Self(input)
    }
}
