use crate::{ParseCalendarError, PrimitiveValue, TimezoneOffset, Value};

/// Parses an `xsd:gYearMonth` literal (`yyyy-mm` and an optional timezone).
///
/// Uses the year grammar and full `i32` range of [`parse_g_year`], including
/// XSD 1.1 year zero. Requires a hyphen followed by exactly two ASCII month
/// digits in `01..=12`. Does not trim whitespace. Formatting preserves the year
/// and month without an era adjustment. Available without allocation or
/// date/time features.
/// An optional `Z`, `+hh:mm`, or `-hh:mm` suffix is retained, within
/// `-14:00..=+14:00`. Absence differs from UTC; zero offsets format as `Z`.
///
/// # Errors
///
/// Returns [`ParseCalendarError::InvalidLexical`] for malformed input,
/// including invalid timezone syntax or bounds, or
/// [`ParseCalendarError::OutOfRange`] for unsupported years or invalid months.
///
/// ```
/// assert_eq!(xsd::parse_g_year_month("-0001-02").unwrap().to_string(), "-0001-02");
/// assert!(xsd::parse_g_year_month("2026-13").is_err());
/// assert_eq!(xsd::parse_g_year_month("0000-01-00:00").unwrap().to_string(), "0000-01Z");
/// ```
pub fn parse_g_year_month(input: impl AsRef<str>) -> Result<Value, ParseCalendarError> {
    let (year, suffix) = year_prefix(input.as_ref())?;
    let month_input = suffix
        .strip_prefix('-')
        .ok_or(ParseCalendarError::InvalidLexical)?;
    let month = two_digits(month_input.as_bytes().get(..2))?;
    let value =
        crate::primitive::GYearMonth::new(year, month).ok_or(ParseCalendarError::OutOfRange)?;
    let timezone = match month_input.get(2..) {
        Some("") => None,
        Some(suffix) => Some(
            suffix
                .parse()
                .map_err(|_| ParseCalendarError::InvalidLexical)?,
        ),
        None => return Err(ParseCalendarError::InvalidLexical),
    };
    Ok(PrimitiveValue::GYearMonth(value.with_timezone(timezone)).into())
}

/// Parses a timezone-free `xsd:gYear` following XSD 1.1 year numbering.
///
/// Requires at least four ASCII digits, optionally preceded by `-`. Longer
/// years must not start with zero. Year `0000` is accepted; `-0000` and a leading
/// `+` are rejected. Supports the entire `i32` range without an era adjustment.
/// Input is not trimmed. Available without allocation or date/time features.
///
/// # Errors
///
/// Returns [`ParseCalendarError::InvalidLexical`] for invalid spelling,
/// [`ParseCalendarError::OutOfRange`] for years outside `i32`, or
/// [`ParseCalendarError::UnsupportedTimezone`] for a valid timezone suffix.
///
/// ```
/// assert_eq!(xsd::parse_g_year("-0001").unwrap().to_string(), "-0001");
/// assert!(xsd::parse_g_year("-0000").is_err());
/// ```
pub fn parse_g_year(input: impl AsRef<str>) -> Result<Value, ParseCalendarError> {
    let (year, suffix) = year_prefix(input.as_ref())?;
    require_no_timezone(Some(suffix))?;
    Ok(PrimitiveValue::GYear(year).into())
}

pub(crate) fn year_prefix(input: &str) -> Result<(i32, &str), ParseCalendarError> {
    let negative = input.starts_with('-');
    let unsigned = input.strip_prefix('-').unwrap_or(input);
    let digits = unsigned.bytes().take_while(u8::is_ascii_digit).count();
    if digits < 4 || (digits > 4 && unsigned.starts_with('0')) {
        return Err(ParseCalendarError::InvalidLexical);
    }
    let end = digits + usize::from(negative);
    let year = input[..end]
        .parse::<i32>()
        .map_err(|_| ParseCalendarError::OutOfRange)?;
    if negative && year == 0 {
        return Err(ParseCalendarError::InvalidLexical);
    }
    Ok((year, &input[end..]))
}

/// Parses an `xsd:gMonthDay` literal (`--mm-dd` and an optional timezone).
///
/// Validates the two-digit ASCII fields against the Gregorian month lengths,
/// allowing February 29 because no year is specified. Requires both hyphens
/// before the month and one between the fields. Does not trim whitespace.
/// Available without allocation or date/time features.
/// An optional `Z`, `+hh:mm`, or `-hh:mm` suffix is retained, within
/// `-14:00..=+14:00`. Absence differs from UTC; zero offsets format as `Z`.
/// Formatting preserves calendar fields and offset, not original spelling.
///
/// # Errors
///
/// Returns [`ParseCalendarError::InvalidLexical`] for malformed input,
/// including invalid timezone syntax or bounds, or
/// [`ParseCalendarError::OutOfRange`] for impossible month/day combinations.
///
/// ```
/// assert_eq!(xsd::parse_g_month_day("--02-29").unwrap().to_string(), "--02-29");
/// assert!(xsd::parse_g_month_day("--04-31").is_err());
/// assert_eq!(xsd::parse_g_month_day("--02-29-00:00").unwrap().to_string(), "--02-29Z");
/// ```
pub fn parse_g_month_day(input: impl AsRef<str>) -> Result<Value, ParseCalendarError> {
    let input = input.as_ref();
    let bytes = input.as_bytes();
    if !bytes.starts_with(b"--") || bytes.get(4) != Some(&b'-') {
        return Err(ParseCalendarError::InvalidLexical);
    }
    let month = two_digits(bytes.get(2..4))?;
    let day = two_digits(bytes.get(5..7))?;
    let value =
        crate::primitive::GMonthDay::new(month, day).ok_or(ParseCalendarError::OutOfRange)?;
    let timezone = match input.get(7..) {
        Some("") => None,
        Some(suffix) => Some(
            suffix
                .parse()
                .map_err(|_| ParseCalendarError::InvalidLexical)?,
        ),
        None => return Err(ParseCalendarError::InvalidLexical),
    };
    Ok(PrimitiveValue::GMonthDay(value.with_timezone(timezone)).into())
}

