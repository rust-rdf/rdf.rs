// This is free and unencumbered software released into the public domain.

use crate::{
    DecimalValue, ParseBooleanError, ParseDecimalError, ParseDoubleError, ParseError,
    ParseFloatError, ParseIntegerError, Type, Value,
    derived::{Byte, Int, Integer, Long, Short},
    primitive::{Boolean, Decimal, Double, Float},
};

#[cfg(feature = "alloc")]
use core::convert::Infallible;

#[cfg(feature = "jiff")]
use crate::{
    ParseDateTimeError, ParseDurationError, ParseTemporalError,
    primitive::{Date, DateTime, Duration, Time},
};

/// Parses an input string containing an XSD literal.
///
/// # Errors
///
/// Returns [`ParseError::UnsupportedDatatype`] when no parser is available for
/// the datatype, including unknown datatypes. String parsing requires `alloc`;
/// date, time, dateTime, and duration parsing require `jiff` (enabled by `datetime`).
/// Disabled capabilities return the same error. Integer-family parser failures
/// return [`ParseError::InvalidInteger`]; decimal failures return
/// [`ParseError::InvalidDecimal`]; float/double failures return
/// [`ParseError::InvalidFloat`]. These numeric errors retain the requested datatype
/// and underlying cause. Boolean failures return [`ParseError::InvalidBoolean`].
#[cfg_attr(
    feature = "jiff",
    doc = "Temporal failures return [`ParseError::InvalidTemporal`], retaining the requested datatype and original Jiff error."
)]
///
/// ```
/// let error = xsd::parse("AQI=", xsd::BASE64_BINARY).unwrap_err();
/// assert!(matches!(
///     error,
///     xsd::ParseError::UnsupportedDatatype(datatype) if datatype == xsd::BASE64_BINARY
/// ));
/// ```
pub fn parse(input: impl AsRef<str>, datatype: impl Into<Type>) -> Result<Value, ParseError> {
    use crate::{DecimalType as D, PrimitiveType as P, Type::*};
    match datatype.into() {
        datatype @ (Decimal(D::Decimal) | Primitive(P::Decimal)) => {
            parse_decimal(input).map_err(|source| ParseError::InvalidDecimal { datatype, source })
        },
        Decimal(D::Integer) => parse_integer(input).map_err(|source| ParseError::InvalidInteger {
            datatype: D::Integer,
            source,
        }),
        Decimal(D::Long) => parse_long(input).map_err(|source| ParseError::InvalidInteger {
            datatype: D::Long,
            source,
        }),
        Decimal(D::Int) => parse_int(input).map_err(|source| ParseError::InvalidInteger {
            datatype: D::Int,
            source,
        }),
        Decimal(D::Short) => parse_short(input).map_err(|source| ParseError::InvalidInteger {
            datatype: D::Short,
            source,
        }),
        Decimal(D::Byte) => parse_byte(input).map_err(|source| ParseError::InvalidInteger {
            datatype: D::Byte,
            source,
        }),
        #[cfg(feature = "alloc")]
        Primitive(P::String) => parse_string(input).map_err(|_| ParseError::InvalidLiteral),
        Primitive(P::Boolean) => parse_boolean(input),
        Primitive(P::Float) => parse_float(input).map_err(|source| ParseError::InvalidFloat {
            datatype: P::Float,
            source,
        }),
        Primitive(P::Double) => parse_double(input).map_err(|source| ParseError::InvalidFloat {
            datatype: P::Double,
            source,
        }),
        #[cfg(feature = "jiff")]
        Primitive(P::Duration) => {
            parse_duration(input).map_err(|source| ParseError::InvalidTemporal {
                datatype: P::Duration,
                source: ParseTemporalError(source),
            })
        },
        #[cfg(feature = "jiff")]
        Primitive(P::DateTime) => {
            parse_datetime(input).map_err(|source| ParseError::InvalidTemporal {
                datatype: P::DateTime,
                source: ParseTemporalError(source),
            })
        },
        #[cfg(feature = "jiff")]
        Primitive(P::Time) => parse_time(input).map_err(|source| ParseError::InvalidTemporal {
            datatype: P::Time,
            source: ParseTemporalError(source),
        }),
        #[cfg(feature = "jiff")]
        Primitive(P::Date) => parse_date(input).map_err(|source| ParseError::InvalidTemporal {
            datatype: P::Date,
            source: ParseTemporalError(source),
        }),
        datatype => Err(ParseError::UnsupportedDatatype(datatype)),
    }
}

