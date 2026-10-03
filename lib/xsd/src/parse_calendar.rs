use crate::{ParseCalendarError, PrimitiveValue, TimezoneOffset, Value};

/// Parses a timezone-free `xsd:gDay` literal (`---dd`).
///
/// Requires exactly two ASCII digits in `01..=31`. No month is implied, so
/// day 31 is valid. Input is not trimmed. Available without allocation or
/// date/time features; the returned value formats as `---dd`.
///
/// # Errors
///
/// Returns [`ParseCalendarError::InvalidLexical`] for malformed input,
/// [`ParseCalendarError::OutOfRange`] for invalid days, or
/// [`ParseCalendarError::UnsupportedTimezone`] for a valid timezone suffix.
///
/// ```
/// assert_eq!(xsd::parse_g_day("---31").unwrap().to_string(), "---31");
/// assert!(xsd::parse_g_day("---00").is_err());
/// ```
pub fn parse_g_day(input: impl AsRef<str>) -> Result<Value, ParseCalendarError> {
    let input = input.as_ref();
    let bytes = input.as_bytes();
    if !bytes.starts_with(b"---") {
        return Err(ParseCalendarError::InvalidLexical);
    }
    let day = two_digits(bytes.get(3..5))?;
    let value = PrimitiveValue::g_day(day).ok_or(ParseCalendarError::OutOfRange)?;
    require_no_timezone(input.get(5..))?;
    Ok(value.into())
}

/// Parses a timezone-free `xsd:gMonth` using XSD 1.1 syntax (`--mm`).
///
/// Available without allocation or date/time features. Requires exactly two
/// ASCII month digits in `01..=12`, without trimming whitespace. Legacy trailing
/// hyphens are rejected. Formatting preserves the month, not original input.
///
/// # Errors
///
/// Returns [`ParseCalendarError::InvalidLexical`] for malformed input or
/// [`ParseCalendarError::OutOfRange`] for invalid months. A valid timezone suffix
/// returns [`ParseCalendarError::UnsupportedTimezone`] instead of being discarded.
///
/// ```
/// assert_eq!(xsd::parse_g_month("--02").unwrap().to_string(), "--02");
/// assert!(xsd::parse_g_month("--13").is_err());
/// assert!(xsd::parse_g_month("--02Z").is_err());
/// ```
pub fn parse_g_month(input: impl AsRef<str>) -> Result<Value, ParseCalendarError> {
    let input = input.as_ref();
    let bytes = input.as_bytes();
    if !bytes.starts_with(b"--") {
        return Err(ParseCalendarError::InvalidLexical);
    }
    let month = two_digits(bytes.get(2..4))?;
    let value = PrimitiveValue::g_month(month).ok_or(ParseCalendarError::OutOfRange)?;
    require_no_timezone(input.get(4..))?;
    Ok(value.into())
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
