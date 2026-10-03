// This is free and unencumbered software released into the public domain.

#[cfg(feature = "jiff")]
pub use self::value::DateTime;

#[cfg(feature = "jiff")]
mod value {
    use crate::{
        TimezoneOffset,
        primitive::{Date, Time},
    };
    use core::fmt;

    /// An XSD 1.1 date-time with an optional validated timezone offset.
    ///
    /// Requires `jiff` (enabled by `datetime`). Supports proleptic Gregorian years
    /// `-9999..=9999`, including zero, and nanosecond precision. Equality, ordering,
    /// and hashing are structural, not instant comparisons: absent timezones differ
    /// from UTC. Formatting uses XSD year spelling and preserves the offset,
    /// normalizing explicit zero offsets to `Z` and omitting fractional trailing zeros.
    /// Use [`crate::parse_datetime`] for XSD lexical validation.
    ///
    /// This replaces the Jiff alias. Conversion to `jiff::civil::DateTime` through
    /// `TryFrom` rejects any present timezone, including UTC, with
    /// [`crate::TimezoneLossError`]. [`Self::civil`] explicitly discards the offset.
    /// With `serde`, the encoding is a struct with `civil` (Jiff's canonical
    /// date-time string) and `timezone` (optional signed minutes), replacing the
    /// former bare string. Decoding validates offset bounds and requires exactly
    /// the canonical civil spelling, rejecting embedded offsets and annotations.
    ///
    /// ```
    /// use xsd::{primitive::DateTime, TimezoneOffset};
    /// let value = DateTime::new(-1, 2, 28, 12, 34, 56, 1).unwrap()
    ///     .with_timezone(Some(TimezoneOffset::UTC));
    /// assert_eq!(value.to_string(), "-0001-02-28T12:34:56.000000001Z");
    /// ```
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct DateTime {
        #[cfg_attr(feature = "serde", serde(deserialize_with = "deserialize_civil"))]
        civil: jiff::civil::DateTime,
        timezone: Option<TimezoneOffset>,
    }

    #[cfg(feature = "serde")]
    fn deserialize_civil<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<jiff::civil::DateTime, D::Error> {
        use alloc::string::{String, ToString};
        let input = <String as serde::Deserialize>::deserialize(deserializer)?;
        let civil = input
            .parse::<jiff::civil::DateTime>()
            .map_err(serde::de::Error::custom)?;
        if civil.to_string() != input {
            return Err(serde::de::Error::custom(
                "expected a canonical Jiff civil date-time without timezone or annotations",
            ));
        }
        Ok(civil)
    }

    impl DateTime {
        /// Constructs a timezone-free date-time. Returns a Jiff error for invalid
        /// calendar or clock fields, years outside `-9999..=9999`, or nanoseconds
        /// outside `0..=999_999_999`. Hour 24 and leap seconds are not stored.
        pub fn new(
            year: i16,
            month: i8,
            day: i8,
            hour: i8,
            minute: i8,
            second: i8,
            nanosecond: i32,
        ) -> Result<Self, jiff::Error> {
            jiff::civil::DateTime::new(year, month, day, hour, minute, second, nanosecond)
                .map(Self::from)
        }

        /// Sets or removes the timezone without changing the civil fields.
        pub const fn with_timezone(mut self, timezone: Option<TimezoneOffset>) -> Self {
            self.timezone = timezone;
            self
        }

        /// Returns the timezone, distinguishing absence from explicit UTC.
        pub const fn timezone(self) -> Option<TimezoneOffset> {
            self.timezone
        }

        /// Returns the civil fields, explicitly discarding any timezone.
        pub const fn civil(self) -> jiff::civil::DateTime {
            self.civil
        }

        /// Returns the date component, retaining the optional timezone.
        pub fn date(self) -> Date {
            Date::from(self.civil.date()).with_timezone(self.timezone)
        }

        /// Returns the time component, retaining the optional timezone.
        pub fn time(self) -> Time {
            Time::from(self.civil.time()).with_timezone(self.timezone)
        }

        /// Returns the signed year, including zero.
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
        /// Returns the hour in `0..=23`.
        pub fn hour(self) -> i8 {
            self.civil.hour()
        }
        /// Returns the minute in `0..=59`.
        pub fn minute(self) -> i8 {
            self.civil.minute()
        }
        /// Returns the second in `0..=59`.
        pub fn second(self) -> i8 {
            self.civil.second()
        }
        /// Returns the fractional second in nanoseconds (`0..=999_999_999`).
        pub fn subsec_nanosecond(self) -> i32 {
            self.civil.subsec_nanosecond()
        }
    }

    impl From<jiff::civil::DateTime> for DateTime {
        /// Wraps civil fields with no timezone.
        fn from(civil: jiff::civil::DateTime) -> Self {
            Self {
                civil,
                timezone: None,
            }
        }
    }

    impl fmt::Display for DateTime {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}T{}", Date::from(self.civil.date()), self.civil.time())?;
            if let Some(offset) = self.timezone {
                offset.fmt(f)?;
            }
            Ok(())
        }
    }

    impl TryFrom<DateTime> for jiff::civil::DateTime {
        type Error = crate::TimezoneLossError;

        /// Converts only timezone-free values; even explicit UTC would be lost.
        /// Calendar fields and nanoseconds are returned without timezone adjustment.
        fn try_from(value: DateTime) -> Result<Self, Self::Error> {
            if value.timezone.is_some() {
                Err(crate::TimezoneLossError)
            } else {
                Ok(value.civil)
            }
        }
    }
}
