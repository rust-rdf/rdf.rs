// This is free and unencumbered software released into the public domain.

#![cfg(feature = "jiff")]

#[test]
fn calendar_year_boundaries_and_leap_days_round_trip() {
    for date in [
        "-9999-01-01",
        "9999-12-31",
        "0000-02-29",
        "-0001-01-01",
        "0001-01-01",
        "2000-02-29",
        "2400-02-29",
        "-2000-02-29",
        "-0004-02-29",
    ] {
        let value = xsd::parse_date(date).unwrap();
        assert_eq!(value.to_string(), date);
        assert_eq!(xsd::parse(date, xsd::DATE).unwrap(), value);
        let datetime = format!("{date}T23:59:59.999999999");
        let value = xsd::parse_datetime(&datetime).unwrap();
        assert_eq!(value.to_string(), datetime);
        assert_eq!(xsd::parse(&datetime, xsd::DATE_TIME).unwrap(), value);
    }
}

#[test]
fn calendar_parsers_reject_invalid_dates_and_unsupported_years() {
    for date in [
        "10000-01-01",
        "-10000-01-01",
        "999999999999999-01-01",
        "-0000-01-01",
        "1900-02-29",
        "2100-02-29",
        "-1900-02-29",
        "0001-02-29",
        "-0001-02-29",
        "2024-00-01",
        "2024-13-01",
        "2024-01-00",
        "2024-04-31",
        "2024-02-30",
    ] {
        assert!(xsd::parse_date(date).is_err(), "{date}");
        assert!(matches!(
            xsd::parse(date, xsd::DATE),
            Err(xsd::ParseError::InvalidTemporal {
                datatype: xsd::PrimitiveType::Date,
                ..
            })
        ));
        for clock in ["00:00:00", "24:00:00"] {
            let datetime = format!("{date}T{clock}");
            assert!(xsd::parse_datetime(&datetime).is_err(), "{datetime}");
            assert!(matches!(
                xsd::parse(&datetime, xsd::DATE_TIME),
                Err(xsd::ParseError::InvalidTemporal {
                    datatype: xsd::PrimitiveType::DateTime,
                    ..
                })
            ));
        }
    }
}
