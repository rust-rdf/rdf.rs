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
    /// This replaces the Jiff alias. Conversion to `jiff::civil::Date` through
    /// `TryFrom` rejects any present offset, including UTC, with
    /// [`crate::TimezoneLossError`]. Use [`Self::civil`] to explicitly discard
    /// the offset. With `serde`, the new representation is a struct with `civil`
    /// (Jiff's date string) and `timezone` (optional signed minutes). This replaces
    /// the former bare civil string; offset bounds are validated on decoding.
    /// The `civil` field must match Jiff's formatted date exactly; embedded times,
    /// offsets, and annotations are rejected rather than discarded. Explicit
    /// JSON/BSON conversion on [`crate::Value`] instead emits an XSD lexical
    /// string, retaining the date and timezone but omitting the datatype tag.
    /// Parse that string with [`crate::DATE`] to recover the represented value;
    /// neither encoding recovers the original spelling of a parsed literal.
    ///
    /// With `borsh`, the new type-local version-1 encoding is: version byte `1`,
    /// little-endian `i16` year, `i8` month, `i8` day, then Borsh
    /// `Option<TimezoneOffset>` (tag `0` for absent, or `1` and little-endian `i16`
    /// minutes). This is 6 or 8 bytes, with no datatype tag. There was no previous
    /// Borsh encoding for this type. Decoding rejects unknown versions, invalid
    /// calendar fields, invalid option tags, and out-of-range offsets.
    ///
    /// ```
    /// use xsd::{primitive::Date, TimezoneOffset};
    /// let date = Date::new(-1, 2, 28).unwrap().with_timezone(Some(TimezoneOffset::UTC));
    /// assert_eq!(date.to_string(), "-0001-02-28Z");
    /// ```
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct Date {
        #[cfg_attr(feature = "serde", serde(deserialize_with = "deserialize_civil"))]
        civil: jiff::civil::Date,
        timezone: Option<TimezoneOffset>,
    }

    #[cfg(feature = "serde")]
    fn deserialize_civil<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<jiff::civil::Date, D::Error> {
        use alloc::string::{String, ToString};
        let input = <String as serde::Deserialize>::deserialize(deserializer)?;
        let civil = input
            .parse::<jiff::civil::Date>()
            .map_err(serde::de::Error::custom)?;
        if civil.to_string() != input {
            return Err(serde::de::Error::custom(
                "expected a canonical Jiff civil date without time, timezone, or annotations",
            ));
        }
        Ok(civil)
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

    impl TryFrom<Date> for jiff::civil::Date {
        type Error = crate::TimezoneLossError;

        /// Converts only timezone-free dates; even explicit UTC would be lost.
        /// Calendar fields are returned without timezone adjustment.
        fn try_from(date: Date) -> Result<Self, Self::Error> {
            if date.timezone.is_some() {
                Err(crate::TimezoneLossError)
            } else {
                Ok(date.civil)
            }
        }
    }

    #[cfg(feature = "borsh")]
    impl borsh::BorshSerialize for Date {
        fn serialize<W: borsh::io::Write>(&self, writer: &mut W) -> Result<(), borsh::io::Error> {
            borsh::BorshSerialize::serialize(
                &(1u8, self.year(), self.month(), self.day(), self.timezone),
                writer,
            )
        }
    }

    #[cfg(feature = "borsh")]
    impl borsh::BorshDeserialize for Date {
        fn deserialize_reader<R: borsh::io::Read>(
            reader: &mut R,
        ) -> Result<Self, borsh::io::Error> {
            use borsh::io::{Error, ErrorKind};
            if u8::deserialize_reader(reader)? != 1 {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "unsupported XSD date encoding version",
                ));
            }
            let (year, month, day, timezone) =
                <(i16, i8, i8, Option<TimezoneOffset>)>::deserialize_reader(reader)?;
            Self::new(year, month, day)
                .map(|date| date.with_timezone(timezone))
                .map_err(|_| Error::new(ErrorKind::InvalidData, "invalid XSD date calendar fields"))
        }
    }
}
