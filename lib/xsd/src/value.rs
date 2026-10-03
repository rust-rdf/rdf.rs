// This is free and unencumbered software released into the public domain.

use crate::{
    DecimalValue, PrimitiveValue, Type,
    primitive::{Boolean, Decimal, Double, Float},
};
use strum_macros::Display;

#[cfg(feature = "jiff")]
use crate::primitive::{Date, DateTime, Duration, Time};

#[cfg(feature = "alloc")]
use ::alloc::{borrow::Cow, string::String};

/// An XSD value.
///
/// Currently supports the primitive datatypes and the derived `xsd:decimal`
/// datatypes.
///
/// [`Display`](core::fmt::Display) delegates to the contained value's lexical
/// formatting. It does not preserve the original spelling of a parsed literal.
///
/// # Signed integer construction
///
/// `From<i8>`, `From<i16>`, `From<i32>`, and `From<i64>` construct the matching
/// [`DecimalValue`] variants (`Byte`, `Short`, `Int`, and `Long`). `From<i128>`
/// and `From<isize>` construct `Integer`. Each preserves the full Rust input
/// range without allocation or decimal conversion, including `i128::MIN`.
/// The datatype agrees with parsing the corresponding XSD integer subtype.
///
/// This replaces the former primitive-decimal construction route: newly
/// constructed integers have different enum variants, structural equality,
/// and derived Serde output. Existing variants and their serialization tags
/// are retained, so previously serialized primitive decimals still decode as
/// primitive decimals. Callers matching values should handle `Value::Decimal`.
///
/// ```
/// assert_eq!(xsd::Value::from(42_i32), xsd::parse("42", xsd::INT).unwrap());
/// assert_eq!(xsd::Value::from(i128::MAX).to_string(), i128::MAX.to_string());
/// ```
///
/// See: <https://www.w3.org/TR/xmlschema-2/#built-in-dataValues>
#[derive(Clone, Debug, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
// #[cfg_attr(
//     feature = "borsh",
//     derive(borsh::BorshSerialize, borsh::BorshDeserialize)
// )]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Value {
    #[strum(transparent)]
    Primitive(PrimitiveValue),

    #[strum(transparent)]
    Decimal(DecimalValue),
}

impl Value {
    #[cfg(feature = "alloc")]
    pub fn string(value: impl Into<String>) -> Self {
        Self::Primitive(value.into().into())
    }

    pub fn boolean(value: impl Into<Boolean>) -> Self {
        Self::Primitive(value.into().into())
    }

    pub fn decimal(value: impl Into<Decimal>) -> Self {
        Self::Decimal(value.into().into())
    }

    pub fn float(value: impl Into<Float>) -> Self {
        Self::Primitive(value.into().into())
    }

    pub fn double(value: impl Into<Double>) -> Self {
        Self::Primitive(value.into().into())
    }

    #[cfg(feature = "jiff")]
    pub fn duration(value: impl Into<Duration>) -> Self {
        Self::Primitive(value.into().into())
    }

    #[cfg(feature = "jiff")]
    pub fn datetime(value: impl Into<DateTime>) -> Self {
        Self::Primitive(value.into().into())
    }

    #[cfg(feature = "jiff")]
    pub fn time(value: impl Into<Time>) -> Self {
        Self::Primitive(value.into().into())
    }

    #[cfg(feature = "jiff")]
    pub fn date(value: impl Into<Date>) -> Self {
        Self::Primitive(value.into().into())
    }

    pub fn r#type(&self) -> Type {
        use Value::*;
        match self {
            Primitive(v) => Type::Primitive(v.r#type()),
            Decimal(v) => Type::Decimal(v.r#type()),
        }
    }

    pub fn as_decimal(&self) -> &DecimalValue {
        self.to_decimal().expect("value must be a decimal")
    }

    pub fn as_primitive(&self) -> &PrimitiveValue {
        self.to_primitive().expect("value must be a primitive")
    }

    pub fn to_decimal(&self) -> Option<&DecimalValue> {
        match self {
            Self::Decimal(val) => Some(&val),
            _ => None,
        }
    }

    pub fn to_primitive(&self) -> Option<&PrimitiveValue> {
        match self {
            Self::Primitive(val) => Some(&val),
            _ => None,
        }
    }

    /// Clones and converts using [`Self::into_json`]. Requires `serde`.
    /// Always returns `Some`; values without a numeric JSON encoding use strings.
    #[cfg(feature = "serde")]
    pub fn to_json(&self) -> Option<serde_json::Value> {
        Some(self.clone().into_json())
    }

    /// Converts to untagged JSON, requiring `serde`.
    ///
    /// Delegates to [`DecimalValue::into_json`] or [`PrimitiveValue::into_json`]:
    /// decimals and integers outside `i64` use strings, as do non-finite floats
    /// (`INF`, `-INF`, `NaN`). Other numeric values use JSON numbers. Retain the
    /// XSD datatype separately for decoding; this is not derived Serde encoding.
    #[cfg(feature = "serde")]
    pub fn into_json(self) -> serde_json::Value {
        match self {
            Self::Primitive(val) => val.into_json(),
            Self::Decimal(val) => val.into_json(),
        }
    }

    #[cfg(feature = "bson")]
    pub fn to_bson(&self) -> Option<bson::Bson> {
        Some(self.clone().into_bson())
    }

    #[cfg(feature = "bson")]
    pub fn into_bson(self) -> bson::Bson {
        match self {
            Self::Primitive(val) => val.into_bson(),
            Self::Decimal(val) => val.into_bson(),
        }
    }
}

impl<T> From<&T> for Value
where
    T: Clone + Into<Self>,
{
    fn from(t: &T) -> Self {
        t.clone().into()
    }
}

impl From<DecimalValue> for Value {
    fn from(input: DecimalValue) -> Self {
        Self::Decimal(input)
    }
}

impl From<PrimitiveValue> for Value {
    fn from(input: PrimitiveValue) -> Self {
        Self::Primitive(input)
    }
}

impl From<&'static str> for Value {
    fn from(input: &'static str) -> Self {
        Self::Primitive(input.into())
    }
}