/// Parses an input string containing an `xsd:decimal` literal.
///
/// Accepts an optional sign and ASCII digits with at most one decimal point;
/// at least one digit is required. Exponents and whitespace are rejected.
/// XML Schema whitespace preprocessing is the caller's responsibility.
/// The numeric backend determines representable range and precision and may
/// round values with excess precision.
///
/// # Errors
///
/// Returns [`ParseDecimalError::InvalidLexical`] for non-XSD spellings, or
/// [`ParseDecimalError::Backend`] when numeric conversion fails.
pub fn parse_decimal(input: impl AsRef<str>) -> Result<Value, ParseDecimalError> {
    let input = input.as_ref();
    let unsigned = input.strip_prefix(['+', '-']).unwrap_or(input);
    if !is_decimal_significand(unsigned) {
        return Err(ParseDecimalError::InvalidLexical);
    }
    input
        .parse::<Decimal>()
        .map(Value::from)
        .map_err(Into::into)
}

/// Parses an input string containing an `xsd:integer` literal.
///
/// Accepts an optional `+` or `-` followed by one or more ASCII digits.
/// Leading zeros and signed zero are accepted; whitespace, decimal points,
/// exponents, and digit separators are rejected. XML Schema whitespace
/// preprocessing must be performed separately by the caller. The backend
/// enforces this grammar and currently limits values to the `i128` range.
/// Original lexical spelling is not retained.
///
/// # Errors
///
/// Returns [`ParseIntegerError`] for malformed input or an out-of-range value.
/// Its [`kind()`](core::num::ParseIntError::kind) identifies the failure.
///
/// ```
/// assert_eq!(xsd::parse_integer("+001").unwrap().to_string(), "1");
/// assert!(xsd::parse_integer("1.0").is_err());
/// ```
pub fn parse_integer(input: impl AsRef<str>) -> Result<Value, ParseIntegerError> {
    input
        .as_ref()
        .parse::<Integer>()
        .map(DecimalValue::from)
        .map(Value::from)
}

/// Parses an input string containing an `xsd:long` literal.
///
/// Uses the lexical rules of [`parse_integer`] and the signed 64-bit range.
///
/// # Errors
///
/// Returns [`ParseIntegerError`] for malformed input or overflow.
pub fn parse_long(input: impl AsRef<str>) -> Result<Value, ParseIntegerError> {
    input
        .as_ref()
        .parse::<Long>()
        .map(DecimalValue::from)
        .map(Value::from)
}

/// Parses an input string containing an `xsd:int` literal.
///
/// Uses the lexical rules of [`parse_integer`] and the signed 32-bit range.
///
/// # Errors
///
/// Returns [`ParseIntegerError`] for malformed input or overflow.
pub fn parse_int(input: impl AsRef<str>) -> Result<Value, ParseIntegerError> {
    input
        .as_ref()
        .parse::<Int>()
        .map(DecimalValue::from)
        .map(Value::from)
}

/// Parses an input string containing an `xsd:short` literal.
///
/// Uses the lexical rules of [`parse_integer`] and the signed 16-bit range.
///
/// # Errors
///
/// Returns [`ParseIntegerError`] for malformed input or overflow.
pub fn parse_short(input: impl AsRef<str>) -> Result<Value, ParseIntegerError> {
    input
        .as_ref()
        .parse::<Short>()
        .map(DecimalValue::from)
        .map(Value::from)
}

/// Parses an input string containing an `xsd:byte` literal.
///
/// Uses the lexical rules of [`parse_integer`] and the signed 8-bit range.
///
/// # Errors
///
/// Returns [`ParseIntegerError`] for malformed input or overflow.
pub fn parse_byte(input: impl AsRef<str>) -> Result<Value, ParseIntegerError> {
    input
        .as_ref()
        .parse::<Byte>()
        .map(DecimalValue::from)
        .map(Value::from)
}

/// Parses an input string containing an `xsd:string` literal.
///
/// With `alloc` enabled, copies the input verbatim into an owned string and
/// returns an infallible result. Without `alloc`, the error type is [`ParseError`]
/// and parsing returns [`ParseError::UnsupportedDatatype`] for [`crate::STRING`].
#[cfg(feature = "alloc")]
pub fn parse_string(input: impl AsRef<str>) -> Result<Value, Infallible> {
    use crate::primitive::String;
    input.as_ref().parse::<String>().map(Value::from)
}

