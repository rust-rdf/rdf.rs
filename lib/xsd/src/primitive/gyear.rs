// This is free and unencumbered software released into the public domain.

use crate::TimezoneOffset;
use core::fmt;

/// A bounded XSD 1.1 `gYear` with an optional validated timezone.
///
/// Stores any `i32` year (`-2147483648..=2147483647`), including zero as in
/// XSD 1.1. Negative years retain their signed value without an era adjustment.
/// Available without allocation or date/time features. Formatting writes at
/// least four year digits, a minus sign for negative years, and the optional
/// timezone (`Z` for UTC). Equality, ordering, and hashing are structural;
/// an absent timezone differs from explicit UTC.
///
/// This replaces the raw `i32` alias. Use [`Self::new`] for construction and
/// [`Self::year`] for the field. All `i32` values are valid years, so construction
/// is infallible; arbitrarily large XSD years cannot be represented.
///
/// With `serde`, the representation is a struct with `year` and `timezone`
/// (optional signed minutes), replacing the former integer. Decoding enforces
/// the `i32` year range and validates offsets. The enclosing `PrimitiveValue`
/// enum retains its `GYear` tag; consumers of old payloads must migrate.
///
/// ```
/// let year = xsd::primitive::GYear::new(-1);
/// assert_eq!(year.to_string(), "-0001");
/// assert_eq!(xsd::PrimitiveValue::GYear(year).to_string(), "-0001");
/// assert_eq!(xsd::primitive::GYear::new(0).to_string(), "0000");
/// ```
///
/// See: <https://www.w3.org/TR/xmlschema-2/#gYear>
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GYear {
    year: i32,
    timezone: Option<TimezoneOffset>,
}

impl GYear {
    /// Constructs a timezone-free year. All `i32` values are accepted, including zero.
    pub const fn new(year: i32) -> Self {
        Self {
            year,
            timezone: None,
        }
    }

    /// Returns the signed year, including zero, without timezone adjustment.
    pub const fn year(self) -> i32 {
        self.year
    }

    /// Sets or removes the timezone without changing the year.
    pub const fn with_timezone(mut self, timezone: Option<TimezoneOffset>) -> Self {
        self.timezone = timezone;
        self
    }

    /// Returns the timezone, distinguishing absence from explicit UTC.
    pub const fn timezone(self) -> Option<TimezoneOffset> {
        self.timezone
    }
}

impl fmt::Display for GYear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.year < 0 {
            write!(f, "-{:04}", self.year.unsigned_abs())?;
        } else {
            write!(f, "{:04}", self.year)?;
        }
        if let Some(offset) = self.timezone {
            offset.fmt(f)?;
        }
        Ok(())
    }
}
