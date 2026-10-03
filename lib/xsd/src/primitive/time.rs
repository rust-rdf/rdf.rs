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
    /// Explicit JSON/BSON conversion on [`crate::Value`] instead emits an XSD
    /// lexical string with the clock, nanoseconds, and optional offset, omitting
    /// the datatype tag. Parse that string with [`crate::TIME`] to recover the
    /// represented value. Neither encoding preserves original lexical spelling,
    /// such as hour 24, fractional trailing zeros, or the sign of a zero offset.
    ///
    /// With `borsh`, the new type-local version-1 encoding is: version byte `1`,
    /// `i8` hour, minute, and second, little-endian `i32` nanoseconds, then Borsh
    /// `Option<TimezoneOffset>` (tag `0` for absent, or `1` and little-endian `i16`
    /// minutes). This is 9 or 11 bytes, with no datatype tag. There was no previous
    /// Borsh encoding for this type. Decoding rejects unknown versions, invalid
    /// clock fields (including hour 24 and leap seconds), option tags, and offsets.
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

    #[cfg(feature = "borsh")]
    impl borsh::BorshSerialize for Time {
        fn serialize<W: borsh::io::Write>(&self, writer: &mut W) -> Result<(), borsh::io::Error> {
            borsh::BorshSerialize::serialize(
                &(
                    1u8,
                    self.hour(),
                    self.minute(),
                    self.second(),
                    self.subsec_nanosecond(),
                    self.timezone,
                ),
                writer,
            )
        }
    }

    #[cfg(feature = "borsh")]
    impl borsh::BorshDeserialize for Time {
        fn deserialize_reader<R: borsh::io::Read>(
            reader: &mut R,
        ) -> Result<Self, borsh::io::Error> {
            use borsh::io::{Error, ErrorKind};
            if u8::deserialize_reader(reader)? != 1 {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "unsupported XSD time encoding version",
                ));
            }
            let (hour, minute, second, nanosecond, timezone) =
                <(i8, i8, i8, i32, Option<TimezoneOffset>)>::deserialize_reader(reader)?;
            Self::new(hour, minute, second, nanosecond)
                .map(|time| time.with_timezone(timezone))
                .map_err(|_| Error::new(ErrorKind::InvalidData, "invalid XSD time clock fields"))
        }
    }
}
