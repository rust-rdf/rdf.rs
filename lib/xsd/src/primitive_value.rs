// This is free and unencumbered software released into the public domain.

use crate::primitive::{Boolean, GDay, GMonth, GMonthDay, GYear, GYearMonth};
use crate::{
    PrimitiveType,
    primitive::{Decimal, Double, Float},
};
use core::fmt;

#[cfg(feature = "jiff")]
use crate::primitive::{Date, DateTime, Duration, Time};

#[cfg(feature = "alloc")]
use ::alloc::{borrow::Cow, string::String, vec::Vec};

/// Value representation for XSD primitive datatypes.
///
/// [`Display`](core::fmt::Display) writes lexical content without a datatype
/// label. Partial calendar fields are zero-padded; binary values use uppercase
/// hexadecimal or padded Base64. Formatting does not validate stored fields or
/// preserve the original spelling of a parsed literal.
///
/// Partial-calendar values use the same lexical strings for explicit JSON
/// (`serde`) and BSON (`bson`) conversion as for `Display`. These conversions
/// do not validate raw calendar fields or include datatype identifiers or
/// timezones. Derived Serde serialization uses a separate enum representation.
///
/// See: <https://www.w3.org/TR/xmlschema-2/#built-in-datatypes>
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
// #[cfg_attr(
//     feature = "borsh",
//     derive(borsh::BorshSerialize, borsh::BorshDeserialize) // FIXME: SignedDuration
// )]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PrimitiveValue {
    /// See: <https://www.w3.org/TR/xmlschema-2/#string>
    #[cfg(feature = "alloc")]
    String(String),
    #[cfg(not(feature = "alloc"))]
    String(&'static str),

    /// See: <https://www.w3.org/TR/xmlschema-2/#boolean>
    Boolean(Boolean),

    /// See: <https://www.w3.org/TR/xmlschema-2/#decimal>
    Decimal(Decimal),

    /// See: <https://www.w3.org/TR/xmlschema-2/#float>
    Float(Float),

    /// See: <https://www.w3.org/TR/xmlschema-2/#double>
    Double(Double),

    /// See: <https://www.w3.org/TR/xmlschema-2/#duration>
    #[cfg(feature = "jiff")]
    Duration(Duration),

    /// An XSD date-time with an optional timezone, available with `jiff` (enabled by `datetime`).
    ///
    /// `Display` uses the date formatting of [`Self::Date`], followed by `T`
    /// and `hh:mm:ss` with fractional seconds when nonzero. Fractional seconds
    /// retain nanosecond precision and omit trailing zeros. The optional timezone
    /// is appended once, and the stored year number is preserved without an era adjustment.
    /// Explicit JSON (`serde`) and BSON (`bson`) conversions emit this same
    /// XSD lexical string, preserving nanoseconds and negative-year formatting.
    /// They carry no datatype identifier and do not use BSON's millisecond
    /// timestamp representation. Derived Serde serialization is separate.
    ///
    /// ```
    /// use xsd::{PrimitiveValue, primitive::DateTime};
    /// let value = PrimitiveValue::DateTime(
    ///     DateTime::new(-1, 1, 2, 12, 34, 56, 125_000_000).unwrap(),
    /// );
    /// assert_eq!(value.to_string(), "-0001-01-02T12:34:56.125");
    /// ```
    ///
    /// See: <https://www.w3.org/TR/xmlschema-2/#dateTime>
    #[cfg(feature = "jiff")]
    DateTime(DateTime),

    /// An XSD time with an optional timezone and nanosecond precision.
    ///
    /// `Display` and explicit JSON/BSON conversion preserve the clock and offset,
    /// using `Z` for UTC. See [`Time`] for formatting and Serde encoding details.
    /// See: <https://www.w3.org/TR/xmlschema-2/#time>
    #[cfg(feature = "jiff")]
    Time(Time),

    /// An XSD date with an optional timezone, available with `jiff` (enabled by `datetime`).
    ///
    /// `Display` writes `yyyy-mm-dd`, with a minus sign before the four-digit
    /// year for negative years. It preserves the stored year number without
    /// applying a historical-era adjustment. A present timezone is appended,
    /// using `Z` for UTC and signed `hh:mm` otherwise.
    /// Explicit JSON (`serde`) and BSON (`bson`) conversions emit this same
    /// XSD lexical string, without a datatype identifier. They do not use BSON's
    /// timestamp representation. Derived Serde serialization is separate.
    ///
    /// ```
    /// use xsd::{PrimitiveValue, primitive::Date};
    /// let value = PrimitiveValue::Date(Date::new(-1, 1, 2).unwrap());
    /// assert_eq!(value.to_string(), "-0001-01-02");
    /// ```
    ///
    /// See: <https://www.w3.org/TR/xmlschema-2/#date>
    #[cfg(feature = "jiff")]
    Date(Date),

    /// A year and month, formatted as `yyyy-mm` with a sign for negative years.
    /// JSON/BSON string conversion follows `Display`; see the type-level docs.
    ///
    /// See: <https://www.w3.org/TR/xmlschema-2/#gYearMonth>
    GYearMonth(GYearMonth),

    /// A year, formatted with at least four digits and a sign for negative years.
    /// JSON/BSON string conversion follows `Display`; see the type-level docs.
    ///
    /// See: <https://www.w3.org/TR/xmlschema-2/#gYear>
    GYear(GYear),

    /// A month and day, formatted as `--mm-dd`.
    /// JSON/BSON string conversion follows `Display`; see the type-level docs.
    ///
    /// See: <https://www.w3.org/TR/xmlschema-2/#gMonthDay>
    GMonthDay(GMonthDay),

    /// A day of the month, formatted as `---dd`.
    /// JSON/BSON string conversion follows `Display`; see the type-level docs.
    ///
    /// See: <https://www.w3.org/TR/xmlschema-2/#gDay>
    GDay(GDay),

    /// A month, formatted as `--mm`.
    /// JSON/BSON string conversion follows `Display`; see the type-level docs.
    ///
    /// See: <https://www.w3.org/TR/xmlschema-2/#gMonth>
    GMonth(GMonth),

    /// Binary data formatted as uppercase hexadecimal, two digits per byte.
    ///
    /// Explicit JSON conversion (`serde`) uses the same lexical string as
    /// `Display`, including an empty string for empty data. BSON conversion
    /// (`bson`) preserves the bytes as generic binary data. Neither encoding
    /// carries the XSD datatype identifier. Derived Serde serialization is a
    /// separate, enum-based representation.
    ///
    /// See: <https://www.w3.org/TR/xmlschema-2/#hexBinary>
    #[cfg(feature = "alloc")]
    HexBinary(Vec<u8>),

    /// Binary data formatted as padded Base64 using the standard `+/` alphabet.
    ///
    /// Explicit JSON conversion (`serde`) uses the same lexical string as
    /// `Display`, without whitespace and with an empty string for empty data.
    /// BSON conversion (`bson`) preserves the bytes as generic binary data.
    /// Neither encoding carries the XSD datatype identifier. Derived Serde
    /// serialization is a separate, enum-based representation.
    ///
    /// See: <https://www.w3.org/TR/xmlschema-2/#base64Binary>
    #[cfg(feature = "alloc")]
    Base64Binary(Vec<u8>),

    /// See: <https://www.w3.org/TR/xmlschema-2/#anyURI>
    #[cfg(feature = "alloc")]
    AnyUri(String),

    /// A lexical prefix and local name, formatted as `prefix:local` or just
    /// `local` when the prefix is empty.
    ///
    /// Explicit JSON (`serde`) and BSON (`bson`) conversion use the same string
    /// as `Display`, without a datatype identifier. These operations do not
    /// validate names or resolve prefixes to namespace IRIs. Derived Serde
    /// serialization retains the two fields in a separate enum representation.
    ///
    /// See: <https://www.w3.org/TR/xmlschema-2/#QName>
    #[cfg(feature = "alloc")]
    QName(String, String),
}

