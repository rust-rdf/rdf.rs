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

pub type ParseDecimalError = valuand::DecimalError;
