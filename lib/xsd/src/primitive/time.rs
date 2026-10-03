// This is free and unencumbered software released into the public domain.

#[cfg(feature = "jiff")]
pub use self::value::Time;

#[cfg(feature = "jiff")]
mod value {
    use crate::TimezoneOffset;
    use core::fmt;

    /// An XSD time with nanosecond precision and an optional timezone offset.
    ///
    /// Requires `jiff` (enabled by `datetime`). Equality, ordering, and hashing
    /// are structural, not XSD instant comparison; absent timezones differ from
    /// UTC. Formatting writes `hh:mm:ss`, a nonzero fraction without trailing
    /// zeros, and the optional offset (`Z` for UTC, signed `hh:mm` otherwise).
    /// Use [`crate::parse_time`] for XSD lexical validation.
    ///
    /// This replaces the Jiff civil-time alias. Conversion to `jiff::civil::Time`
    /// through `TryFrom` rejects any present offset, including UTC, with
    /// [`crate::TimezoneLossError`]. [`Self::civil`] explicitly discards any
    /// timezone. With `serde`, the encoding is now a struct with
    /// `civil` (Jiff's canonical time string) and `timezone` (optional signed
    /// minutes), replacing the former bare string. Decoding validates offset
    /// bounds and rejects noncanonical civil strings, including embedded offsets,
    /// dates, leap seconds, and annotations, rather than silently losing data.
    ///
    /// ```
    /// use xsd::{primitive::Time, TimezoneOffset};
    /// let time = Time::new(12, 34, 56, 1).unwrap().with_timezone(Some(TimezoneOffset::UTC));
    /// assert_eq!(time.to_string(), "12:34:56.000000001Z");
    /// ```
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct Time {
        #[cfg_attr(feature = "serde", serde(deserialize_with = "deserialize_civil"))]
        civil: jiff::civil::Time,
        timezone: Option<TimezoneOffset>,
    }

    #[cfg(feature = "serde")]
    fn deserialize_civil<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<jiff::civil::Time, D::Error> {
        use alloc::string::{String, ToString};
        let input = <String as serde::Deserialize>::deserialize(deserializer)?;
        let civil = input
            .parse::<jiff::civil::Time>()
            .map_err(serde::de::Error::custom)?;
        if civil.to_string() != input {
            return Err(serde::de::Error::custom(
                "expected a canonical Jiff civil time without date, timezone, or annotations",
            ));
        }
        Ok(civil)
    }

    impl Time {
        /// Constructs a timezone-free time. Returns a Jiff error unless hours
        /// are `0..=23`, minutes and seconds `0..=59`, and nanoseconds
        /// `0..=999_999_999`. Use the XSD parser for end-of-day notation.
        pub fn new(hour: i8, minute: i8, second: i8, nanosecond: i32) -> Result<Self, jiff::Error> {
            jiff::civil::Time::new(hour, minute, second, nanosecond).map(Self::from)
        }

        /// Sets or removes the timezone without changing the clock fields.
        pub const fn with_timezone(mut self, timezone: Option<TimezoneOffset>) -> Self {
            self.timezone = timezone;
            self
        }

        /// Returns the timezone, distinguishing absence from explicit UTC.
        pub const fn timezone(self) -> Option<TimezoneOffset> {
            self.timezone
        }

        /// Returns the civil fields, explicitly discarding any timezone.
        pub const fn civil(self) -> jiff::civil::Time {
            self.civil
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

    impl From<jiff::civil::Time> for Time {
        /// Wraps a civil time with no timezone.
        fn from(civil: jiff::civil::Time) -> Self {
            Self {
                civil,
                timezone: None,
            }
        }
    }

    impl fmt::Display for Time {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.civil.fmt(f)?;
            if let Some(offset) = self.timezone {
                offset.fmt(f)?;
            }
            Ok(())
        }
    }

    impl TryFrom<Time> for jiff::civil::Time {
        type Error = crate::TimezoneLossError;

        /// Converts only timezone-free times; even explicit UTC would be lost.
        /// Preserves nanoseconds without timezone adjustment or a reference date.
        fn try_from(time: Time) -> Result<Self, Self::Error> {
            if time.timezone.is_some() {
                Err(crate::TimezoneLossError)
            } else {
                Ok(time.civil)
            }
        }
    }
}