/// Parses an input string containing an `xsd:string` literal.
///
/// String parsing requires `alloc`. With that feature enabled, this function
/// copies the input verbatim into an owned string and its error type is
/// [`core::convert::Infallible`].
///
/// # Errors
///
/// Without `alloc`, always returns [`ParseError::UnsupportedDatatype`] for
/// [`crate::STRING`], including for empty or static input.
///
/// ```
/// assert!(matches!(
///     xsd::parse_string("hello"),
///     Err(xsd::ParseError::UnsupportedDatatype(datatype)) if datatype == xsd::STRING
/// ));
/// ```
#[cfg(not(feature = "alloc"))]
pub fn parse_string(_input: impl AsRef<str>) -> Result<Value, ParseError> {
    Err(ParseError::UnsupportedDatatype(crate::STRING))
}

/// Parses an input string containing an `xsd:boolean` literal.
///
/// Accepts the exact lexical forms `true`, `false`, `1`, and `0`.
///
/// # Errors
///
/// Returns [`ParseError::InvalidBoolean`] when the literal cannot be parsed.
pub fn parse_boolean(input: impl AsRef<str>) -> Result<Value, ParseBooleanError> {
    input
        .as_ref()
        .parse::<Boolean>()
        .map(Value::from)
        .map_err(|_| ParseError::InvalidBoolean)
}

/// Parses an input string containing an `xsd:float` literal.
///
/// Validates the XSD 1.1 lexical grammar before converting to binary32. Accepts
/// a signed decimal significand with an optional `e`/`E` exponent, or exactly
/// `INF`, `+INF`, `-INF`, or `NaN`. At least one significand digit and, when
/// present, one exponent digit are required. Input is not trimmed: callers
/// performing XML Schema whitespace preprocessing must do so separately.
/// The backend rounds to binary32; overflow and underflow can yield infinity
/// and zero, respectively.
///
/// # Errors
///
/// Returns [`ParseFloatError::InvalidLexical`] for invalid lexical forms, and
/// [`ParseFloatError::Backend`] for backend conversion failures.
///
/// ```
/// assert!(xsd::parse_float("+.5E2").is_ok());
/// assert!(xsd::parse_float("+INF").is_ok());
/// assert!(matches!(xsd::parse_float("infinity"),
///     Err(xsd::ParseFloatError::InvalidLexical)));
/// ```
pub fn parse_float(input: impl AsRef<str>) -> Result<Value, ParseFloatError> {
    let input = input.as_ref();
    if !is_xsd_floating_point(input) {
        return Err(ParseFloatError::InvalidLexical);
    }
    input.parse::<Float>().map(Value::from).map_err(Into::into)
}

fn is_xsd_floating_point(input: &str) -> bool {
    if matches!(input, "INF" | "+INF" | "-INF" | "NaN") {
        return true;
    }
    let unsigned = input.strip_prefix(['+', '-']).unwrap_or(input);
    let significand = if let Some((significand, exponent)) = unsigned.split_once(['e', 'E']) {
        let exponent = exponent.strip_prefix(['+', '-']).unwrap_or(exponent);
        if exponent.is_empty() || !exponent.bytes().all(|byte| byte.is_ascii_digit()) {
            return false;
        }
        significand
    } else {
        unsigned
    };
    is_decimal_significand(significand)
}

fn is_decimal_significand(significand: &str) -> bool {
    if let Some((integer, fraction)) = significand.split_once('.') {
        !(integer.is_empty() && fraction.is_empty())
            && integer.bytes().all(|byte| byte.is_ascii_digit())
            && fraction.bytes().all(|byte| byte.is_ascii_digit())
    } else {
        !significand.is_empty() && significand.bytes().all(|byte| byte.is_ascii_digit())
    }
}

/// Parses an input string containing an `xsd:double` literal.
///
/// Uses the same XSD 1.1 lexical grammar and untrimmed-input policy as
/// [`parse_float`], but converts to binary64. Accepts signed decimal
/// significands with optional `e`/`E` exponents and exactly `INF`, `+INF`,
/// `-INF`, or `NaN`. The backend rounds to binary64; overflow and underflow
/// can yield infinity and zero, respectively.
///
/// # Errors
///
/// Returns [`ParseFloatError::InvalidLexical`] for invalid lexical forms, and
/// [`ParseFloatError::Backend`] for backend conversion failures.
/// [`ParseDoubleError`] aliases [`ParseFloatError`].
///
/// ```
/// assert!(xsd::parse_double("+.5E2").is_ok());
/// assert!(xsd::parse_double("+INF").is_ok());
/// assert!(matches!(xsd::parse_double("nan"),
///     Err(xsd::ParseDoubleError::InvalidLexical)));
/// ```
pub fn parse_double(input: impl AsRef<str>) -> Result<Value, ParseDoubleError> {
    let input = input.as_ref();
    if !is_xsd_floating_point(input) {
        return Err(ParseFloatError::InvalidLexical);
    }
    input.parse::<Double>().map(Value::from).map_err(Into::into)
}

