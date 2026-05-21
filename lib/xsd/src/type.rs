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

impl From<&str> for Type {
    fn from(input: &str) -> Self {
        TYPES
            .get(input)
            .cloned()
            .unwrap_or_else(|| Type::Other(input.into()))
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

pub static TYPES: phf::Map<&'static str, Type> = phf_map! {
    "QName" => Type::Primitive(PrimitiveType::QName),
    "anyURI" => Type::Primitive(PrimitiveType::AnyUri),
    "base64Binary" => Type::Primitive(PrimitiveType::Base64Binary),
    "boolean" => Type::Primitive(PrimitiveType::Boolean),
    "byte" => Type::Decimal(DecimalType::Byte),
    "date" => Type::Primitive(PrimitiveType::Date),
    "dateTime" => Type::Primitive(PrimitiveType::DateTime),
    "decimal" => Type::Decimal(DecimalType::Decimal),
    "double" => Type::Primitive(PrimitiveType::Double),
    "duration" => Type::Primitive(PrimitiveType::Duration),
    "float" => Type::Primitive(PrimitiveType::Float),
    "gDay" => Type::Primitive(PrimitiveType::GDay),
    "gMonth" => Type::Primitive(PrimitiveType::GMonth),
    "gMonthDay" => Type::Primitive(PrimitiveType::GMonthDay),
    "gYear" => Type::Primitive(PrimitiveType::GYear),
    "gYearMonth" => Type::Primitive(PrimitiveType::GYearMonth),
    "hexBinary" => Type::Primitive(PrimitiveType::HexBinary),
    "int" => Type::Decimal(DecimalType::Int),
    "integer" => Type::Decimal(DecimalType::Integer),
    "long" => Type::Decimal(DecimalType::Long),
    "short" => Type::Decimal(DecimalType::Short),
    "string" => Type::Primitive(PrimitiveType::String),
    "time" => Type::Primitive(PrimitiveType::Time),
};
