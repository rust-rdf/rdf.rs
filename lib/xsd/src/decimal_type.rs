// This is free and unencumbered software released into the public domain.

use phf::phf_map;
use strum_macros::Display;

#[cfg(feature = "alloc")]
use ::alloc::{borrow::Cow, format, vec, vec::Vec};

/// The XML Schema `xsd:decimal` datatypes.
///
/// See: https://www.w3.org/TR/xmlschema-2/#built-in-datatypes
#[derive(Clone, Debug, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DecimalType {
    /// See: https://www.w3.org/TR/xmlschema-2/#decimal
    #[strum(to_string = "decimal")]
    Decimal,

    /// See: https://www.w3.org/TR/xmlschema-2/#integer
    #[strum(to_string = "integer")]
    Integer,

    /// See: https://www.w3.org/TR/xmlschema-2/#long
    #[strum(to_string = "long")]
    Long,

    /// See: https://www.w3.org/TR/xmlschema-2/#int
    #[strum(to_string = "int")]
    Int,

    /// See: https://www.w3.org/TR/xmlschema-2/#short
    #[strum(to_string = "short")]
    Short,

    /// See: https://www.w3.org/TR/xmlschema-2/#byte
    #[strum(to_string = "byte")]
    Byte,
}

impl DecimalType {
    pub fn name(&self) -> &str {
        use DecimalType::*;
        match self {
            Decimal => "decimal",
            Integer => "integer",
            Long => "long",
            Int => "int",
            Short => "short",
            Byte => "byte",
        }
    }

    pub fn curie(&self) -> &str {
        use DecimalType::*;
        match self {
            Decimal => "xsd:decimal",
            Integer => "xsd:integer",
            Long => "xsd:long",
            Int => "xsd:int",
            Short => "xsd:short",
            Byte => "xsd:byte",
        }
    }

    #[cfg(feature = "alloc")]
    pub fn iri_string(&self) -> Cow<'_, str> {
        Cow::Owned(format!("{}{}", crate::BASE_URI, self))
    }

    pub fn base_type(&self) -> Option<DecimalType> {
        use DecimalType::*;
        match self {
            Decimal => None,
            Integer => Some(Decimal),
            Long => Some(Integer),
            Int => Some(Long),
            Short => Some(Int),
            Byte => Some(Short),
        }
    }

    #[cfg(feature = "alloc")]
    pub fn base_types(&self) -> Vec<DecimalType> {
        use DecimalType::*;
        match self {
            Decimal => vec![],
            Integer => vec![Decimal],
            Long => vec![Integer, Decimal],
            Int => vec![Long, Integer, Decimal],
            Short => vec![Int, Long, Integer, Decimal],
            Byte => vec![Short, Int, Long, Integer, Decimal],
        }
    }
}

pub static DECIMAL_TYPES: phf::Map<&'static str, DecimalType> = phf_map! {
    "decimal" => DecimalType::Decimal,
    "integer" => DecimalType::Integer,
    "long" => DecimalType::Long,
    "int" => DecimalType::Int,
    "short" => DecimalType::Short,
    "byte" => DecimalType::Byte,
};