/// Parses an input string containing an `xsd:duration` literal.
///
/// Requires `jiff` (enabled by `datetime`). Duration designators must be
/// uppercase ASCII; friendly and clock-style backend spellings are rejected.
/// Only a leading minus sign is allowed; a leading plus sign is rejected.
/// Only seconds may have a fraction, with ASCII digits on both sides of a
/// period separator; commas and fractional hours or minutes are rejected.
/// Fractional seconds support at most nine digits (nanosecond precision).
/// Longer fractions are rejected, including excess zeros, never rounded.
///
/// # Errors
///
/// Returns an error for invalid lexical forms or values outside the backend's
/// signed, fixed-length duration representation. Calendar years and months are
/// not supported. Fractions longer than nine digits return an explicit precision
/// error. Overflow returns an error rather than panicking.
///
/// ```
/// let value = xsd::parse_duration("-PT1.000000001S").unwrap();
/// assert_eq!(value.to_string(), "-PT1.000000001S");
/// assert!(xsd::parse_duration("PT0.0000000001S").is_err());
/// ```
#[cfg(feature = "jiff")]
pub fn parse_duration(input: impl AsRef<str>) -> Result<Value, ParseDurationError> {
    let input = input.as_ref();
    if input.starts_with('+') {
        return Err(jiff::Error::from_args(format_args!(
            "xsd:duration must not have a leading plus sign"
        )));
    }
    let unsigned = input.strip_prefix('-').unwrap_or(input);
    if input.contains(',') {
        return Err(jiff::Error::from_args(format_args!(
            "xsd:duration fractions require a period separator"
        )));
    }
    if !unsigned.starts_with('P') || input.bytes().any(|byte| byte.is_ascii_lowercase()) {
        return Err(jiff::Error::from_args(format_args!(
            "xsd:duration requires uppercase duration designators"
        )));
    }
    if let Some((whole, suffix)) = input.split_once('.') {
        let valid_fraction = suffix.strip_suffix('S').is_some_and(|fraction| {
            !fraction.is_empty() && fraction.bytes().all(|byte| byte.is_ascii_digit())
        });
        if !whole.as_bytes().last().is_some_and(u8::is_ascii_digit) || !valid_fraction {
            return Err(jiff::Error::from_args(format_args!(
                "xsd:duration permits fractions only on seconds, with digits before and after the period"
            )));
        }
        if suffix.len() - 1 > 9 {
            return Err(jiff::Error::from_args(format_args!(
                "xsd:duration fractional seconds exceed the supported nanosecond precision (9 digits)"
            )));
        }
    }
    input.parse::<Duration>().map(Value::from)
}

