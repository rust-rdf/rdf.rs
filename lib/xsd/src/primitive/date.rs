// This is free and unencumbered software released into the public domain.

#[cfg(feature = "jiff")]
pub use self::value::Date;

#[cfg(feature = "jiff")]
mod value {
    use crate::TimezoneOffset;
    use core::fmt;

    /// An XSD 1.1 date with an optional validated timezone offset.
    ///
    /// Requires `jiff` (enabled by `datetime`). Years range from `-9999` through
    /// `9999`, including year zero, using the proleptic Gregorian calendar.
    /// Equality, ordering, and hashing are structural, not XSD instant comparison:
    /// an absent timezone differs from UTC. Formatting uses XSD year spelling
    /// and retains the offset, normalizing explicit zero offsets to `Z`.
    /// Use [`crate::parse_date`] to parse XSD lexical forms.
    ///
    /// This replaces the Jiff alias. Use [`Self::civil`] to explicitly discard
    /// the offset. With `serde`, the new representation is a struct with `civil`
    /// (Jiff's date string) and `timezone` (optional signed minutes). This replaces
    /// the former bare civil string; offset bounds are validated on decoding.
    ///
    /// ```
    /// use xsd::{primitive::Date, TimezoneOffset};
    /// let date = Date::new(-1, 2, 28).unwrap().with_timezone(Some(TimezoneOffset::UTC));
    /// assert_eq!(date.to_string(), "-0001-02-28Z");
    /// ```
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct Date {
        civil: jiff::civil::Date,
        timezone: Option<TimezoneOffset>,
    }

    impl Date {
        /// Constructs a timezone-free date, rejecting invalid fields or years
        /// outside `-9999..=9999` with a Jiff error.
        pub fn new(year: i16, month: i8, day: i8) -> Result<Self, jiff::Error> {
            jiff::civil::Date::new(year, month, day).map(Self::from)
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

        /// Returns the civil fields, explicitly discarding any timezone.
        pub const fn civil(self) -> jiff::civil::Date {
            self.civil
        }

        /// Returns the signed year, including year zero.
        pub fn year(self) -> i16 {
            self.civil.year()
        }

        /// Returns the month in `1..=12`.
        pub fn month(self) -> i8 {
            self.civil.month()
        }

        /// Returns the day of the month in `1..=31`.
        pub fn day(self) -> i8 {
            self.civil.day()
        }
    }

    impl From<jiff::civil::Date> for Date {
        /// Wraps a civil date with no timezone.
        fn from(civil: jiff::civil::Date) -> Self {
            Self {
                civil,
                timezone: None,
            }
        }
    }

    impl fmt::Display for Date {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let year = self.year();
            if year < 0 {
                write!(f, "-{:04}", year.unsigned_abs())?;
            } else {
                write!(f, "{year:04}")?;
            }
            write!(f, "-{:02}-{:02}", self.month(), self.day())?;
            if let Some(offset) = self.timezone {
                offset.fmt(f)?;
            }
            Ok(())
        }
    }
}
