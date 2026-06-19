// This is free and unencumbered software released into the public domain.

use crate::{DecimalType, DecimalValue, PrimitiveType, PrimitiveValue};
use core::str::FromStr;
use phf::phf_map;
use strum_macros::Display;

#[cfg(feature = "alloc")]
use ::alloc::{borrow::Cow, format, string::String, vec::Vec};

/// An XSD datatype.
///
/// Currently supports the primitive datatypes and the derived `xsd:decimal`
/// datatypes.
///
/// See: <https://www.w3.org/TR/xmlschema-2/#built-in-datatypes>
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
    type Err = (); // TODO

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match TYPES.get(input) {
            Some(&t) => Ok(t.clone()),
            None => Err(()),
        }
    }
}

impl From<&Type> for Type {
    fn from(input: &Type) -> Self {
        input.clone()
    }
}

#[cfg(all(feature = "oxrdf", feature = "alloc"))]
impl From<oxrdf::NamedNode> for Type {
    fn from(input: oxrdf::NamedNode) -> Self {
        let input_iri = input.as_str();
        if let Some(input_name) = input_iri.strip_prefix(crate::BASE_URI) {
            return Type::from(input_name);
        }
        Type::Other(input_iri.into()) // TODO
    }
}

#[cfg(feature = "alloc")]
impl From<&str> for Type {
    fn from(input: &str) -> Self {
        match TYPES.get(input) {
            Some(&t) => t.clone(),
            None => Type::Other(input.into()),
        }
    }
}

#[cfg(not(feature = "alloc"))]
impl From<&'static str> for Type {
    fn from(input: &'static str) -> Self {
        match TYPES.get(input) {
            Some(&t) => t.clone(),
            None => Type::Other(input.into()),
        }
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

/// A PHF map from XSD typenames to datatypes.
pub static TYPES: phf::Map<&'static str, &'static Type> = phf_map! {
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

/// The `xsd:QName` primitive datatype.
pub const QNAME: Type = Type::Primitive(PrimitiveType::QName);

/// The `xsd:anyURI` primitive datatype.
pub const ANY_URI: Type = Type::Primitive(PrimitiveType::AnyUri);

/// The `xsd:base64Binary` primitive datatype.
pub const BASE64_BINARY: Type = Type::Primitive(PrimitiveType::Base64Binary);

/// The `xsd:boolean` primitive datatype.
pub const BOOLEAN: Type = Type::Primitive(PrimitiveType::Boolean);

/// The `xsd:byte` derived datatype.
pub const BYTE: Type = Type::Decimal(DecimalType::Byte);

/// The `xsd:date` primitive datatype.
pub const DATE: Type = Type::Primitive(PrimitiveType::Date);

/// The `xsd:dateTime` primitive datatype.
pub const DATE_TIME: Type = Type::Primitive(PrimitiveType::DateTime);

/// The `xsd:decimal` primitive datatype.
pub const DECIMAL: Type = Type::Decimal(DecimalType::Decimal);

/// The `xsd:double` primitive datatype.
pub const DOUBLE: Type = Type::Primitive(PrimitiveType::Double);

/// The `xsd:duration` primitive datatype.
pub const DURATION: Type = Type::Primitive(PrimitiveType::Duration);

/// The `xsd:float` primitive datatype.
pub const FLOAT: Type = Type::Primitive(PrimitiveType::Float);

/// The `xsd:gDay` primitive datatype.
pub const G_DAY: Type = Type::Primitive(PrimitiveType::GDay);

/// The `xsd:gMonth` primitive datatype.
pub const G_MONTH: Type = Type::Primitive(PrimitiveType::GMonth);

/// The `xsd:gMonthDay` primitive datatype.
pub const G_MONTH_DAY: Type = Type::Primitive(PrimitiveType::GMonthDay);

/// The `xsd:gYear` primitive datatype.
pub const G_YEAR: Type = Type::Primitive(PrimitiveType::GYear);

/// The `xsd:gYearMonth` primitive datatype.
pub const G_YEAR_MONTH: Type = Type::Primitive(PrimitiveType::GYearMonth);

/// The `xsd:hexBinary` primitive datatype.
pub const HEX_BINARY: Type = Type::Primitive(PrimitiveType::HexBinary);

/// The `xsd:int` derived datatype.
pub const INT: Type = Type::Decimal(DecimalType::Int);

/// The `xsd:integer` derived datatype.
pub const INTEGER: Type = Type::Decimal(DecimalType::Integer);

/// The `xsd:long` derived datatype.
pub const LONG: Type = Type::Decimal(DecimalType::Long);

/// The `xsd:short` derived datatype.
pub const SHORT: Type = Type::Decimal(DecimalType::Short);

/// The `xsd:string` primitive datatype.
pub const STRING: Type = Type::Primitive(PrimitiveType::String);

/// The `xsd:time` primitive datatype.
pub const TIME: Type = Type::Primitive(PrimitiveType::Time);