/// Parses an input string containing an `xsd:dateTime` literal.
///
/// Requires `jiff` (enabled by `datetime`). The date and time components must
/// be separated by uppercase ASCII `T`; lowercase `t` and space are rejected.
/// Year, month, and day must be separated by hyphens; compact dates such as
/// `20261231T12:34:56` are rejected.
/// Years must not have a leading plus sign.
/// Years longer than four digits must not begin with zero (excluding the sign).
/// Four-digit negative years (`-0001` through `-9999`) are accepted. The signed
/// year is preserved in the returned value and its XSD formatting without a
/// historical-era adjustment. Negative zero (`-0000`) is rejected.
/// Supported years are `-9999` through `9999`, including `0000` (the year before
/// `0001`, as in XSD 1.1). Dates use the proleptic Gregorian calendar: years
/// divisible by 4 are leap years except centuries not divisible by 400.
/// This rule also applies to zero and negative years. Invalid calendar dates
/// and years outside this bounded representation return errors.
/// The time must begin with two-digit hours, minutes, and seconds separated by
/// colons (`hh:mm:ss`); omitted clock fields are not filled in with zero.
/// Timezone-free `24:00:00`, optionally followed by a period and any number of
/// ASCII zero digits, is normalized to midnight on the next calendar day.
/// At least one digit is required after the period. Calendar arithmetic uses
/// the stored year numbering, including year zero. Hour-24 forms with timezone
/// suffixes are not yet supported.
/// For hours 00 through 23, fractional seconds require one to nine ASCII digits
/// (nanosecond precision). Longer fractions, including excess zero digits, are
/// rejected instead of rounded or truncated.
/// Fractional seconds use a period separator; a comma is rejected.
/// Seconds must be less than 60; leap seconds are rejected rather than clamped
/// to 59 by the underlying parser.
/// Numeric timezone offsets must use `+hh:mm` or `-hh:mm` and be between `-14:00`
/// and `+14:00`, inclusive. Abbreviated offsets and offset seconds are rejected.
/// The returned civil value currently does not retain accepted timezone offsets.
/// Bracketed timezone and calendar annotations (such as `[Europe/Paris]` and
/// `[u-ca=iso8601]`) are not XSD syntax and are rejected instead of discarded.
///
/// # Errors
///
/// Returns an error when the input cannot be parsed by the underlying
/// civil-dateTime parser, does not use the required uppercase `T` separator,
/// omits the required date-component hyphens, uses a leading plus sign on the year,
/// has a year longer than four digits beginning with zero,
/// omits the required `hh:mm:ss` clock fields, uses a comma to separate fractional
/// seconds, specifies a leap second, contains bracketed annotations, or has a
/// numeric timezone offset with invalid XSD syntax or a value outside the XSD range.
/// End-of-day normalization also returns an error if the next day exceeds the
/// supported year range (for example, `9999-12-31T24:00:00`).
/// Fractions beyond nanosecond precision return an explicit precision error.
///
/// ```
/// let value = xsd::parse_datetime("2026-12-31T12:34:56").unwrap();
/// assert_eq!(value.to_string(), "2026-12-31T12:34:56");
/// let value = xsd::parse_datetime("-2024-02-29T12:34:56.125").unwrap();
/// assert_eq!(value.to_string(), "-2024-02-29T12:34:56.125");
/// assert_eq!(
///     xsd::parse_datetime("2024-02-29T24:00:00").unwrap().to_string(),
///     "2024-03-01T00:00:00",
/// );
/// assert!(xsd::parse_datetime("2026-12-31 12:34:56").is_err());
/// assert!(xsd::parse_datetime("2026-12-31t12:34:56").is_err());
/// assert!(xsd::parse_datetime("20261231T12:34:56").is_err());
/// assert!(xsd::parse_datetime("+002026-12-31T12:34:56").is_err());
/// assert!(xsd::parse_datetime("-002026-12-31T12:34:56").is_err());
/// assert!(xsd::parse_datetime("2026-12-31T12:34").is_err());
/// assert!(xsd::parse_datetime("2026-12-31T12:34:56,125").is_err());
/// assert!(xsd::parse_datetime("2026-12-31T23:59:60").is_err());
/// assert!(xsd::parse_datetime("2026-12-31T12:34:56+14:00").is_ok());
/// assert!(xsd::parse_datetime("2026-12-31T12:34:56+14:01").is_err());
/// assert!(xsd::parse_datetime("2026-12-31T12:34:56+02").is_err());
/// assert!(xsd::parse_datetime("2026-12-31T12:34:56+0200").is_err());
/// assert!(xsd::parse_datetime("2026-12-31T12:34:56+02:00:00").is_err());
/// assert!(xsd::parse_datetime("2026-12-31T12:34:56[Europe/Paris]").is_err());
/// ```
#[cfg(feature = "jiff")]
pub fn parse_datetime(input: impl AsRef<str>) -> Result<Value, ParseDateTimeError> {
    let input = input.as_ref();
    if let Some((date, time)) = input.split_once('T')
        && is_timezone_free_end_of_day(time)
        && let Value::Primitive(crate::PrimitiveValue::Date(date)) = parse_date(date)?
    {
        let next = date.tomorrow()?;
        return DateTime::new(next.year(), next.month(), next.day(), 0, 0, 0, 0).map(Value::from);
    }
    if let Some((_, clock)) = input.split_once('T') {
        validate_nanosecond_precision(clock)?;
    }
    // Jiff requires six digits for negative years. Parse four-digit years by
    // magnitude and restore the sign without allocating. Gregorian leap-year
    // validity is identical for a year and its negation.
    let negative_year = input.starts_with('-') && input.as_bytes().get(5) == Some(&b'-');
    let civil_input = if negative_year { &input[1..] } else { input };
    let mut datetime = civil_input.parse::<DateTime>()?;
    if negative_year {
        if datetime.year() == 0 {
            return Err(jiff::Error::from_args(format_args!(
                "xsd:dateTime years must not be negative zero"
            )));
        }
        datetime = DateTime::new(
            -datetime.year(),
            datetime.month(),
            datetime.day(),
            datetime.hour(),
            datetime.minute(),
            datetime.second(),
            datetime.subsec_nanosecond(),
        )?;
    }
    // Jiff accepts T, t, or space here; XSD only permits uppercase T.
    let separator = input
        .bytes()
        .position(|byte| matches!(byte, b'T' | b't' | b' '));
    let Some(separator) = separator.filter(|&index| input.as_bytes()[index] == b'T') else {
        return Err(jiff::Error::from_args(format_args!(
            "xsd:dateTime literals require an uppercase T separator"
        )));
    };
    // Jiff validates field widths and ranges; XSD requires both date separators.
    let date = input[..separator]
        .strip_prefix(['+', '-'])
        .unwrap_or(&input[..separator]);
    if !date
        .split_once('-')
        .is_some_and(|(_, rest)| matches!(rest.as_bytes(), [_, _, b'-', _, _]))
    {
        return Err(jiff::Error::from_args(format_args!(
            "xsd:dateTime literals require hyphen-separated year, month, and day"
        )));
    }
    if input.starts_with('+') {
        return Err(jiff::Error::from_args(format_args!(
            "xsd:dateTime years must not have a leading plus sign"
        )));
    }
    if date
        .split_once('-')
        .is_some_and(|(year, _)| year.len() > 4 && year.starts_with('0'))
    {
        return Err(jiff::Error::from_args(format_args!(
            "xsd:dateTime years longer than four digits must not begin with zero"
        )));
    }
    // Jiff validates the digits and ranges; require all three clock fields.
    if !matches!(
        &input.as_bytes()[separator + 1..],
        [_, _, b':', _, _, b':', _, _, ..]
    ) {
        return Err(jiff::Error::from_args(format_args!(
            "xsd:dateTime literals require hours, minutes, and seconds (hh:mm:ss)"
        )));
    }
    if input.as_bytes().get(separator + 9) == Some(&b',') {
        return Err(jiff::Error::from_args(format_args!(
            "xsd:dateTime fractional seconds require a period separator"
        )));
    }
    // Jiff accepts leap seconds and clamps them to 59, losing the input value.
    if &input.as_bytes()[separator + 7..separator + 9] == b"60" {
        return Err(jiff::Error::from_args(format_args!(
            "xsd:dateTime seconds must be less than 60"
        )));
    }
    if jiff::fmt::temporal::DateTimeParser::new()
        .parse_pieces(civil_input)?
        .to_numeric_offset()
        .is_some_and(|offset| offset.seconds().unsigned_abs() > 14 * 60 * 60)
    {
        return Err(jiff::Error::from_args(format_args!(
            "xsd:dateTime timezone offsets must be between -14:00 and +14:00"
        )));
    }
    // Jiff accepts bracketed annotations that are outside the XSD lexical grammar.
    if input.as_bytes().contains(&b'[') {
        return Err(jiff::Error::from_args(format_args!(
            "xsd:dateTime literals must not contain bracketed annotations"
        )));
    }
    // Exclude the date's signs and hyphens when locating a numeric offset.
    let suffix = &input.as_bytes()[separator + 9..];
    if let Some(start) = suffix.iter().position(|byte| matches!(byte, b'+' | b'-'))
        && !matches!(&suffix[start..], [b'+' | b'-', _, _, b':', _, _])
    {
        return Err(jiff::Error::from_args(format_args!(
            "xsd:dateTime numeric timezone offsets require +hh:mm or -hh:mm"
        )));
    }
    Ok(Value::from(datetime))
}

