use core::fmt::Write;
use xsd::{PrimitiveValue, TimezoneOffset, Value, primitive::GDay};

#[test]
fn day_round_trip_uses_fixed_capacity_buffers() {
    const DAY: GDay = GDay::new(31)
        .unwrap()
        .with_timezone(Some(TimezoneOffset::from_minutes(-840).unwrap()));
    let mut lexical = heapless::String::<11>::new();
    write!(&mut lexical, "{DAY}").unwrap();
    assert_eq!(lexical.as_str(), "---31-14:00");
    assert_eq!(
        xsd::parse(&lexical, xsd::G_DAY).unwrap(),
        Value::from(PrimitiveValue::GDay(DAY))
    );
    let mut too_small = heapless::String::<9>::new();
    assert!(write!(&mut too_small, "{DAY}").is_err());
}

#[test]
#[cfg(not(feature = "jiff"))]
fn partial_day_support_does_not_require_full_calendar_support() {
    assert!(xsd::parse("---31Z", xsd::G_DAY).is_ok());
    assert!(matches!(
        xsd::parse("---32Z", xsd::G_DAY),
        Err(xsd::ParseError::InvalidCalendar {
            source: xsd::ParseCalendarError::OutOfRange,
            ..
        })
    ));
    assert!(matches!(xsd::parse("2026-12-31Z", xsd::DATE),
        Err(xsd::ParseError::UnsupportedDatatype(datatype)) if datatype == xsd::DATE));
}
