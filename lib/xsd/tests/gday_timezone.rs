use xsd::{PrimitiveValue, TimezoneOffset, Value, primitive::GDay};

#[test]
fn every_day_and_offset_round_trips() {
    for day in 1..=31 {
        let bare = GDay::new(day).unwrap();
        assert_eq!(
            xsd::parse_g_day(bare.to_string()).unwrap(),
            Value::from(PrimitiveValue::GDay(bare))
        );
        for minutes in -840..=840 {
            let offset = TimezoneOffset::from_minutes(minutes).unwrap();
            let value = bare.with_timezone(Some(offset));
            let parsed = xsd::parse(value.to_string(), xsd::G_DAY).unwrap();
            assert_eq!(parsed, Value::from(PrimitiveValue::GDay(value)));
            assert_eq!(parsed.r#type(), xsd::G_DAY);
            assert_eq!(value.timezone(), Some(offset));
            assert_eq!(value.with_timezone(None), bare);
        }
    }
    for input in ["---31Z", "---31+00:00", "---31-00:00"] {
        assert_eq!(xsd::parse_g_day(input).unwrap().to_string(), "---31Z");
    }
}

#[test]
fn malformed_suffixes_return_errors() {
    for suffix in [
        "z",
        "+14:01",
        "-14:01",
        "+15:00",
        "+00:60",
        "+01",
        "+0100",
        "+01:00:00",
        "ZZ",
        "Z+01:00",
        "[UTC]",
        " ",
        "é",
        "+０1:00",
    ] {
        let error = xsd::parse(format!("---01{suffix}"), xsd::G_DAY).unwrap_err();
        assert!(
            matches!(
                error,
                xsd::ParseError::InvalidCalendar {
                    datatype: xsd::PrimitiveType::GDay,
                    source: xsd::ParseCalendarError::InvalidLexical
                }
            ),
            "{suffix}"
        );
        assert!(core::error::Error::source(&error).is_some());
    }
}
