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
/// ```
/// let offset = xsd::TimezoneOffset::from_minutes(330).unwrap();
/// assert_eq!(offset.to_string(), "+05:30");
/// assert_ne!(None, Some(xsd::TimezoneOffset::UTC));
/// assert!(xsd::TimezoneOffset::from_minutes(841).is_none());
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TimezoneOffset(i16);

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
