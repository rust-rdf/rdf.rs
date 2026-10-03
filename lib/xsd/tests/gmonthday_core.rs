use core::fmt::Write;
use xsd::{PrimitiveValue, TimezoneOffset, Value, primitive::GMonthDay};

#[test]
fn month_day_round_trip_uses_fixed_capacity_buffers() {
    const LEAP_DAY: GMonthDay = GMonthDay::new(2, 29)
        .unwrap()
        .with_timezone(Some(TimezoneOffset::from_minutes(-840).unwrap()));
    let mut lexical = heapless::String::<13>::new();
    write!(&mut lexical, "{LEAP_DAY}").unwrap();
    assert_eq!(lexical.as_str(), "--02-29-14:00");
    assert_eq!(
        xsd::parse(&lexical, xsd::G_MONTH_DAY).unwrap(),
        Value::from(PrimitiveValue::GMonthDay(LEAP_DAY))
    );
    let mut too_small = heapless::String::<11>::new();
    assert!(write!(&mut too_small, "{LEAP_DAY}").is_err());
}

#[test]
#[cfg(not(feature = "jiff"))]
fn partial_calendar_support_does_not_require_full_calendar_support() {
    assert!(xsd::parse("--02-29Z", xsd::G_MONTH_DAY).is_ok());
    assert!(matches!(
        xsd::parse("--02-30Z", xsd::G_MONTH_DAY),
        Err(xsd::ParseError::InvalidCalendar {
            source: xsd::ParseCalendarError::OutOfRange,
            ..
        })
    ));
    assert!(matches!(xsd::parse("2024-02-29Z", xsd::DATE),
        Err(xsd::ParseError::UnsupportedDatatype(datatype)) if datatype == xsd::DATE));
}
