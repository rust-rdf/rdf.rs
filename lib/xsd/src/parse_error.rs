// This is free and unencumbered software released into the public domain.

pub type ParseBooleanError = ParseError;
pub type ParseDateTimeError = ParseError;
pub type ParseDecimalError = ParseError;
pub type ParseDoubleError = ParseError;
pub type ParseDurationError = ParseError;
pub type ParseFloatError = ParseError;

#[derive(Debug)]
pub struct ParseError;

impl core::fmt::Display for ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "parse error")
    }
}

impl core::error::Error for ParseError {}
