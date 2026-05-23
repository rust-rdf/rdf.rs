// This is free and unencumbered software released into the public domain.

use crate::{DecimalType, DecimalValue, PrimitiveType, PrimitiveValue};
use core::str::FromStr;
use phf::phf_map;
use strum_macros::Display;

#[cfg(feature = "alloc")]
use ::alloc::{borrow::Cow, format, string::String, vec::Vec};

/// An XML Schema datatype.
///
/// See: https://www.w3.org/TR/xmlschema-2/#built-in-datatypes
#[derive(Clone, Debug, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    Primitive(PrimitiveType),

    Decimal(DecimalType),

    #[cfg(feature = "alloc")]
    Other(String),
    #[cfg(not(feature = "alloc"))]
    Other(&'static str),
}

impl Type {
    pub fn name(&self) -> &str {
        use Type::*;
        match self {
            Decimal(t) => t.name(),
            Primitive(t) => t.name(),
            Other(s) => s.as_ref(),
        }
    }

    pub fn curie(&self) -> &str {
        use Type::*;
        match self {
            Decimal(t) => t.curie(),
            Primitive(t) => t.curie(),
            Other(s) => s.as_ref(), // FIXME
        }
    }

    #[cfg(feature = "alloc")]
    pub fn iri_string(&self) -> Cow<'_, str> {
        Cow::Owned(format!("{}{}", crate::BASE_URI, self))
    }

    pub fn base_type(&self) -> Option<Type> {
        use Type::*;
        match self {
            Decimal(t) => t.base_type().map(Decimal),
            Primitive(_) => None,
            Other(_) => None,
        }
    }

    #[cfg(feature = "alloc")]
    pub fn base_types(&self) -> Vec<Type> {
        use Type::*;
        match self {
            Decimal(t) => t.base_types().into_iter().map(Decimal).collect(),
            Primitive(_) => Vec::new(),
            Other(_) => Vec::new(),
        }
    }
}

impl FromStr for Type {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        TYPES.get(input).cloned().ok_or(())
    }
}

#[cfg(feature = "alloc")]
impl From<&str> for Type {
    fn from(input: &str) -> Self {
        TYPES
            .get(input)
            .cloned()
            .unwrap_or_else(|| Type::Other(input.into()))
    }
}

#[cfg(not(feature = "alloc"))]
impl From<&'static str> for Type {
    fn from(input: &'static str) -> Self {
        TYPES
            .get(input)
            .cloned()
            .unwrap_or_else(|| Type::Other(input))
    }
}

impl From<DecimalType> for Type {
    fn from(input: DecimalType) -> Self {
        Self::Decimal(input)
    }
}

impl From<DecimalValue> for Type {
    fn from(input: DecimalValue) -> Self {
        Self::Decimal(input.r#type())
    }
}

impl From<PrimitiveType> for Type {
    fn from(input: PrimitiveType) -> Self {
        Self::Primitive(input)
    }
}

impl From<PrimitiveValue> for Type {
    fn from(input: PrimitiveValue) -> Self {
        Self::Primitive(input.r#type())
    }
}

pub static TYPES: phf::Map<&'static str, &Type> = phf_map! {
    "QName" => &QNAME,
    "anyURI" => &ANY_URI,
    "base64Binary" => &BASE64_BINARY,
    "boolean" => &BOOLEAN,
    "byte" => &BYTE,
    "date" => &DATE,
    "dateTime" => &DATE_TIME,
    "decimal" => &DECIMAL,
    "double" => &DOUBLE,
    "duration" => &DURATION,
    "float" => &FLOAT,
    "gDay" => &G_DAY,
    "gMonth" => &G_MONTH,
    "gMonthDay" => &G_MONTH_DAY,
    "gYear" => &G_YEAR,
    "gYearMonth" => &G_YEAR_MONTH,
    "hexBinary" => &HEX_BINARY,
    "int" => &INT,
    "integer" => &INTEGER,
    "long" => &LONG,
    "short" => &SHORT,
    "string" => &STRING,
    "time" => &TIME,
};

pub const QNAME: Type = Type::Primitive(PrimitiveType::QName);
pub const ANY_URI: Type = Type::Primitive(PrimitiveType::AnyUri);
pub const BASE64_BINARY: Type = Type::Primitive(PrimitiveType::Base64Binary);
pub const BOOLEAN: Type = Type::Primitive(PrimitiveType::Boolean);
pub const BYTE: Type = Type::Decimal(DecimalType::Byte);
pub const DATE: Type = Type::Primitive(PrimitiveType::Date);
pub const DATE_TIME: Type = Type::Primitive(PrimitiveType::DateTime);
pub const DECIMAL: Type = Type::Decimal(DecimalType::Decimal);
pub const DOUBLE: Type = Type::Primitive(PrimitiveType::Double);
pub const DURATION: Type = Type::Primitive(PrimitiveType::Duration);
pub const FLOAT: Type = Type::Primitive(PrimitiveType::Float);
pub const G_DAY: Type = Type::Primitive(PrimitiveType::GDay);
pub const G_MONTH: Type = Type::Primitive(PrimitiveType::GMonth);
pub const G_MONTH_DAY: Type = Type::Primitive(PrimitiveType::GMonthDay);
pub const G_YEAR: Type = Type::Primitive(PrimitiveType::GYear);
pub const G_YEAR_MONTH: Type = Type::Primitive(PrimitiveType::GYearMonth);
pub const HEX_BINARY: Type = Type::Primitive(PrimitiveType::HexBinary);
pub const INT: Type = Type::Decimal(DecimalType::Int);
pub const INTEGER: Type = Type::Decimal(DecimalType::Integer);
pub const LONG: Type = Type::Decimal(DecimalType::Long);
pub const SHORT: Type = Type::Decimal(DecimalType::Short);
pub const STRING: Type = Type::Primitive(PrimitiveType::String);
pub const TIME: Type = Type::Primitive(PrimitiveType::Time);