#[cfg(feature = "alloc")]
impl From<Cow<'_, str>> for Value {
    fn from(input: Cow<'_, str>) -> Self {
        Self::Primitive(input.into())
    }
}

#[cfg(feature = "alloc")]
impl From<String> for Value {
    fn from(input: String) -> Self {
        Self::Primitive(input.into())
    }
}

impl From<bool> for Value {
    fn from(input: bool) -> Self {
        Self::Primitive(input.into())
    }
}

impl From<Boolean> for Value {
    fn from(input: Boolean) -> Self {
        Self::Primitive(input.into())
    }
}

impl From<i8> for Value {
    /// Constructs an `xsd:byte` without changing the number.
    fn from(input: i8) -> Self {
        Self::Decimal(input.into())
    }
}

impl From<i16> for Value {
    /// Constructs an `xsd:short` without changing the number.
    fn from(input: i16) -> Self {
        Self::Decimal(input.into())
    }
}

impl From<i32> for Value {
    /// Constructs an `xsd:int` without changing the number.
    fn from(input: i32) -> Self {
        Self::Decimal(input.into())
    }
}

impl From<i64> for Value {
    /// Constructs an `xsd:long` without changing the number.
    fn from(input: i64) -> Self {
        Self::Decimal(input.into())
    }
}

impl From<i128> for Value {
    /// Constructs an `xsd:integer`, preserving the full signed 128-bit range.
    fn from(input: i128) -> Self {
        Self::Decimal(input.into())
    }
}

impl From<isize> for Value {
    /// Constructs an `xsd:integer`, preserving the full pointer-sized range.
    fn from(input: isize) -> Self {
        Self::Decimal(input.into())
    }
}

impl From<Decimal> for Value {
    fn from(input: Decimal) -> Self {
        Self::Decimal(input.into())
    }
}

impl From<f32> for Value {
    fn from(input: f32) -> Self {
        Self::Primitive(input.into())
    }
}

impl From<Float> for Value {
    fn from(input: Float) -> Self {
        Self::Primitive(input.into())
    }
}

impl From<f64> for Value {
    fn from(input: f64) -> Self {
        Self::Primitive(input.into())
    }
}

impl From<Double> for Value {
    fn from(input: Double) -> Self {
        Self::Primitive(input.into())
    }
}

#[cfg(feature = "jiff")]
impl From<Duration> for Value {
    fn from(input: Duration) -> Self {
        Self::Primitive(input.into())
    }
}

#[cfg(feature = "jiff")]
impl From<DateTime> for Value {
    fn from(input: DateTime) -> Self {
        Self::Primitive(input.into())
    }
}

#[cfg(feature = "jiff")]
impl From<Time> for Value {
    fn from(input: Time) -> Self {
        Self::Primitive(input.into())
    }
}

#[cfg(feature = "jiff")]
impl From<Date> for Value {
    fn from(input: Date) -> Self {
        Self::Primitive(input.into())
    }
}

#[cfg(feature = "jiff")]
impl From<jiff::civil::Date> for Value {
    fn from(input: jiff::civil::Date) -> Self {
        Self::Primitive(input.into())
    }
}

#[cfg(feature = "serde")]
impl From<Value> for serde_json::Value {
    fn from(input: Value) -> Self {
        input.into_json()
    }
}

#[cfg(feature = "serde")]
impl From<&Value> for serde_json::Value {
    fn from(input: &Value) -> Self {
        input.clone().into_json()
    }
}

#[cfg(feature = "bson")]
impl From<Value> for bson::Bson {
    fn from(input: Value) -> Self {
        input.into_bson()
    }
}
