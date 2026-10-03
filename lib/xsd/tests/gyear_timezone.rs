use xsd::{PrimitiveValue, TimezoneOffset, Value, primitive::GYear};

#[test]
fn signed_year_boundaries_round_trip_at_every_offset() {
    for year in [i32::MIN, -10000, -1, 0, 1, 10000, i32::MAX] {
        let bare = GYear::new(year);
        assert_eq!(
            xsd::parse_g_year(bare.to_string()).unwrap(),
            Value::from(PrimitiveValue::GYear(bare))
        );
        for minutes in -840..=840 {
            let offset = TimezoneOffset::from_minutes(minutes).unwrap();
            let value = bare.with_timezone(Some(offset));
            let parsed = xsd::parse(value.to_string(), xsd::G_YEAR).unwrap();
            assert_eq!(parsed, Value::from(PrimitiveValue::GYear(value)));
            assert_eq!(parsed.r#type(), xsd::G_YEAR);
            assert_eq!(value.timezone(), Some(offset));
            assert_eq!(value.with_timezone(None), bare);
        }
    }
    for year in ["0000", "-0000"] {
        for suffix in ["", "Z", "+00:00", "-00:00", "+14:00", "-14:00"] {
            assert_eq!(
                xsd::parse_g_year(format!("{year}{suffix}")).unwrap(),
                xsd::parse_g_year(format!("0000{suffix}")).unwrap()
            );
        }
    }
}

#[test]
fn malformed_suffixes_and_unsupported_years_return_errors() {
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
        let error = xsd::parse(format!("-0001{suffix}"), xsd::G_YEAR).unwrap_err();
        assert!(
            matches!(
                error,
                xsd::ParseError::InvalidCalendar {
                    datatype: xsd::PrimitiveType::GYear,
                    source: xsd::ParseCalendarError::InvalidLexical
                }
            ),
            "{suffix}"
        );
        assert!(core::error::Error::source(&error).is_some());
    }
    for input in ["2147483648Z", "-2147483649Z"] {
        assert_eq!(
            xsd::parse_g_year(input).unwrap_err(),
            xsd::ParseCalendarError::OutOfRange
        );
    }
}
