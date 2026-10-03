use xsd::{PrimitiveValue, TimezoneOffset, Value, primitive::GYearMonth};

#[test]
fn year_boundaries_round_trip_at_every_offset() {
    assert_eq!(
        xsd::parse_g_year_month("-0000-01Z").unwrap().to_string(),
        "0000-01Z"
    );
    for year in [i32::MIN, -10000, -1, 0, 1, 10000, i32::MAX] {
        for month in [1, 12] {
            let bare = GYearMonth::new(year, month).unwrap();
            assert_eq!(
                xsd::parse_g_year_month(bare.to_string()).unwrap(),
                Value::from(PrimitiveValue::GYearMonth(bare))
            );
            for minutes in -840..=840 {
                let offset = TimezoneOffset::from_minutes(minutes).unwrap();
                let value = bare.with_timezone(Some(offset));
                let parsed = xsd::parse(value.to_string(), xsd::G_YEAR_MONTH).unwrap();
                assert_eq!(parsed, Value::from(PrimitiveValue::GYearMonth(value)));
                assert_eq!(parsed.r#type(), xsd::G_YEAR_MONTH);
                assert_eq!(value.timezone(), Some(offset));
                assert_eq!(value.with_timezone(None), bare);
            }
        }
    }
    for input in ["0000-01Z", "0000-01+00:00", "0000-01-00:00"] {
        assert_eq!(
            xsd::parse_g_year_month(input).unwrap().to_string(),
            "0000-01Z"
        );
    }
}

#[test]
fn malformed_suffixes_and_out_of_range_fields_return_errors() {
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
        let error = xsd::parse(format!("-0001-01{suffix}"), xsd::G_YEAR_MONTH).unwrap_err();
        assert!(
            matches!(
                error,
                xsd::ParseError::InvalidCalendar {
                    datatype: xsd::PrimitiveType::GYearMonth,
                    source: xsd::ParseCalendarError::InvalidLexical
                }
            ),
            "{suffix}"
        );
        assert!(core::error::Error::source(&error).is_some());
    }
    for input in [
        "2147483648-01Z",
        "-2147483649-12Z",
        "0000-00Z",
        "0000-13Z",
        "-0000-00Z",
    ] {
        assert_eq!(
            xsd::parse_g_year_month(input).unwrap_err(),
            xsd::ParseCalendarError::OutOfRange
        );
    }
}