impl fmt::Display for PrimitiveValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use PrimitiveValue::*;
        match self {
            String(s) => f.write_str(s),
            Boolean(b) => b.fmt(f),
            Decimal(d) => d.fmt(f),
            Float(n) => n.fmt(f),
            Double(n) => n.fmt(f),
            #[cfg(feature = "jiff")]
            Duration(d) => d.fmt(f),
            #[cfg(feature = "jiff")]
            DateTime(d) => d.fmt(f),
            #[cfg(feature = "jiff")]
            Time(t) => t.fmt(f),
            #[cfg(feature = "jiff")]
            Date(d) => d.fmt(f),
            GYearMonth((y, m)) => {
                fmt_year(*y, f)?;
                write!(f, "-{m:02}")
            },
            GYear(y) => fmt_year(*y, f),
            GMonthDay((m, d)) => write!(f, "--{m:02}-{d:02}"),
            GDay(d) => write!(f, "---{d:02}"),
            GMonth(m) => write!(f, "--{m:02}"),
            #[cfg(feature = "alloc")]
            HexBinary(bytes) => {
                for byte in bytes {
                    write!(f, "{byte:02X}")?;
                }
                Ok(())
            },
            #[cfg(feature = "alloc")]
            Base64Binary(bytes) => fmt_base64(bytes, f),
            #[cfg(feature = "alloc")]
            AnyUri(uri) => f.write_str(uri),
            #[cfg(feature = "alloc")]
            QName(prefix, local) => {
                if !prefix.is_empty() {
                    write!(f, "{prefix}:")?;
                }
                f.write_str(local)
            },
        }
    }
}