/// Parses an `xsd:gDay` literal (`---dd` and an optional timezone).
///
/// Requires exactly two ASCII digits in `01..=31`. No month is implied, so
/// day 31 is valid. Input is not trimmed. Available without allocation or
/// date/time features. An optional `Z`, `+hh:mm`, or `-hh:mm` suffix is retained,
/// within `-14:00..=+14:00`. Absence differs from UTC; zero offsets format as `Z`.
/// Formatting preserves the day and offset, not original input spelling.
///
/// # Errors
///
/// Returns [`ParseCalendarError::InvalidLexical`] for malformed input, including
/// invalid timezone syntax or bounds, or [`ParseCalendarError::OutOfRange`]
/// for invalid days.
///
/// ```
/// assert_eq!(xsd::parse_g_day("---31").unwrap().to_string(), "---31");
/// assert!(xsd::parse_g_day("---00").is_err());
/// assert_eq!(xsd::parse_g_day("---31+00:00").unwrap().to_string(), "---31Z");
/// ```
pub fn parse_g_day(input: impl AsRef<str>) -> Result<Value, ParseCalendarError> {
    let input = input.as_ref();
    let bytes = input.as_bytes();
    if !bytes.starts_with(b"---") {
        return Err(ParseCalendarError::InvalidLexical);
    }
    let day = two_digits(bytes.get(3..5))?;
    let day = crate::primitive::GDay::new(day).ok_or(ParseCalendarError::OutOfRange)?;
    let timezone = match input.get(5..) {
        Some("") => None,
        Some(suffix) => Some(
            suffix
                .parse()
                .map_err(|_| ParseCalendarError::InvalidLexical)?,
        ),
        None => return Err(ParseCalendarError::InvalidLexical),
    };
    Ok(PrimitiveValue::GDay(day.with_timezone(timezone)).into())
}

/// Parses an `xsd:gMonth` using XSD 1.1 syntax (`--mm` and an optional timezone).
///
/// Available without allocation or date/time features. Requires exactly two
/// ASCII month digits in `01..=12`, without trimming whitespace. Legacy trailing
/// hyphens are rejected. An optional `Z`, `+hh:mm`, or `-hh:mm` suffix is retained,
/// within `-14:00..=+14:00`. Absence differs from UTC; zero offsets format as `Z`.
/// Formatting preserves the month and offset, not original input spelling.
///
/// # Errors
///
/// Returns [`ParseCalendarError::InvalidLexical`] for malformed input or
/// [`ParseCalendarError::OutOfRange`] for invalid months. Invalid timezone syntax
/// or bounds return [`ParseCalendarError::InvalidLexical`].
///
/// ```
/// assert_eq!(xsd::parse_g_month("--02").unwrap().to_string(), "--02");
/// assert!(xsd::parse_g_month("--13").is_err());
/// assert_eq!(xsd::parse_g_month("--02+00:00").unwrap().to_string(), "--02Z");
/// ```
pub fn parse_g_month(input: impl AsRef<str>) -> Result<Value, ParseCalendarError> {
    let input = input.as_ref();
    let bytes = input.as_bytes();
    if !bytes.starts_with(b"--") {
        return Err(ParseCalendarError::InvalidLexical);
    }
    let month = two_digits(bytes.get(2..4))?;
    let month = crate::primitive::GMonth::new(month).ok_or(ParseCalendarError::OutOfRange)?;
    let timezone = match input.get(4..) {
        Some("") => None,
        Some(suffix) => Some(
            suffix
                .parse()
                .map_err(|_| ParseCalendarError::InvalidLexical)?,
        ),
        None => return Err(ParseCalendarError::InvalidLexical),
    };
    Ok(PrimitiveValue::GMonth(month.with_timezone(timezone)).into())
}

fn two_digits(bytes: Option<&[u8]>) -> Result<u8, ParseCalendarError> {
    match bytes {
        Some([a, b]) if a.is_ascii_digit() && b.is_ascii_digit() => Ok((a - b'0') * 10 + b - b'0'),
        _ => Err(ParseCalendarError::InvalidLexical),
    }
}

fn require_no_timezone(suffix: Option<&str>) -> Result<(), ParseCalendarError> {
    match suffix {
        Some("") => Ok(()),
        Some(suffix) if suffix.parse::<TimezoneOffset>().is_ok() => {
            Err(ParseCalendarError::UnsupportedTimezone)
        },
        _ => Err(ParseCalendarError::InvalidLexical),
    }
}
