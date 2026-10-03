// This is free and unencumbered software released into the public domain.

#![cfg(feature = "jiff")]

#[test]
fn datetime_end_of_day_advances_the_calendar() {
    for (date, next) in [
        ("2026-12-31", "2027-01-01"),
        ("2024-02-28", "2024-02-29"),
        ("2024-02-29", "2024-03-01"),
        ("2023-02-28", "2023-03-01"),
        ("-2024-02-28", "-2024-02-29"),
        ("-0001-12-31", "0000-01-01"),
    ] {
        for fraction in ["", ".0", ".00000000000000000000"] {
            let input = format!("{date}T24:00:00{fraction}");
            let lexical = format!("{next}T00:00:00");
            let value = xsd::parse_datetime(&input).unwrap();
            assert_eq!(value.to_string(), lexical);
            assert_eq!(value.r#type(), xsd::DATE_TIME);
            assert_eq!(xsd::parse(&input, xsd::DATE_TIME).unwrap(), value);
            assert_eq!(xsd::parse(&lexical, xsd::DATE_TIME).unwrap(), value);
        }
    }
}

#[test]
fn datetime_end_of_day_rejects_invalid_or_unrepresentable_inputs() {
    for input in [
        "9999-12-31T24:00:00",
        "2023-02-29T24:00:00",
        "2026-13-01T24:00:00",
        "2026-12-31T24:00:00.1",
        "2026-12-31T24:00:00.0000000001",
        "2026-12-31T24:00:00.",
        "2026-12-31T24:01:00",
        "2026-12-31T24:00:01",
        "2026-12-31t24:00:00",
        "2026-12-31 24:00:00",
        "20261231T24:00:00",
        "+002026-12-31T24:00:00",
        "-002026-12-31T24:00:00",
        "2026-12-31T24:00:00Z",
        "2026-12-31T24:00:00+02:00",
        "2026-12-31T24:00:00[Etc/UTC]",
    ] {
        assert!(xsd::parse_datetime(input).is_err(), "{input}");
        assert!(
            matches!(
                xsd::parse(input, xsd::DATE_TIME),
                Err(xsd::ParseError::InvalidTemporal {
                    datatype: xsd::PrimitiveType::DateTime,
                    ..
                })
            ),
            "{input}"
        );
    }
}
