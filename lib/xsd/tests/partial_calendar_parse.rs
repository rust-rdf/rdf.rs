#[test]
fn months_round_trip() {
    for month in 1..=12 {
        let lexical = format!("--{month:02}");
        let value = xsd::parse(&lexical, xsd::G_MONTH).unwrap();
        assert_eq!(value.to_string(), lexical);
        assert_eq!(value.r#type(), xsd::G_MONTH);
        assert_eq!(xsd::parse_g_month(&lexical).unwrap(), value);
    }
}

#[test]
fn months_report_lexical_range_and_timezone_failures() {
    use xsd::ParseCalendarError::{InvalidLexical, OutOfRange, UnsupportedTimezone};
    for (input, cause) in [
        ("--00", OutOfRange),
        ("--13", OutOfRange),
        ("--99", OutOfRange),
        ("--1", InvalidLexical),
        ("--01--", InvalidLexical),
        (" --01", InvalidLexical),
        ("--０1", InvalidLexical),
        ("--01+14:01", InvalidLexical),
        ("--01Z", UnsupportedTimezone),
        ("--01-14:00", UnsupportedTimezone),
    ] {
        assert_eq!(xsd::parse_g_month(input).unwrap_err(), cause, "{input}");
        let error = xsd::parse(input, xsd::G_MONTH).unwrap_err();
        assert!(core::error::Error::source(&error).is_some());
        assert!(error.to_string().contains("xsd:gMonth"));
        assert!(
            matches!(error, xsd::ParseError::InvalidCalendar { datatype: xsd::PrimitiveType::GMonth, source } if source == cause)
        );
    }
}
#[test]
fn days_round_trip_and_reject_invalid_fields() {
    use xsd::ParseCalendarError::{InvalidLexical, OutOfRange, UnsupportedTimezone};
    for day in 1..=31 {
        let lexical = format!("---{day:02}");
        let value = xsd::parse_g_day(&lexical).unwrap();
        assert_eq!(value.to_string(), lexical);
        assert_eq!(value.r#type(), xsd::G_DAY);
        assert_eq!(xsd::parse(&lexical, xsd::G_DAY).unwrap(), value);
    }
    for (input, cause) in [
        ("---00", OutOfRange),
        ("---32", OutOfRange),
        ("---99", OutOfRange),
        ("---1", InvalidLexical),
        ("--01", InvalidLexical),
        ("---01 ", InvalidLexical),
        ("---０1", InvalidLexical),
        ("---01+00:60", InvalidLexical),
        ("---01Z", UnsupportedTimezone),
        ("---01+14:00", UnsupportedTimezone),
    ] {
        assert_eq!(xsd::parse_g_day(input).unwrap_err(), cause, "{input}");
        assert!(
            matches!(xsd::parse(input, xsd::G_DAY), Err(xsd::ParseError::InvalidCalendar { datatype: xsd::PrimitiveType::GDay, source }) if source == cause)
        );
    }
}

#[test]
fn month_days_round_trip_and_validate_calendar_combinations() {
    use xsd::ParseCalendarError::{InvalidLexical, OutOfRange, UnsupportedTimezone};
    for (month, last_day) in (1..=12).zip([31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]) {
        for day in 0..=32 {
            let lexical = format!("--{month:02}-{day:02}");
            if day == 0 || day > last_day {
                assert_eq!(xsd::parse_g_month_day(&lexical).unwrap_err(), OutOfRange);
            } else {
                let value = xsd::parse_g_month_day(&lexical).unwrap();
                assert_eq!(value.to_string(), lexical);
                assert_eq!(value.r#type(), xsd::G_MONTH_DAY);
                assert_eq!(xsd::parse(&lexical, xsd::G_MONTH_DAY).unwrap(), value);
            }
        }
    }
    for (input, cause) in [
        ("--00-01", OutOfRange),
        ("--13-01", OutOfRange),
        ("--1-01", InvalidLexical),
        ("--0101", InvalidLexical),
        ("--01-01 ", InvalidLexical),
        ("--01-１", InvalidLexical),
        ("--01-01+15:00", InvalidLexical),
        ("--02-29Z", UnsupportedTimezone),
        ("--01-01-00:00", UnsupportedTimezone),
    ] {
        assert_eq!(xsd::parse_g_month_day(input).unwrap_err(), cause, "{input}");
        assert!(
            matches!(xsd::parse(input, xsd::G_MONTH_DAY), Err(xsd::ParseError::InvalidCalendar { datatype: xsd::PrimitiveType::GMonthDay, source }) if source == cause)
        );
    }
}
