// This is free and unencumbered software released into the public domain.

use crate::TimezoneOffset;
use core::fmt;

/// A validated XSD 1.1 `gYearMonth` with an optional timezone.
///
/// Available without allocation or date/time features. Years span the entire
/// `i32` range, including zero as in XSD 1.1; months must be in `1..=12`.
/// Formatting writes a signed year padded to at least four digits, then `-mm`
/// and the optional offset (`Z` for UTC), without an era adjustment. Equality,
/// ordering, and hashing are structural; absence differs from explicit UTC.
/// This replaces the raw `(i32, u8)` alias: use [`Self::new`] for construction
/// and [`Self::year`] and [`Self::month`] for the fields.
///
/// With `serde`, the representation is a struct with `year`, `month`, and
/// `timezone` (optional signed minutes), replacing the former two-element tuple.
/// Decoding validates the month and offset and enforces the `i32` year range.
/// The enclosing `PrimitiveValue` enum retains its `GYearMonth` tag; old
/// payloads must migrate.
///
/// See: <https://www.w3.org/TR/xmlschema-2/#gYearMonth>
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GYearMonth {
    year: i32,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "deserialize_month"))]
    month: u8,
    timezone: Option<TimezoneOffset>,
}

impl GYearMonth {
    /// Constructs a timezone-free year/month; returns `None` for months outside
    /// `1..=12`. All `i32` years are accepted, including zero.
    pub const fn new(year: i32, month: u8) -> Option<Self> {
        if month >= 1 && month <= 12 {
            Some(Self {
                year,
                month,
                timezone: None,
            })
        } else {
            None
        }
    }

    /// Returns the signed year, including zero, without timezone adjustment.
    pub const fn year(self) -> i32 {
        self.year
    }

    /// Returns the month in `1..=12`, without timezone adjustment.
    pub const fn month(self) -> u8 {
        self.month
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

impl fmt::Display for GYearMonth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.year < 0 {
            write!(f, "-{:04}", self.year.unsigned_abs())?;
        } else {
            write!(f, "{:04}", self.year)?;
        }
        write!(f, "-{:02}", self.month)?;
        if let Some(offset) = self.timezone {
            offset.fmt(f)?;
        }
        Ok(())
    }
}

#[cfg(feature = "serde")]
fn deserialize_month<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<u8, D::Error> {
    let month = <u8 as serde::Deserialize>::deserialize(deserializer)?;
    GYearMonth::new(0, month)
        .map(GYearMonth::month)
        .ok_or_else(|| serde::de::Error::custom("XSD month is outside 1..=12"))
}
