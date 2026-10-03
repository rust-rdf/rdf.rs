use xsd::{PrimitiveValue, TimezoneOffset, Value, primitive::GMonthDay};

#[test]
fn calendar_boundaries_round_trip_at_every_offset() {
    for (month, day) in [(1, 1), (2, 28), (2, 29), (4, 30), (12, 31)] {
        let bare = GMonthDay::new(month, day).unwrap();
        assert_eq!(
            xsd::parse_g_month_day(bare.to_string()).unwrap(),
            Value::from(PrimitiveValue::GMonthDay(bare))
        );
        for minutes in -840..=840 {
            let offset = TimezoneOffset::from_minutes(minutes).unwrap();
            let value = bare.with_timezone(Some(offset));
            let parsed = xsd::parse(value.to_string(), xsd::G_MONTH_DAY).unwrap();
            assert_eq!(parsed, Value::from(PrimitiveValue::GMonthDay(value)));
            assert_eq!(parsed.r#type(), xsd::G_MONTH_DAY);
            assert_eq!(value.timezone(), Some(offset));
            assert_eq!(value.with_timezone(None), bare);
        }
    }
    for input in ["--02-29Z", "--02-29+00:00", "--02-29-00:00"] {
        assert_eq!(
            xsd::parse_g_month_day(input).unwrap().to_string(),
            "--02-29Z"
        );
    }
}

#[test]
fn malformed_suffixes_and_impossible_dates_return_errors() {
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
        let error = xsd::parse(format!("--02-29{suffix}"), xsd::G_MONTH_DAY).unwrap_err();
        assert!(
            matches!(
                error,
                xsd::ParseError::InvalidCalendar {
                    datatype: xsd::PrimitiveType::GMonthDay,
                    source: xsd::ParseCalendarError::InvalidLexical
                }
            ),
            "{suffix}"
        );
        assert!(core::error::Error::source(&error).is_some());
    }
    for input in ["--02-30Z", "--04-31+01:00", "--00-01Z", "--01-00Z"] {
        assert_eq!(
            xsd::parse_g_month_day(input).unwrap_err(),
            xsd::ParseCalendarError::OutOfRange
        );
    }
}
