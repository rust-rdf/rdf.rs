// This is free and unencumbered software released into the public domain.

use crate::TimezoneOffset;
use core::fmt;

/// A validated XSD 1.1 `gMonthDay` with an optional timezone.
///
/// Available without allocation or date/time features. Month/day combinations
/// follow Gregorian month lengths, allowing February 29 because no year is
/// specified. Formatting writes `--mm-dd` followed by the optional offset, using
/// `Z` for UTC. Equality, ordering, and hashing are structural; absence differs
/// from explicit UTC. `Ord` compares month, then day, then the optional signed
/// minute count, with absence before any present offset. This total order is for
/// Rust collections, not XSD temporal comparison: no timezone normalization or
/// uncertainty from absent timezones is considered. `Eq` and `Hash` use the same
/// fields. No XSD semantic comparison operation is provided yet.
///
/// ```
/// use xsd::{primitive::GMonthDay, TimezoneOffset};
/// let leap_day = GMonthDay::new(2, 29).unwrap();
/// let utc = leap_day.with_timezone(Some(TimezoneOffset::UTC));
/// assert_ne!(leap_day, utc);
/// assert!(leap_day < utc); // structural ordering only
/// ```
///
/// This replaces the raw `(u8, u8)` alias: use [`Self::new`]
/// for checked construction and [`Self::month`] and [`Self::day`] for the fields.
/// Parsing follows XSD 1.1 `--mm-dd` syntax, with exactly two ASCII digits per
/// field. Use [`crate::parse_g_month_day`] to parse the optional timezone;
/// whitespace is not trimmed. February 29 is valid with any supported offset.
/// This follows [XSD 1.1 §3.3.12](https://www.w3.org/TR/xmlschema11-2/#gMonthDay),
/// even though the optional differential-test reference `oxsdatatypes` 0.2.3
/// rejects February 29.
/// Parsed values do not retain RDF lexical identity: `--02-29Z`,
/// `--02-29+00:00`, and `--02-29-00:00` become the same value. RDF applications
/// must retain original lexical strings and datatype identifiers separately
/// when term identity matters.
///
/// With `serde`, the representation is a struct with `month`, `day`, and
/// `timezone` (optional signed minutes), replacing the former two-element tuple.
/// Decoding validates the combined calendar fields and offset. The enclosing
/// `PrimitiveValue` enum retains its `GMonthDay` tag; old payloads must migrate.
/// A missing `timezone` field decodes as absent. Round trips preserve the fields,
/// offset, and enclosing enum tags, but not original lexical spelling. Invalid
/// combinations such as February 30 are rejected even inside value wrappers.
///
/// Explicit JSON conversion on [`crate::PrimitiveValue`] or [`crate::Value`]
/// (requires `serde`) instead emits the formatted XSD string. Recover the value
/// with [`crate::parse`] and [`crate::G_MONTH_DAY`], not Serde deserialization of
/// the wrapper. This preserves calendar fields and optional offset, but carries
/// no datatype tag and normalizes signed zero offsets to `Z`.
/// Explicit BSON conversion (requires `bson`, hence `std`) uses the same XSD
/// string, not a BSON date-time or array. Reparse with [`crate::G_MONTH_DAY`]
/// to recover the value; a string literal with the same text has identical BSON.
///
/// With `borsh`, the type-local version-1 encoding is a version byte `1`, a
/// `u8` month, a `u8` day, then `Option<TimezoneOffset>`: tag `0` for absent,
/// or tag `1` followed by little-endian `i16` minutes. This is 4 or 6 bytes with
/// no datatype tag. It replaces the raw alias's two-byte encoding; decode legacy
/// data as `(u8, u8)` and validate with [`Self::new`] before re-encoding.
/// Decoding rejects unknown versions, impossible calendar combinations, invalid
/// option tags, and out-of-range offsets.
///
/// See: <https://www.w3.org/TR/xmlschema-2/#gMonthDay>
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct GMonthDay {
    month: u8,
    day: u8,
    timezone: Option<TimezoneOffset>,
}

impl GMonthDay {
    /// Constructs a timezone-free month/day, returning `None` for invalid fields.
    /// February 29 is valid; February 30 and April 31 are not.
    pub const fn new(month: u8, day: u8) -> Option<Self> {
        let last_day = match month {
            2 => 29,
            4 | 6 | 9 | 11 => 30,
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            _ => return None,
        };
        if day >= 1 && day <= last_day {
            Some(Self {
                month,
                day,
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

    /// Returns the valid day of the stored month, without timezone adjustment.
    pub const fn day(self) -> u8 {
        self.day
    }

    /// Sets or removes the timezone without changing the calendar fields.
    pub const fn with_timezone(mut self, timezone: Option<TimezoneOffset>) -> Self {
        self.timezone = timezone;
        self
    }

    /// Returns the timezone, distinguishing absence from explicit UTC.
    pub const fn timezone(self) -> Option<TimezoneOffset> {
        self.timezone
    }
}

impl fmt::Display for GMonthDay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "--{:02}-{:02}", self.month, self.day)?;
        if let Some(offset) = self.timezone {
            offset.fmt(f)?;
        }
        Ok(())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for GMonthDay {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        #[serde(rename = "GMonthDay")]
        struct Fields {
            month: u8,
            day: u8,
            timezone: Option<TimezoneOffset>,
        }
        let fields = Fields::deserialize(deserializer)?;
        Self::new(fields.month, fields.day)
            .map(|value| value.with_timezone(fields.timezone))
            .ok_or_else(|| serde::de::Error::custom("invalid XSD month/day combination"))
    }
}

#[cfg(feature = "borsh")]
impl borsh::BorshSerialize for GMonthDay {
    fn serialize<W: borsh::io::Write>(&self, writer: &mut W) -> Result<(), borsh::io::Error> {
        borsh::BorshSerialize::serialize(&(1u8, self.month, self.day, self.timezone), writer)
    }
}

#[cfg(feature = "borsh")]
impl borsh::BorshDeserialize for GMonthDay {
    fn deserialize_reader<R: borsh::io::Read>(reader: &mut R) -> Result<Self, borsh::io::Error> {
        use borsh::io::{Error, ErrorKind};
        if u8::deserialize_reader(reader)? != 1 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "unsupported XSD gMonthDay encoding version",
            ));
        }
        let (month, day, timezone) =
            <(u8, u8, Option<TimezoneOffset>)>::deserialize_reader(reader)?;
        Self::new(month, day)
            .map(|value| value.with_timezone(timezone))
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "invalid XSD month/day combination"))
    }
}
