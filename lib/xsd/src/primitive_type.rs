// This is free and unencumbered software released into the public domain.

extern crate alloc;

use alloc::{format, string::String};
use phf::phf_map;
use strum_macros::{AsRefStr, Display, EnumString};

/// The XML Schema built-in primitive datatypes.
///
/// See: https://www.w3.org/TR/xmlschema-2/#built-in-datatypes
#[derive(
    AsRefStr, Clone, Debug, Default, Display, EnumString, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum PrimitiveType {
    /// See: https://www.w3.org/TR/xmlschema-2/#string
    #[default]
    #[strum(to_string = "string")]
    String,

    /// See: https://www.w3.org/TR/xmlschema-2/#boolean
    #[strum(to_string = "boolean")]
    Boolean,

    /// See: https://www.w3.org/TR/xmlschema-2/#decimal
    #[strum(to_string = "decimal")]
    Decimal,

    /// See: https://www.w3.org/TR/xmlschema-2/#float
    #[strum(to_string = "float")]
    Float,

    /// See: https://www.w3.org/TR/xmlschema-2/#double
    #[strum(to_string = "double")]
    Double,

    /// See: https://www.w3.org/TR/xmlschema-2/#duration
    #[strum(to_string = "duration")]
    Duration,

    /// See: https://www.w3.org/TR/xmlschema-2/#dateTime
    #[strum(to_string = "dateTime")]
    DateTime,

    /// See: https://www.w3.org/TR/xmlschema-2/#time
    #[strum(to_string = "time")]
    Time,

    /// See: https://www.w3.org/TR/xmlschema-2/#date
    #[strum(to_string = "date")]
    Date,

    /// See: https://www.w3.org/TR/xmlschema-2/#gYearMonth
    #[strum(to_string = "gYearMonth")]
    GYearMonth,

    /// See: https://www.w3.org/TR/xmlschema-2/#gYear
    #[strum(to_string = "gYear")]
    GYear,

    /// See: https://www.w3.org/TR/xmlschema-2/#gMonthDay
    #[strum(to_string = "gMonthDay")]
    GMonthDay,

    /// See: https://www.w3.org/TR/xmlschema-2/#gDay
    #[strum(to_string = "gDay")]
    GDay,

    /// See: https://www.w3.org/TR/xmlschema-2/#gMonth
    #[strum(to_string = "gMonth")]
    GMonth,

    /// See: https://www.w3.org/TR/xmlschema-2/#hexBinary
    #[strum(to_string = "hexBinary")]
    HexBinary,

    /// See: https://www.w3.org/TR/xmlschema-2/#base64Binary
    #[strum(to_string = "base64Binary")]
    Base64Binary,

    /// See: https://www.w3.org/TR/xmlschema-2/#anyURI
    #[strum(to_string = "anyURI")]
    AnyUri,

    /// See: https://www.w3.org/TR/xmlschema-2/#QName
    #[strum(to_string = "QName")]
    QName,

    #[strum(to_string = "{0}")]
    #[strum(default)]
    Other(String),
}

impl PrimitiveType {}

pub static TYPES: phf::Map<&'static str, PrimitiveType> = phf_map! {
    "string" => PrimitiveType::String,
    "boolean" => PrimitiveType::Boolean,
    "decimal" => PrimitiveType::Decimal,
    "float" => PrimitiveType::Float,
    "double" => PrimitiveType::Double,
    "duration" => PrimitiveType::Duration,
    "dateTime" => PrimitiveType::DateTime,
    "time" => PrimitiveType::Time,
    "date" => PrimitiveType::Date,
    "gYearMonth" => PrimitiveType::GYearMonth,
    "gYear" => PrimitiveType::GYear,
    "gMonthDay" => PrimitiveType::GMonthDay,
    "gDay" => PrimitiveType::GDay,
    "gMonth" => PrimitiveType::GMonth,
    "hexBinary" => PrimitiveType::HexBinary,
    "base64Binary" => PrimitiveType::Base64Binary,
    "anyURI" => PrimitiveType::AnyUri,
    "QName" => PrimitiveType::QName,
};
