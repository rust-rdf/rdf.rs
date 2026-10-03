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
///
/// With `serde`, the representation is a struct with `day` (an integer) and
/// `timezone` (optional signed minutes), replacing the former bare integer.
/// Both fields are validated on decoding. The enclosing `PrimitiveValue` enum
/// retains its `GDay` tag; consumers of the former payload must migrate.
/// A missing `timezone` field decodes as absent. Round trips retain the day,
/// offset, and enclosing enum tags, but cannot recover original lexical spelling.
/// Invalid days and offsets are rejected even inside the value wrappers.
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