fn fmt_year(year: GYear, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if year < 0 {
        write!(f, "-{:04}", year.unsigned_abs())
    } else {
        write!(f, "{year:04}")
    }
}

#[cfg(feature = "alloc")]
fn fmt_base64(bytes: &[u8], f: &mut fmt::Formatter<'_>) -> fmt::Result {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    for chunk in bytes.chunks(3) {
        let a = chunk[0];
        let b = chunk.get(1).copied().unwrap_or(0);
        let c = chunk.get(2).copied().unwrap_or(0);
        write!(
            f,
            "{}{}{}{}",
            ALPHABET[(a >> 2) as usize] as char,
            ALPHABET[(((a & 3) << 4) | (b >> 4)) as usize] as char,
            if chunk.len() > 1 {
                ALPHABET[(((b & 15) << 2) | (c >> 6)) as usize] as char
            } else {
                '='
            },
            if chunk.len() > 2 {
                ALPHABET[(c & 63) as usize] as char
            } else {
                '='
            },
        )?;
    }
    Ok(())
}

impl PrimitiveValue {
    /// Constructs a timezone-free `xsd:gYearMonth`, validating the month.
    ///
    /// Returns `None` unless `month` is in `1..=12`. All `i32` years are accepted,
    /// including zero following XSD 1.1. Formatting preserves the signed year,
    /// padded to at least four digits, followed by `-mm`; no era adjustment is
    /// applied. Available without allocation or date/time features. Direct
    /// [`Self::GYearMonth`] construction is unchecked.
    ///
    /// ```
    /// let value = xsd::PrimitiveValue::g_year_month(-1, 2).unwrap();
    /// assert_eq!(value.to_string(), "-0001-02");
    /// assert!(xsd::PrimitiveValue::g_year_month(2026, 13).is_none());
    /// ```
    pub const fn g_year_month(year: GYear, month: GMonth) -> Option<Self> {
        if month >= 1 && month <= 12 {
            Some(Self::GYearMonth((year, month)))
        } else {
            None
        }
    }

    /// Constructs a timezone-free `xsd:gMonthDay`, validating both fields.
    ///
    /// Returns `None` for an invalid month or a day outside that month's range.
    /// February 29 is valid because no year is specified; February 30 and April
    /// 31 are not. Available without allocation or date/time features. Formatting
    /// uses `--mm-dd`. Direct [`Self::GMonthDay`] construction is unchecked.
    ///
    /// ```
    /// let leap_day = xsd::PrimitiveValue::g_month_day(2, 29).unwrap();
    /// assert_eq!(leap_day.to_string(), "--02-29");
    /// assert!(xsd::PrimitiveValue::g_month_day(2, 30).is_none());
    /// ```
    pub const fn g_month_day(month: GMonth, day: GDay) -> Option<Self> {
        let last_day = match month {
            2 => 29,
            4 | 6 | 9 | 11 => 30,
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            _ => return None,
        };
        if day >= 1 && day <= last_day {
            Some(Self::GMonthDay((month, day)))
        } else {
            None
        }
    }

    /// Constructs a timezone-free `xsd:gDay`, validating the day of the month.
    ///
    /// Returns `None` unless `day` is in `1..=31`. No month or year is implied,
    /// so day 31 is valid. Available without allocation or date/time features.
    /// Formatting uses `---dd`. Direct [`Self::GDay`] construction is unchecked.
    ///
    /// ```
    /// assert_eq!(xsd::PrimitiveValue::g_day(31).unwrap().to_string(), "---31");
    /// assert!(xsd::PrimitiveValue::g_day(32).is_none());
    /// ```
    pub const fn g_day(day: GDay) -> Option<Self> {
        if day >= 1 && day <= 31 {
            Some(Self::GDay(day))
        } else {
            None
        }
    }

