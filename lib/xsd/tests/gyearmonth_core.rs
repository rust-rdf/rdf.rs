use core::fmt::Write;
use xsd::{PrimitiveValue, TimezoneOffset, Value, primitive::GYearMonth};

#[test]
fn year_month_boundaries_use_fixed_capacity_buffers() {
    const MINIMUM: GYearMonth = GYearMonth::new(i32::MIN, 12)
        .unwrap()
        .with_timezone(Some(TimezoneOffset::from_minutes(-840).unwrap()));
    let mut lexical = heapless::String::<20>::new();
    write!(&mut lexical, "{MINIMUM}").unwrap();
    assert_eq!(lexical.as_str(), "-2147483648-12-14:00");
    assert_eq!(
        xsd::parse(&lexical, xsd::G_YEAR_MONTH).unwrap(),
        Value::from(PrimitiveValue::GYearMonth(MINIMUM))
    );
    let mut too_small = heapless::String::<19>::new();
    assert!(write!(&mut too_small, "{MINIMUM}").is_err());
}

#[test]
#[cfg(not(feature = "jiff"))]
fn partial_calendar_support_does_not_require_full_calendar_support() {
    assert_eq!(
        xsd::parse("-0000-01Z", xsd::G_YEAR_MONTH).unwrap(),
        xsd::parse("0000-01Z", xsd::G_YEAR_MONTH).unwrap()
    );
    assert!(matches!(
        xsd::parse("2147483648-01Z", xsd::G_YEAR_MONTH),
        Err(xsd::ParseError::InvalidCalendar {
            source: xsd::ParseCalendarError::OutOfRange,
            ..
        })
    ));
    assert!(matches!(xsd::parse("2026-01-01Z", xsd::DATE),
        Err(xsd::ParseError::UnsupportedDatatype(datatype)) if datatype == xsd::DATE));
}
