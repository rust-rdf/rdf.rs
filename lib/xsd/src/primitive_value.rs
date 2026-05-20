// This is free and unencumbered software released into the public domain.

extern crate alloc;

use crate::PrimitiveType;
use alloc::{string::String, vec::Vec};
use float_ord::FloatOrd;

/// Values based on built-in primitive datatypes.
///
/// See: https://www.w3.org/TR/xmlschema-2/#built-in-datatypes
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PrimitiveValue {
    /// See: https://www.w3.org/TR/xmlschema-2/#string
    String(String),

    /// See: https://www.w3.org/TR/xmlschema-2/#boolean
    Boolean(bool),

    /// See: https://www.w3.org/TR/xmlschema-2/#decimal
    #[cfg(feature = "rust_decimal")]
    Decimal(crate::Decimal),
    #[cfg(not(feature = "rust_decimal"))]
    Decimal(String),

    /// See: https://www.w3.org/TR/xmlschema-2/#float
    Float(FloatOrd<f32>),

    /// See: https://www.w3.org/TR/xmlschema-2/#double
    Double(FloatOrd<f64>),

    /// See: https://www.w3.org/TR/xmlschema-2/#duration
    #[cfg(feature = "jiff")]
    Duration(crate::Duration),

    /// See: https://www.w3.org/TR/xmlschema-2/#dateTime
    #[cfg(feature = "jiff")]
    DateTime(crate::DateTime),

    /// See: https://www.w3.org/TR/xmlschema-2/#time
    #[cfg(feature = "jiff")]
    Time(crate::Time),

    /// See: https://www.w3.org/TR/xmlschema-2/#date
    #[cfg(feature = "jiff")]
    Date(crate::Date),

    /// See: https://www.w3.org/TR/xmlschema-2/#gYearMonth
    GYearMonth(i32, u8),

    /// See: https://www.w3.org/TR/xmlschema-2/#gYear
    GYear(i32),

    /// See: https://www.w3.org/TR/xmlschema-2/#gMonthDay
    GMonthDay(u8, u8),

    /// See: https://www.w3.org/TR/xmlschema-2/#gDay
    GDay(u8),

    /// See: https://www.w3.org/TR/xmlschema-2/#gMonth
    GMonth(u8),

    /// See: https://www.w3.org/TR/xmlschema-2/#hexBinary
    HexBinary(Vec<u8>),

    /// See: https://www.w3.org/TR/xmlschema-2/#base64Binary
    Base64Binary(Vec<u8>),

    /// See: https://www.w3.org/TR/xmlschema-2/#anyURI
    AnyUri(String),

    /// See: https://www.w3.org/TR/xmlschema-2/#QName
    QName(String, String),
}

impl PrimitiveValue {
    pub fn r#type(&self) -> PrimitiveType {
        use PrimitiveValue::*;
        match self {
            String(_) => PrimitiveType::String,
            Boolean(_) => PrimitiveType::Boolean,
            Decimal(_) => PrimitiveType::Decimal,
            Float(_) => PrimitiveType::Float,
            Double(_) => PrimitiveType::Double,
            #[cfg(feature = "jiff")]
            Duration(_) => PrimitiveType::Duration,
            #[cfg(feature = "jiff")]
            DateTime(_) => PrimitiveType::DateTime,
            #[cfg(feature = "jiff")]
            Time(_) => PrimitiveType::Time,
            #[cfg(feature = "jiff")]
            Date(_) => PrimitiveType::Date,
            GYearMonth(_, _) => PrimitiveType::GYearMonth,
            GYear(_) => PrimitiveType::GYear,
            GMonthDay(_, _) => PrimitiveType::GMonthDay,
            GDay(_) => PrimitiveType::GDay,
            GMonth(_) => PrimitiveType::GMonth,
            HexBinary(_) => PrimitiveType::HexBinary,
            Base64Binary(_) => PrimitiveType::Base64Binary,
            AnyUri(_) => PrimitiveType::AnyUri,
            QName(_, _) => PrimitiveType::QName,
        }
    }
}

impl From<&str> for PrimitiveValue {
    fn from(input: &str) -> Self {
        Self::String(input.into())
    }
}

impl From<String> for PrimitiveValue {
    fn from(input: String) -> Self {
        Self::String(input)
    }
}

impl From<&String> for PrimitiveValue {
    fn from(input: &String) -> Self {
        Self::String(input.clone())
    }
}

impl From<bool> for PrimitiveValue {
    fn from(input: bool) -> Self {
        Self::Boolean(input)
    }
}

#[cfg(feature = "rust_decimal")]
impl From<rust_decimal::Decimal> for PrimitiveValue {
    fn from(input: rust_decimal::Decimal) -> Self {
        Self::Decimal(input)
    }
}

#[cfg(feature = "rust_decimal")]
impl From<&rust_decimal::Decimal> for PrimitiveValue {
    fn from(input: &rust_decimal::Decimal) -> Self {
        Self::Decimal(input.clone())
    }
}

impl From<f32> for PrimitiveValue {
    fn from(input: f32) -> Self {
        Self::Float(FloatOrd(input))
    }
}

impl From<f64> for PrimitiveValue {
    fn from(input: f64) -> Self {
        Self::Double(FloatOrd(input))
    }
}

#[cfg(feature = "jiff")]
impl From<jiff::SignedDuration> for PrimitiveValue {
    fn from(input: jiff::SignedDuration) -> Self {
        Self::Duration(input)
    }
}

#[cfg(feature = "jiff")]
impl From<&jiff::SignedDuration> for PrimitiveValue {
    fn from(input: &jiff::SignedDuration) -> Self {
        Self::Duration(input.clone())
    }
}

#[cfg(feature = "jiff")]
impl From<jiff::civil::DateTime> for PrimitiveValue {
    fn from(input: jiff::civil::DateTime) -> Self {
        Self::DateTime(input)
    }
}

#[cfg(feature = "jiff")]
impl From<&jiff::civil::DateTime> for PrimitiveValue {
    fn from(input: &jiff::civil::DateTime) -> Self {
        Self::DateTime(input.clone())
    }
}

#[cfg(feature = "jiff")]
impl From<jiff::civil::Time> for PrimitiveValue {
    fn from(input: jiff::civil::Time) -> Self {
        Self::Time(input)
    }
}

#[cfg(feature = "jiff")]
impl From<&jiff::civil::Time> for PrimitiveValue {
    fn from(input: &jiff::civil::Time) -> Self {
        Self::Time(input.clone())
    }
}

#[cfg(feature = "jiff")]
impl From<jiff::civil::Date> for PrimitiveValue {
    fn from(input: jiff::civil::Date) -> Self {
        Self::Date(input)
    }
}

#[cfg(feature = "jiff")]
impl From<&jiff::civil::Date> for PrimitiveValue {
    fn from(input: &jiff::civil::Date) -> Self {
        Self::Date(input.clone())
    }
}

impl From<Vec<u8>> for PrimitiveValue {
    fn from(input: Vec<u8>) -> Self {
        Self::Base64Binary(input)
    }
}

impl From<&Vec<u8>> for PrimitiveValue {
    fn from(input: &Vec<u8>) -> Self {
        Self::Base64Binary(input.clone())
    }
}

#[cfg(feature = "jiff")]
impl TryFrom<jiff::Span> for PrimitiveValue {
    type Error = jiff::Error;

    fn try_from(input: jiff::Span) -> Result<Self, Self::Error> {
        Ok(Self::Duration(jiff::SignedDuration::try_from(input)?))
    }
}