/// Parses an input string containing an `xsd:time` literal.
///
/// Requires `jiff` (enabled by `datetime`). Inputs must begin with two-digit
/// hours, minutes, and seconds separated by colons (`hh:mm:ss`). Seconds may
/// include a fractional part; omitted clock fields are not filled in with zero.
/// Timezone-free `24:00:00` denotes midnight and is normalized to `00:00:00`.
/// It may have a period followed by one or more ASCII zero digits, with no
/// precision limit for this all-zero fraction. Other hour-24 forms, including
/// those with a timezone suffix, currently return an error.
/// For hours 00 through 23, a fractional part requires one to nine ASCII digits
/// (nanosecond precision). Longer fractions are rejected even when the excess
/// digits are zero; they are never rounded or truncated.
/// Fractional seconds use a period separator; a comma is rejected.
/// Seconds must be less than 60; leap seconds are rejected rather than clamped
/// to 59 by the underlying parser.
/// Numeric timezone offsets must use `+hh:mm` or `-hh:mm` and be between `-14:00`
/// and `+14:00`, inclusive. Abbreviated offsets and offset seconds are rejected.
/// The returned civil value currently does not retain accepted timezone offsets.
/// Bracketed timezone and calendar annotations (such as `[Europe/Paris]` and
/// `[u-ca=iso8601]`) are not XSD syntax and are rejected instead of discarded.
///
/// # Errors
///
/// Apart from the timezone-free end-of-day forms described above, returns an
/// error when the input cannot be parsed by the underlying civil-time
/// parser, does not begin with the required `hh:mm:ss` clock fields, uses a
/// comma to separate fractional seconds, specifies a leap second, contains
/// bracketed annotations, or has a numeric timezone offset with invalid XSD syntax
/// or a value outside the XSD range.
/// Fractions beyond nanosecond precision return an explicit precision error.
///
/// ```
/// assert!(xsd::parse_time("12:34").is_err());
/// assert!(xsd::parse_time("12:34:56,125").is_err());
/// assert!(xsd::parse_time("23:59:60").is_err());
/// assert!(xsd::parse_time("12:34:56[Europe/Paris]").is_err());
/// assert!(xsd::parse_time("12:34:56+14:00").is_ok());
/// assert!(xsd::parse_time("12:34:56+14:01").is_err());
/// assert!(xsd::parse_time("12:34:56+02").is_err());
/// assert!(xsd::parse_time("12:34:56+0200").is_err());
/// assert!(xsd::parse_time("12:34:56+02:00:00").is_err());
/// assert_eq!(xsd::parse_time("12:34:00").unwrap().to_string(), "12:34:00");
/// assert_eq!(xsd::parse_time("12:34:56.125").unwrap().to_string(), "12:34:56.125");
/// assert_eq!(xsd::parse_time("24:00:00.000").unwrap().to_string(), "00:00:00");
/// assert!(xsd::parse_time("24:00:00.001").is_err());
/// ```
#[cfg(feature = "jiff")]
pub fn parse_time(input: impl AsRef<str>) -> Result<Value, ParseDateTimeError> {
    let input = input.as_ref();
    // XSD permits end-of-day notation; Jiff's civil clock stops at hour 23.
    // Inspect the entire suffix so no nonzero fraction or annotation is lost.
    if is_timezone_free_end_of_day(input) {
        return Time::new(0, 0, 0, 0).map(Value::from);
    }
    validate_nanosecond_precision(input)?;
    let time = input.parse::<Time>()?;
    // Jiff validates the digits and ranges; require all three colon-separated fields.
    if !matches!(input.as_bytes(), [_, _, b':', _, _, b':', _, _, ..]) {
        return Err(jiff::Error::from_args(format_args!(
            "xsd:time literals require hours, minutes, and seconds (hh:mm:ss)"
        )));
    }
    if input.as_bytes().get(8) == Some(&b',') {
        return Err(jiff::Error::from_args(format_args!(
            "xsd:time fractional seconds require a period separator"
        )));
    }
    // Jiff accepts leap seconds and clamps them to 59, losing the input value.
    if &input.as_bytes()[6..8] == b"60" {
        return Err(jiff::Error::from_args(format_args!(
            "xsd:time seconds must be less than 60"
        )));
    }
    // Jiff accepts bracketed annotations that are outside the XSD lexical grammar.
    if input.as_bytes().contains(&b'[') {
        return Err(jiff::Error::from_args(format_args!(
            "xsd:time literals must not contain bracketed annotations"
        )));
    }
    // With clock fields validated and annotations excluded, a sign starts an offset.
    if let Some(start) = input.bytes().position(|byte| matches!(byte, b'+' | b'-')) {
        // Jiff validates the two hour digits. At 14 hours, all smaller units must be zero.
        let (hours, rest) = input[start + 1..].split_at(2);
        if hours > "14" || (hours == "14" && rest.bytes().any(|byte| matches!(byte, b'1'..=b'9'))) {
            return Err(jiff::Error::from_args(format_args!(
                "xsd:time timezone offsets must be between -14:00 and +14:00"
            )));
        }
        // Jiff validates the digits; XSD requires exactly two colon-separated fields.
        if !matches!(&input.as_bytes()[start..], [b'+' | b'-', _, _, b':', _, _]) {
            return Err(jiff::Error::from_args(format_args!(
                "xsd:time numeric timezone offsets require +hh:mm or -hh:mm"
            )));
        }
    }
    Ok(Value::from(time))
}