    /// Constructs a timezone-free `xsd:gMonth`, validating the month.
    ///
    /// Returns `None` unless `month` is in `1..=12`. Available without allocation
    /// or date/time features. Formatting uses `--mm`. Direct construction with
    /// [`Self::GMonth`] remains unchecked; this constructor adds no timezone.
    ///
    /// ```
    /// let value = xsd::PrimitiveValue::g_month(2).unwrap();
    /// assert_eq!(value.to_string(), "--02");
    /// assert!(xsd::PrimitiveValue::g_month(0).is_none());
    /// ```
    pub const fn g_month(month: GMonth) -> Option<Self> {
        if month >= 1 && month <= 12 {
            Some(Self::GMonth(month))
        } else {
            None
        }
    }

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
            GYearMonth(_) => PrimitiveType::GYearMonth,
            GYear(_) => PrimitiveType::GYear,
            GMonthDay(_) => PrimitiveType::GMonthDay,
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

    /// Clones and converts using [`Self::into_json`]. Requires `serde`.
    /// Always returns `Some`, including string encodings for non-finite floats.
    #[cfg(feature = "serde")]
    pub fn to_json(&self) -> Option<serde_json::Value> {
        Some(self.clone().into_json())
    }

    /// Converts to an untagged JSON value. Requires `serde`.
    ///
    /// Decimal values use strings preserving the stored decimal precision,
    /// matching [`crate::DecimalValue::into_json`]. This replaces their former
    /// numeric encoding. Datatype identity and original lexical spelling are
    /// not included; this API differs from derived Serde serialization.
    /// Finite floats use JSON numbers (binary32 values widen exactly to binary64).
    /// Non-finite floats use XSD strings `INF`, `-INF`, or `NaN`, replacing the
    /// former panic. Signed finite zero is retained; NaN payload bits are not.
    #[cfg(feature = "serde")]
    pub fn into_json(self) -> serde_json::Value {
        use PrimitiveValue::*;
        use alloc::string::ToString;
        use serde_json::Value;
        #[allow(unused)]
        match self {
            String(s) => Value::String(s),
            Boolean(b) => b.into_json(),
            Decimal(d) => Value::String(d.to_string()),
            Float(f) => float_into_json(f32::from(f) as f64),
            Double(d) => float_into_json(f64::from(d)),
            #[cfg(feature = "jiff")]
            Duration(d) => Value::String(d.to_string()),
            #[cfg(feature = "jiff")]
            value @ DateTime(_) => Value::String(value.to_string()),
            #[cfg(feature = "jiff")]
            Time(t) => Value::String(t.to_string()),
            #[cfg(feature = "jiff")]
            value @ Date(_) => Value::String(value.to_string()),
            value @ (GYearMonth(_) | GYear(_) | GMonthDay(_) | GDay(_) | GMonth(_)) => {
                Value::String(value.to_string())
            },
            #[cfg(feature = "alloc")]
            value @ HexBinary(_) => Value::String(value.to_string()),
            #[cfg(feature = "alloc")]
            value @ Base64Binary(_) => Value::String(value.to_string()),
            #[cfg(feature = "alloc")]
            AnyUri(u) => Value::String(u),
            #[cfg(feature = "alloc")]
            value @ QName(_, _) => Value::String(value.to_string()),
        }
    }

    #[cfg(feature = "bson")]
    pub fn to_bson(&self) -> Option<bson::Bson> {
        Some(self.clone().into_bson())
    }

    #[cfg(feature = "bson")]
    pub fn into_bson(self) -> bson::Bson {
        use PrimitiveValue::*;
        use alloc::string::ToString;
        use bson::{Binary, Bson, spec::BinarySubtype};
        #[allow(unused)]
        match self {
            String(s) => Bson::String(s),
            Boolean(b) => b.into_bson(),
            Decimal(d) => d.into_bson().unwrap(),
            Float(f) => f.into_bson(),
            Double(d) => d.into_bson(),
            #[cfg(feature = "jiff")]
            Duration(d) => Bson::String(d.to_string()),
            #[cfg(feature = "jiff")]
            value @ DateTime(_) => Bson::String(value.to_string()),
            #[cfg(feature = "jiff")]
            Time(t) => Bson::String(t.to_string()),
            #[cfg(feature = "jiff")]
            value @ Date(_) => Bson::String(value.to_string()),
            value @ (GYearMonth(_) | GYear(_) | GMonthDay(_) | GDay(_) | GMonth(_)) => {
                Bson::String(value.to_string())
            },
            #[cfg(feature = "alloc")]
            HexBinary(b) => Bson::Binary(Binary {
                bytes: b,
                subtype: BinarySubtype::Generic,
            }),
            #[cfg(feature = "alloc")]
            Base64Binary(b) => Bson::Binary(Binary {
                bytes: b,
                subtype: BinarySubtype::Generic,
            }),
            #[cfg(feature = "alloc")]
            AnyUri(u) => Bson::String(u.to_string()),
            #[cfg(feature = "alloc")]
            value @ QName(_, _) => Bson::String(value.to_string()),
        }
    }
}

