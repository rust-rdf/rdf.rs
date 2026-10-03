// This is free and unencumbered software released into the public domain.

use crate::TimezoneOffset;
use core::fmt;

/// A validated XSD 1.1 `gMonth` with an optional timezone.
///
/// Available without allocation or date/time features. Formatting writes `--mm`
/// followed by the optional offset, using `Z` for UTC. Equality, ordering, and
/// hashing are structural. An absent timezone is distinct from explicit UTC.
/// This replaces the raw `u8` alias: use [`Self::new`] for checked construction
/// and [`Self::month`] to retrieve the field.
/// Parsing follows XSD 1.1 `--mm` syntax; legacy trailing hyphens (`--mm--`)
/// are rejected. Use [`crate::parse_g_month`] to parse the optional timezone.
///
/// With `serde`, the representation is now a struct with `month` (an integer)
/// and `timezone` (optional signed minutes), replacing the former bare integer.
/// Deserialization validates both fields. The enclosing `PrimitiveValue` enum
/// retains its `GMonth` tag; consumers of the former payload must migrate.
/// A missing `timezone` field decodes as absent. Round trips retain the month,
/// offset, and enclosing enum tags, but cannot recover original lexical spelling.
/// Invalid months and offsets are rejected even inside the value wrappers.
///
/// Explicit JSON conversion on [`crate::PrimitiveValue`] or [`crate::Value`]
/// (requires `serde`) instead emits the formatted XSD string. Recover the value
/// with [`crate::parse`] and [`crate::G_MONTH`], not Serde deserialization of
/// the wrapper. The string preserves the month and optional offset, but carries
/// no datatype tag and normalizes signed zero offsets to `Z`.
/// Explicit BSON conversion (requires `bson`, hence `std`) uses the same XSD
/// string, not a BSON date-time or integer month. Reparse with [`crate::G_MONTH`]
/// to recover the value; a string literal with the same text has identical BSON.
///
/// With `borsh`, the type-local version-1 encoding is a version byte `1`, a
/// `u8` month, then `Option<TimezoneOffset>`: tag `0` for absent, or tag `1`
/// followed by little-endian `i16` minutes. This is 3 or 5 bytes with no datatype
/// tag. It replaces the raw alias's single-byte encoding; decode legacy data as
/// `u8` and validate with [`Self::new`] before re-encoding. Decoding rejects
/// unknown versions, invalid months, option tags, and out-of-range offsets.
///
/// See: <https://www.w3.org/TR/xmlschema-2/#gMonth>
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GMonth {
    #[cfg_attr(feature = "serde", serde(deserialize_with = "deserialize_month"))]
    month: u8,
    timezone: Option<TimezoneOffset>,
}

impl GMonth {
    /// Constructs a timezone-free month; returns `None` outside `1..=12`.
    pub const fn new(month: u8) -> Option<Self> {
        if month >= 1 && month <= 12 {
            Some(Self {
                month,
                timezone: None,
            })
        } else {
            None
        }
    }

    /// Returns the month in `1..=12`, without applying a timezone adjustment.
    pub const fn month(self) -> u8 {
        self.month
    }

    /// Sets or removes the timezone without changing the month.
    pub const fn with_timezone(mut self, timezone: Option<TimezoneOffset>) -> Self {
        self.timezone = timezone;
        self
    }

    /// Returns the timezone, distinguishing absence from explicit UTC.
    pub const fn timezone(self) -> Option<TimezoneOffset> {
        self.timezone
    }
}

impl fmt::Display for GMonth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "--{:02}", self.month)?;
        if let Some(offset) = self.timezone {
            offset.fmt(f)?;
        }
        Ok(())
    }
}

#[cfg(feature = "serde")]
fn deserialize_month<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<u8, D::Error> {
    let month = <u8 as serde::Deserialize>::deserialize(deserializer)?;
    GMonth::new(month)
        .map(GMonth::month)
        .ok_or_else(|| serde::de::Error::custom("XSD month is outside 1..=12"))
}

#[cfg(feature = "borsh")]
impl borsh::BorshSerialize for GMonth {
    fn serialize<W: borsh::io::Write>(&self, writer: &mut W) -> Result<(), borsh::io::Error> {
        borsh::BorshSerialize::serialize(&(1u8, self.month, self.timezone), writer)
    }
}

#[cfg(feature = "borsh")]
impl borsh::BorshDeserialize for GMonth {
    fn deserialize_reader<R: borsh::io::Read>(reader: &mut R) -> Result<Self, borsh::io::Error> {
        use borsh::io::{Error, ErrorKind};
        if u8::deserialize_reader(reader)? != 1 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "unsupported XSD gMonth encoding version",
            ));
        }
        let (month, timezone) = <(u8, Option<TimezoneOffset>)>::deserialize_reader(reader)?;
        Self::new(month)
            .map(|month| month.with_timezone(timezone))
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "XSD month is outside 1..=12"))
    }
}