#[cfg(feature = "jiff")]
fn validate_nanosecond_precision(clock: &str) -> Result<(), jiff::Error> {
    if let Some(fraction) = clock.as_bytes().get(8..).and_then(|s| s.strip_prefix(b".")) {
        let digits = fraction
            .iter()
            .take_while(|byte| byte.is_ascii_digit())
            .count();
        if digits > 9 {
            return Err(jiff::Error::from_args(format_args!(
                "XSD fractional seconds exceed the supported nanosecond precision (9 digits)"
            )));
        }
    }
    Ok(())
}

#[cfg(feature = "jiff")]
fn is_timezone_free_end_of_day(input: &str) -> bool {
    input.strip_prefix("24:00:00").is_some_and(|suffix| {
        suffix.is_empty()
            || suffix.strip_prefix('.').is_some_and(|fraction| {
                !fraction.is_empty() && fraction.bytes().all(|byte| byte == b'0')
            })
    })
}

/// Parses an input string containing an `xsd:date` literal.
///
/// Requires `jiff` (enabled by `datetime`). Inputs containing a time component
/// are rejected instead of being truncated to their date component.
/// Year, month, and day must be separated by hyphens; compact dates such as
/// `20261231` are rejected.
/// Years must not have a leading plus sign.
/// Years longer than four digits must not begin with zero (excluding the sign).
/// Four-digit negative years (`-0001` through `-9999`) are accepted for dates
/// without a timezone. The signed year is preserved in the returned value and
/// its XSD formatting without applying a historical-era adjustment.
/// Supported years are `-9999` through `9999`. Year `0000` denotes the year
/// before `0001`, following XSD 1.1; negative zero (`-0000`) is rejected.
/// The proleptic Gregorian leap-year rule applies to signed years, including
/// zero: divisible by 4, except centuries not divisible by 400. Invalid dates
/// and years outside the supported range return errors rather than being clamped.
/// Bracketed timezone and calendar annotations (such as `[Europe/Paris]` and
/// `[u-ca=iso8601]`) are not XSD syntax and are rejected instead of discarded.
///
/// # Errors
///
/// Returns an error when the input contains a time component, contains bracketed
/// annotations, omits the required date-component hyphens, uses a leading plus
/// sign on the year, has a year longer than four digits beginning with zero, or
/// cannot be parsed by the underlying civil-date parser.
///
/// ```
/// assert!(xsd::parse_date("2026-12-31T12:34:56").is_err());
/// assert!(xsd::parse_date("2026-12-31[Europe/Paris]").is_err());
/// assert!(xsd::parse_date("20261231").is_err());
/// assert!(xsd::parse_date("+002026-12-31").is_err());
/// assert!(xsd::parse_date("-002026-12-31").is_err());
/// assert_eq!(
///     xsd::parse_date("-2026-12-31").unwrap(),
///     xsd::Value::from(xsd::primitive::Date::new(-2026, 12, 31).unwrap()),
/// );
/// assert_eq!(xsd::parse_date("2024-02-29").unwrap().to_string(), "2024-02-29");
/// ```
#[cfg(feature = "jiff")]
pub fn parse_date(input: impl AsRef<str>) -> Result<Value, ParseDateTimeError> {
    let input = input.as_ref();
    // Jiff requires six digits for negative years. Adapt four-digit XSD years
    // on the stack, keeping the original input for the lexical checks below.
    let mut normalized = [b'0'; 13];
    let civil_input = if input.len() == 11 && input.starts_with('-') && input.as_bytes()[5] == b'-'
    {
        normalized[0] = b'-';
        normalized[3..].copy_from_slice(&input.as_bytes()[1..]);
        core::str::from_utf8(&normalized).map_err(|error| {
            jiff::Error::from_args(format_args!("invalid xsd:date encoding: {error}"))
        })?
    } else {
        input
    };
    if jiff::fmt::temporal::DateTimeParser::new()
        .parse_pieces(civil_input)?
        .time()
        .is_some()
    {
        return Err(jiff::Error::from_args(format_args!(
            "xsd:date literals must not contain a time component"
        )));
    }
    // Preserve the civil-date parser's other checks, including offset handling.
    let date = civil_input.parse::<Date>()?;
    // Jiff accepts bracketed annotations that are outside the XSD lexical grammar.
    if input.as_bytes().contains(&b'[') {
        return Err(jiff::Error::from_args(format_args!(
            "xsd:date literals must not contain bracketed annotations"
        )));
    }
    // Jiff validates field widths and ranges; XSD requires both date separators.
    let unsigned = input.strip_prefix(['+', '-']).unwrap_or(input);
    if !unsigned
        .split_once('-')
        .is_some_and(|(_, rest)| matches!(rest.as_bytes(), [_, _, b'-', _, _, ..]))
    {
        return Err(jiff::Error::from_args(format_args!(
            "xsd:date literals require hyphen-separated year, month, and day"
        )));
    }
    if input.starts_with('+') {
        return Err(jiff::Error::from_args(format_args!(
            "xsd:date years must not have a leading plus sign"
        )));
    }
    if unsigned
        .split_once('-')
        .is_some_and(|(year, _)| year.len() > 4 && year.starts_with('0'))
    {
        return Err(jiff::Error::from_args(format_args!(
            "xsd:date years longer than four digits must not begin with zero"
        )));
    }
    Ok(Value::from(date))
}
