// This is free and unencumbered software released into the public domain.

use phf::phf_map;
use strum_macros::{AsRefStr, Display, EnumString};

#[cfg(feature = "alloc")]
use ::alloc::{borrow::Cow, format, string::String};

/// The XSD primitive datatype hierarchy.
///
/// See: <https://www.w3.org/TR/xmlschema-2/#built-in-datatypes>
#[derive(
    AsRefStr, Clone, Debug, Default, Display, EnumString, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PrimitiveType {
    /// See: <https://www.w3.org/TR/xmlschema-2/#string>
    #[default]
    #[strum(to_string = "string")]
    String,

    /// See: <https://www.w3.org/TR/xmlschema-2/#boolean>
    #[strum(to_string = "boolean")]
    Boolean,

    /// See: <https://www.w3.org/TR/xmlschema-2/#decimal>
    #[strum(to_string = "decimal")]
    Decimal,

    /// See: <https://www.w3.org/TR/xmlschema-2/#float>
    #[strum(to_string = "float")]
    Float,

    /// See: <https://www.w3.org/TR/xmlschema-2/#double>
    #[strum(to_string = "double")]
    Double,

    /// See: <https://www.w3.org/TR/xmlschema-2/#duration>
    #[strum(to_string = "duration")]
    Duration,

    /// See: <https://www.w3.org/TR/xmlschema-2/#dateTime>
    #[strum(to_string = "dateTime")]
    DateTime,

    /// See: <https://www.w3.org/TR/xmlschema-2/#time>
    #[strum(to_string = "time")]
    Time,

    /// See: <https://www.w3.org/TR/xmlschema-2/#date>
    #[strum(to_string = "date")]
    Date,

    /// See: <https://www.w3.org/TR/xmlschema-2/#gYearMonth>
    #[strum(to_string = "gYearMonth")]
    GYearMonth,

    /// See: <https://www.w3.org/TR/xmlschema-2/#gYear>
    #[strum(to_string = "gYear")]
    GYear,

    /// See: <https://www.w3.org/TR/xmlschema-2/#gMonthDay>
    #[strum(to_string = "gMonthDay")]
    GMonthDay,

    /// See: <https://www.w3.org/TR/xmlschema-2/#gDay>
    #[strum(to_string = "gDay")]
    GDay,

    /// See: <https://www.w3.org/TR/xmlschema-2/#gMonth>
    #[strum(to_string = "gMonth")]
    GMonth,

    /// See: <https://www.w3.org/TR/xmlschema-2/#hexBinary>
    #[strum(to_string = "hexBinary")]
    HexBinary,

    /// See: <https://www.w3.org/TR/xmlschema-2/#base64Binary>
    #[strum(to_string = "base64Binary")]
    Base64Binary,

    /// See: <https://www.w3.org/TR/xmlschema-2/#anyURI>
    #[strum(to_string = "anyURI")]
    AnyUri,

    /// See: <https://www.w3.org/TR/xmlschema-2/#QName>
    #[strum(to_string = "QName")]
    QName,

    #[strum(to_string = "{0}")]
    #[strum(default)]
    #[cfg(feature = "alloc")]
    Other(String),
    #[cfg(not(feature = "alloc"))]
    Other(&'static str),
}

impl PrimitiveType {
    pub fn name(&self) -> &str {
        use PrimitiveType::*;
        match self {
            String => "string",
            Boolean => "boolean",
            Decimal => "decimal",
            Float => "float",
            Double => "double",
            Duration => "duration",
            DateTime => "dateTime",
            Time => "time",
            Date => "date",
            GYearMonth => "gYearMonth",
            GYear => "gYear",
            GMonthDay => "gMonthDay",
            GDay => "gDay",
            GMonth => "gMonth",
            HexBinary => "hexBinary",
            Base64Binary => "base64Binary",
            AnyUri => "anyURI",
            QName => "QName",
            Other(s) => s.as_ref(),
        }
    }

    pub fn curie(&self) -> &str {
        use PrimitiveType::*;
        match self {
            String => "xsd:string",
            Boolean => "xsd:boolean",
            Decimal => "xsd:decimal",
            Float => "xsd:float",
            Double => "xsd:double",
            Duration => "xsd:duration",
            DateTime => "xsd:dateTime",
            Time => "xsd:time",
            Date => "xsd:date",
            GYearMonth => "xsd:gYearMonth",
            GYear => "xsd:gYear",
            GMonthDay => "xsd:gMonthDay",
            GDay => "xsd:gDay",
            GMonth => "xsd:gMonth",
            HexBinary => "xsd:hexBinary",
            Base64Binary => "xsd:base64Binary",
            AnyUri => "xsd:anyURI",
            QName => "xsd:QName",
            Other(s) => s.as_ref(),
        }
    }

    #[cfg(feature = "alloc")]
    pub fn iri_string(&self) -> Cow<'_, str> {
        Cow::Owned(format!("{}{}", crate::BASE_URI, self))
    }
}

/// A PHF map from XSD primitive typenames to datatypes.
#[allow(unused)]
static PRIMITIVE_TYPES: phf::Map<&'static str, PrimitiveType> = phf_map! {
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
