// This is free and unencumbered software released into the public domain.

use crate::{
    PrimitiveType,
    primitives::{Decimal, Double, Float},
};

#[cfg(feature = "jiff")]
use crate::primitives::{Date, DateTime, Duration, Time};

#[cfg(feature = "alloc")]
use strum_macros::Display;

#[cfg(feature = "alloc")]
use ::alloc::{borrow::Cow, format, string::String, vec::Vec};

/// Values based on built-in primitive datatypes.
///
/// See: https://www.w3.org/TR/xmlschema-2/#built-in-datatypes
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "alloc", derive(Display))]
// #[cfg_attr(
//     feature = "borsh",
//     derive(borsh::BorshSerialize, borsh::BorshDeserialize) // FIXME: SignedDuration
// )]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PrimitiveValue {
    /// See: https://www.w3.org/TR/xmlschema-2/#string
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    String(String),
    #[cfg(not(feature = "alloc"))]
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    String(&'static str),

    /// See: https://www.w3.org/TR/xmlschema-2/#boolean
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    Boolean(bool),

    /// See: https://www.w3.org/TR/xmlschema-2/#decimal
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    Decimal(Decimal),

    /// See: https://www.w3.org/TR/xmlschema-2/#float
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    Float(Float),

    /// See: https://www.w3.org/TR/xmlschema-2/#double
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    Double(Double),

    /// See: https://www.w3.org/TR/xmlschema-2/#duration
    #[cfg(feature = "jiff")]
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    Duration(Duration),

    /// See: https://www.w3.org/TR/xmlschema-2/#dateTime
    #[cfg(feature = "jiff")]
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    DateTime(DateTime),

    /// See: https://www.w3.org/TR/xmlschema-2/#time
    #[cfg(feature = "jiff")]
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    Time(Time),

    /// See: https://www.w3.org/TR/xmlschema-2/#date
    #[cfg(feature = "jiff")]
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    Date(Date),

    /// See: https://www.w3.org/TR/xmlschema-2/#gYearMonth
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}-{1}"))]
    GYearMonth(i32, u8),

    /// See: https://www.w3.org/TR/xmlschema-2/#gYear
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    GYear(i32),

    /// See: https://www.w3.org/TR/xmlschema-2/#gMonthDay
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}-{1}"))]
    GMonthDay(u8, u8),

    /// See: https://www.w3.org/TR/xmlschema-2/#gDay
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    GDay(u8),

    /// See: https://www.w3.org/TR/xmlschema-2/#gMonth
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    GMonth(u8),

    /// See: https://www.w3.org/TR/xmlschema-2/#hexBinary
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "alloc", strum(to_string = "HexBinary"))]
    HexBinary(Vec<u8>),

    /// See: https://www.w3.org/TR/xmlschema-2/#base64Binary
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "alloc", strum(to_string = "Base64Binary"))]
    Base64Binary(Vec<u8>),

    /// See: https://www.w3.org/TR/xmlschema-2/#anyURI
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    AnyUri(String),

    /// See: https://www.w3.org/TR/xmlschema-2/#QName
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}:{1}"))]
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
            #[cfg(feature = "alloc")]
            HexBinary(_) => PrimitiveType::HexBinary,
            #[cfg(feature = "alloc")]
            Base64Binary(_) => PrimitiveType::Base64Binary,
            #[cfg(feature = "alloc")]
            AnyUri(_) => PrimitiveType::AnyUri,
            #[cfg(feature = "alloc")]
            QName(_, _) => PrimitiveType::QName,
        }
    }
}

impl From<&'static str> for PrimitiveValue {
    fn from(input: &'static str) -> Self {
        Self::String(input.into())
    }
}

#[cfg(feature = "alloc")]
impl From<Cow<'_, str>> for PrimitiveValue {
    fn from(input: Cow<'_, str>) -> Self {
        Self::String(input.into())
    }
}

#[cfg(feature = "alloc")]
impl From<String> for PrimitiveValue {
    fn from(input: String) -> Self {
        Self::String(input)
    }
}

#[cfg(feature = "alloc")]
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
        Self::Decimal(input.into())
    }
}

#[cfg(feature = "rust_decimal")]
impl From<&rust_decimal::Decimal> for PrimitiveValue {
    fn from(input: &rust_decimal::Decimal) -> Self {
        Self::Decimal(input.clone().into())
    }
}

impl From<f32> for PrimitiveValue {
    fn from(input: f32) -> Self {
        Self::Float(input.into())
    }
}

impl From<f64> for PrimitiveValue {
    fn from(input: f64) -> Self {
        Self::Double(input.into())
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

#[cfg(feature = "alloc")]
impl From<Vec<u8>> for PrimitiveValue {
    fn from(input: Vec<u8>) -> Self {
        Self::Base64Binary(input)
    }
}

#[cfg(feature = "alloc")]
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
