// This is free and unencumbered software released into the public domain.

use crate::DecimalType;
use strum_macros::Display;

#[cfg(feature = "alloc")]
use ::alloc::format;

/// The XML Schema `xsd:decimal` datatypes.
///
/// See: https://www.w3.org/TR/xmlschema-2/#built-in-datatypes
#[derive(Clone, Debug, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
// #[cfg_attr(
//     feature = "borsh",
//     derive(borsh::BorshSerialize, borsh::BorshDeserialize)
// )]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DecimalValue {
    /// See: https://www.w3.org/TR/xmlschema-2/#decimal
    #[cfg(feature = "rust_decimal")]
    #[cfg_attr(feature = "serde", serde(with = "rust_decimal::serde::str"))]
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    Decimal(crate::Decimal),

    /// See: https://www.w3.org/TR/xmlschema-2/#integer
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    Integer(i128),

    /// See: https://www.w3.org/TR/xmlschema-2/#long
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    Long(i64),

    /// See: https://www.w3.org/TR/xmlschema-2/#int
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    Int(i32),

    /// See: https://www.w3.org/TR/xmlschema-2/#short
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    Short(i16),

    /// See: https://www.w3.org/TR/xmlschema-2/#byte
    #[cfg_attr(feature = "alloc", strum(to_string = "{0}"))]
    Byte(i8),
}

impl DecimalValue {
    pub fn r#type(&self) -> DecimalType {
        use DecimalValue::*;
        match self {
            #[cfg(feature = "rust_decimal")]
            Decimal(_) => DecimalType::Decimal,
            Integer(_) => DecimalType::Integer,
            Long(_) => DecimalType::Long,
            Int(_) => DecimalType::Int,
            Short(_) => DecimalType::Short,
            Byte(_) => DecimalType::Byte,
        }
    }
}
