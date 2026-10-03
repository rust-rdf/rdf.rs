use core::fmt::Write;
use xsd::{PrimitiveValue, TimezoneOffset, Value, primitive::GMonth};

#[test]
fn month_round_trip_uses_fixed_capacity_buffers() {
    const MONTH: GMonth = GMonth::new(12)
        .unwrap()
        .with_timezone(Some(TimezoneOffset::from_minutes(-840).unwrap()));
    let mut lexical = heapless::String::<10>::new();
    write!(&mut lexical, "{MONTH}").unwrap();
    assert_eq!(lexical.as_str(), "--12-14:00");
    assert_eq!(
        xsd::parse(&lexical, xsd::G_MONTH).unwrap(),
        Value::from(PrimitiveValue::GMonth(MONTH))
    );
    let mut too_small = heapless::String::<8>::new();
    assert!(write!(&mut too_small, "{MONTH}").is_err());
}

#[test]
#[cfg(not(feature = "jiff"))]
fn partial_month_support_does_not_require_full_calendar_support() {
    assert!(xsd::parse("--12Z", xsd::G_MONTH).is_ok());
    assert!(matches!(
        xsd::parse("--13Z", xsd::G_MONTH),
        Err(xsd::ParseError::InvalidCalendar {
            source: xsd::ParseCalendarError::OutOfRange,
            ..
        })
    ));
    assert!(matches!(xsd::parse("2026-12-01Z", xsd::DATE),
        Err(xsd::ParseError::UnsupportedDatatype(datatype)) if datatype == xsd::DATE));
}
