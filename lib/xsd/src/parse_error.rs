// This is free and unencumbered software released into the public domain.

pub type ParseBooleanError = ParseError;
pub type ParseDoubleError = ParseError;
pub type ParseFloatError = ParseError;
pub type ParseIntegerError = core::num::ParseIntError;

#[cfg(feature = "jiff")]
pub type ParseDateTimeError = jiff::Error;

#[cfg(feature = "jiff")]
pub type ParseDurationError = jiff::Error;

#[derive(Debug)]
pub struct ParseError;

impl core::fmt::Display for ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "parse error")
    }
}

impl core::error::Error for ParseError {}

#[cfg(feature = "rust_decimal")]
#[derive(Debug)]
pub struct ParseDecimalError(rust_decimal::Error);

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
