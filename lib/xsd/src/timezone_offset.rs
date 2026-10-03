use core::fmt;

/// A validated XSD timezone offset in whole minutes east of UTC.
///
/// Offsets range from `-14:00` through `+14:00`. This allocation-free type is
/// available without date/time features. Use `Option<TimezoneOffset>` to
/// distinguish an absent timezone (`None`) from UTC (`Some(Self::UTC)`).
/// Existing civil date/time aliases do not yet store this type.
///
/// Equality, ordering, and hashing use the signed minute count. Formatting
/// writes `Z` for UTC and `+hh:mm` or `-hh:mm` otherwise; it does not preserve
/// the original spelling of zero. This is an offset, not a named timezone.
///
/// With `serde`, serialization uses a signed integer minute count, not a lexical
/// string. Deserialization validates the XSD bounds. In JSON, an
/// `Option<TimezoneOffset>` distinguishes absence (`null`) from UTC (`0`).
///
/// ```
/// let offset = xsd::TimezoneOffset::from_minutes(330).unwrap();
/// assert_eq!(offset.to_string(), "+05:30");
/// assert_ne!(None, Some(xsd::TimezoneOffset::UTC));
/// assert!(xsd::TimezoneOffset::from_minutes(841).is_none());
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TimezoneOffset(i16);

#[cfg(feature = "serde")]
impl serde::Serialize for TimezoneOffset {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i16(self.0)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for TimezoneOffset {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let minutes = i16::deserialize(deserializer)?;
        Self::from_minutes(minutes)
            .ok_or_else(|| serde::de::Error::custom(TimezoneOffsetError::OutOfRange))
    }
}

/// An invalid XSD timezone-offset spelling or value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum TimezoneOffsetError {
    /// Expected `Z`, `+hh:mm`, or `-hh:mm`, using ASCII digits.
    InvalidLexical,
    /// The offset exceeds 14 hours or its minute field exceeds 59.
    OutOfRange,
    /// A source offset contains seconds that cannot be represented as whole minutes.
    SubMinute,
}

impl fmt::Display for TimezoneOffsetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidLexical => "XSD timezone offsets require Z, +hh:mm, or -hh:mm",
            Self::OutOfRange => {
                "XSD timezone offset is outside -14:00..=+14:00 or has invalid minutes"
            },
            Self::SubMinute => "XSD timezone offsets must use whole minutes",
        })
    }
}

impl core::error::Error for TimezoneOffsetError {}

/// Exact conversion from Jiff, available with `jiff` (or `datetime`).
#[cfg(feature = "jiff")]
impl TryFrom<jiff::tz::Offset> for TimezoneOffset {
    type Error = TimezoneOffsetError;

    /// Rejects offsets outside XSD's 14-hour bound or containing sub-minute
    /// seconds. Neither rounding nor truncation is performed. Range validation
    /// precedes precision validation when both constraints are violated.
    fn try_from(offset: jiff::tz::Offset) -> Result<Self, Self::Error> {
        let seconds = offset.seconds();
        if !(-50_400..=50_400).contains(&seconds) {
            return Err(TimezoneOffsetError::OutOfRange);
        }
        if seconds % 60 != 0 {
            return Err(TimezoneOffsetError::SubMinute);
        }
        Ok(Self((seconds / 60) as i16))
    }
}

/// Lossless conversion to Jiff, available with `jiff` (or `datetime`).
#[cfg(feature = "jiff")]
impl From<TimezoneOffset> for jiff::tz::Offset {
    fn from(offset: TimezoneOffset) -> Self {
        Self::from_seconds(i32::from(offset.minutes()) * 60)
            .expect("validated XSD offsets fit in Jiff's offset range")
    }
}

impl core::str::FromStr for TimezoneOffset {
    type Err = TimezoneOffsetError;

    /// Parses exact XSD timezone syntax without whitespace preprocessing.
    ///
    /// Accepts uppercase `Z` or signed `hh:mm` in the inclusive range
    /// `-14:00..=+14:00`. Both signed zero spellings normalize to UTC.
    /// Empty input is an error, not an absent timezone. Returns
    /// [`TimezoneOffsetError::InvalidLexical`] for malformed syntax and
    /// [`TimezoneOffsetError::OutOfRange`] for invalid field values.
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input == "Z" {
            return Ok(Self::UTC);
        }
        let [sign @ (b'+' | b'-'), h1, h2, b':', m1, m2] = input.as_bytes() else {
            return Err(TimezoneOffsetError::InvalidLexical);
        };
        if ![h1, h2, m1, m2].iter().all(|byte| byte.is_ascii_digit()) {
            return Err(TimezoneOffsetError::InvalidLexical);
        }
        let hours = i16::from((h1 - b'0') * 10 + (h2 - b'0'));
        let minutes = i16::from((m1 - b'0') * 10 + (m2 - b'0'));
        if minutes > 59 {
            return Err(TimezoneOffsetError::OutOfRange);
        }
        let total = hours * 60 + minutes;
        Self::from_minutes(if *sign == b'-' { -total } else { total })
            .ok_or(TimezoneOffsetError::OutOfRange)
    }
}

impl TimezoneOffset {
    /// The explicit UTC timezone, distinct from an absent timezone.
    pub const UTC: Self = Self(0);

    /// Constructs an offset in minutes east of UTC.
    ///
    /// Returns `None` outside the inclusive XSD range `-840..=840` minutes.
    pub const fn from_minutes(minutes: i16) -> Option<Self> {
        if minutes >= -840 && minutes <= 840 {
            Some(Self(minutes))
        } else {
            None
        }
    }

    /// Returns the signed number of whole minutes east of UTC.
    pub const fn minutes(self) -> i16 {
        self.0
    }
}

impl fmt::Display for TimezoneOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 == 0 {
            return f.write_str("Z");
        }
        let magnitude = self.0.unsigned_abs();
        write!(
            f,
            "{}{:02}:{:02}",
            if self.0 < 0 { '-' } else { '+' },
            magnitude / 60,
            magnitude % 60
        )
    }
}
