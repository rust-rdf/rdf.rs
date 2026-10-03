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
