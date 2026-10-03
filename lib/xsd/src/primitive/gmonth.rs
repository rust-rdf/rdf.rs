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
///
/// With `serde`, the representation is now a struct with `month` (an integer)
/// and `timezone` (optional signed minutes), replacing the former bare integer.
/// Deserialization validates both fields. The enclosing `PrimitiveValue` enum
/// retains its `GMonth` tag; consumers of the former payload must migrate.
/// A missing `timezone` field decodes as absent. Round trips retain the month,
/// offset, and enclosing enum tags, but cannot recover original lexical spelling.
/// Invalid months and offsets are rejected even inside the value wrappers.
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