#[cfg(feature = "serde")]
fn float_into_json(number: f64) -> serde_json::Value {
    match serde_json::Number::from_f64(number) {
        Some(number) => serde_json::Value::Number(number),
        None => serde_json::Value::String(
            if number.is_nan() {
                "NaN"
            } else if number.is_sign_negative() {
                "-INF"
            } else {
                "INF"
            }
            .into(),
        ),
    }
}

impl<T> From<&T> for PrimitiveValue
where
    T: Clone + Into<Self>,
{
    fn from(t: &T) -> Self {
        t.clone().into()
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

impl From<bool> for PrimitiveValue {
    fn from(input: bool) -> Self {
        Self::Boolean(input.into())
    }
}

impl From<Boolean> for PrimitiveValue {
    fn from(input: Boolean) -> Self {
        Self::Boolean(input)
    }
}

impl From<i8> for PrimitiveValue {
    fn from(input: i8) -> Self {
        Self::Decimal(input.into())
    }
}

impl From<i16> for PrimitiveValue {
    fn from(input: i16) -> Self {
        Self::Decimal(input.into())
    }
}

impl From<i32> for PrimitiveValue {
    fn from(input: i32) -> Self {
        Self::Decimal(input.into())
    }
}

impl From<i64> for PrimitiveValue {
    fn from(input: i64) -> Self {
        Self::Decimal(input.into())
    }
}

impl From<i128> for PrimitiveValue {
    fn from(input: i128) -> Self {
        Self::Decimal(input.into())
    }
}

impl From<isize> for PrimitiveValue {
    fn from(input: isize) -> Self {
        Self::Decimal(input.into())
    }
}

#[cfg(feature = "rust_decimal")]
impl From<rust_decimal::Decimal> for PrimitiveValue {
    fn from(input: rust_decimal::Decimal) -> Self {
        Self::Decimal(input.into())
    }
}

impl From<f32> for PrimitiveValue {
    fn from(input: f32) -> Self {
        Self::Float(input.into())
    }
}

impl From<Float> for PrimitiveValue {
    fn from(input: Float) -> Self {
        Self::Float(input.into())
    }
}

impl From<f64> for PrimitiveValue {
    fn from(input: f64) -> Self {
        Self::Double(input.into())
    }
}

impl From<Double> for PrimitiveValue {
    fn from(input: Double) -> Self {
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
impl From<jiff::civil::DateTime> for PrimitiveValue {
    fn from(input: jiff::civil::DateTime) -> Self {
        Self::DateTime(input.into())
    }
}

#[cfg(feature = "jiff")]
impl From<DateTime> for PrimitiveValue {
    fn from(input: DateTime) -> Self {
        Self::DateTime(input)
    }
}

#[cfg(feature = "jiff")]
impl From<jiff::civil::Time> for PrimitiveValue {
    fn from(input: jiff::civil::Time) -> Self {
        Self::Time(input.into())
    }
}

#[cfg(feature = "jiff")]
impl From<Time> for PrimitiveValue {
    fn from(input: Time) -> Self {
        Self::Time(input)
    }
}

#[cfg(feature = "jiff")]
impl From<jiff::civil::Date> for PrimitiveValue {
    fn from(input: jiff::civil::Date) -> Self {
        Self::Date(input.into())
    }
}

#[cfg(feature = "jiff")]
impl From<Date> for PrimitiveValue {
    fn from(input: Date) -> Self {
        Self::Date(input)
    }
}

#[cfg(feature = "alloc")]
impl From<Vec<u8>> for PrimitiveValue {
    fn from(input: Vec<u8>) -> Self {
        Self::Base64Binary(input)
    }
}

#[cfg(feature = "jiff")]
impl TryFrom<jiff::Span> for PrimitiveValue {
    type Error = jiff::Error;

    fn try_from(input: jiff::Span) -> Result<Self, Self::Error> {
        Ok(Self::Duration(jiff::SignedDuration::try_from(input)?))
    }
}

#[cfg(feature = "serde")]
impl From<PrimitiveValue> for serde_json::Value {
    fn from(input: PrimitiveValue) -> Self {
        input.into_json()
    }
}

#[cfg(feature = "serde")]
impl From<&PrimitiveValue> for serde_json::Value {
    fn from(input: &PrimitiveValue) -> Self {
        input.clone().into_json()
    }
}

#[cfg(feature = "bson")]
impl From<PrimitiveValue> for bson::Bson {
    fn from(input: PrimitiveValue) -> Self {
        input.into_bson()
    }
}
