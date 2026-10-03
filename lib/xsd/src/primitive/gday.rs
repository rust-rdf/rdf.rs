// This is free and unencumbered software released into the public domain.

use crate::TimezoneOffset;
use core::fmt;

/// A validated XSD 1.1 `gDay` with an optional timezone.
///
/// Available without allocation or date/time features. All days in `1..=31`
/// are valid because no month or year is specified. Formatting writes `---dd`
/// followed by the optional offset, using `Z` for UTC. Equality, ordering, and
/// hashing are structural; absence differs from explicit UTC.
/// This replaces the raw `u8` alias: use [`Self::new`] for checked construction
/// and [`Self::day`] to retrieve the field.
/// Parsing follows XSD 1.1 `---dd` syntax, with exactly two ASCII day digits.
/// Use [`crate::parse_g_day`] to parse the optional timezone; whitespace is not
/// trimmed. Day 31 remains valid with any supported offset.
///
/// With `serde`, the representation is a struct with `day` (an integer) and
/// `timezone` (optional signed minutes), replacing the former bare integer.
/// Both fields are validated on decoding. The enclosing `PrimitiveValue` enum
/// retains its `GDay` tag; consumers of the former payload must migrate.
/// A missing `timezone` field decodes as absent. Round trips retain the day,
/// offset, and enclosing enum tags, but cannot recover original lexical spelling.
/// Invalid days and offsets are rejected even inside the value wrappers.
///
/// Explicit JSON conversion on [`crate::PrimitiveValue`] or [`crate::Value`]
/// (requires `serde`) instead emits the formatted XSD string. Recover the value
/// with [`crate::parse`] and [`crate::G_DAY`], not Serde deserialization of the
/// wrapper. The string preserves the day and optional offset, but carries no
/// datatype tag and normalizes signed zero offsets to `Z`.
/// Explicit BSON conversion (requires `bson`, hence `std`) uses the same XSD
/// string, not a BSON date-time or integer day. Reparse with [`crate::G_DAY`]
/// to recover the value; a string literal with the same text has identical BSON.
///
/// With `borsh`, the type-local version-1 encoding is a version byte `1`, a
/// `u8` day, then `Option<TimezoneOffset>`: tag `0` for absent, or tag `1`
/// followed by little-endian `i16` minutes. This is 3 or 5 bytes with no datatype
/// tag. It replaces the raw alias's single-byte encoding; decode legacy data as
/// `u8` and validate with [`Self::new`] before re-encoding. Decoding rejects
/// unknown versions, invalid days, option tags, and out-of-range offsets.
///
/// See: <https://www.w3.org/TR/xmlschema-2/#gDay>
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GDay {
    #[cfg_attr(feature = "serde", serde(deserialize_with = "deserialize_day"))]
    day: u8,
    timezone: Option<TimezoneOffset>,
}

impl GDay {
    /// Constructs a timezone-free day; returns `None` outside `1..=31`.
    pub const fn new(day: u8) -> Option<Self> {
        if day >= 1 && day <= 31 {
            Some(Self {
                day,
                timezone: None,
            })
        } else {
            None
        }
    }

    /// Returns the day in `1..=31`, without applying a timezone adjustment.
    pub const fn day(self) -> u8 {
        self.day
    }

    /// Sets or removes the timezone without changing the day.
    pub const fn with_timezone(mut self, timezone: Option<TimezoneOffset>) -> Self {
        self.timezone = timezone;
        self
    }

    /// Returns the timezone, distinguishing absence from explicit UTC.
    pub const fn timezone(self) -> Option<TimezoneOffset> {
        self.timezone
    }
}

impl fmt::Display for GDay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "---{:02}", self.day)?;
        if let Some(offset) = self.timezone {
            offset.fmt(f)?;
        }
        Ok(())
    }
}

#[cfg(feature = "serde")]
fn deserialize_day<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<u8, D::Error> {
    let day = <u8 as serde::Deserialize>::deserialize(deserializer)?;
    GDay::new(day)
        .map(GDay::day)
        .ok_or_else(|| serde::de::Error::custom("XSD day is outside 1..=31"))
}

#[cfg(feature = "borsh")]
impl borsh::BorshSerialize for GDay {
    fn serialize<W: borsh::io::Write>(&self, writer: &mut W) -> Result<(), borsh::io::Error> {
        borsh::BorshSerialize::serialize(&(1u8, self.day, self.timezone), writer)
    }
}

#[cfg(feature = "borsh")]
impl borsh::BorshDeserialize for GDay {
    fn deserialize_reader<R: borsh::io::Read>(reader: &mut R) -> Result<Self, borsh::io::Error> {
        use borsh::io::{Error, ErrorKind};
        if u8::deserialize_reader(reader)? != 1 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "unsupported XSD gDay encoding version",
            ));
        }
        let (day, timezone) = <(u8, Option<TimezoneOffset>)>::deserialize_reader(reader)?;
        Self::new(day)
            .map(|day| day.with_timezone(timezone))
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "XSD day is outside 1..=31"))
    }
}
