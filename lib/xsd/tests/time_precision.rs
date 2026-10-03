// This is free and unencumbered software released into the public domain.

#![cfg(feature = "jiff")]

#[test]
fn time_fractional_seconds_round_trip_at_nanosecond_precision() {
    for input in ["12:34:56.1", "12:34:56.123456789", "00:00:00.000000001"] {
        let value = xsd::parse_time(input).unwrap();
        assert_eq!(value.to_string(), input);
        assert_eq!(xsd::parse(input, xsd::TIME).unwrap(), value);
        assert_eq!(xsd::parse_time(value.to_string()).unwrap(), value);
    }
    assert_eq!(
        xsd::parse_time("12:34:56.100000000").unwrap().to_string(),
        "12:34:56.1"
    );
}

#[test]
fn time_precision_limits_are_explicit() {
    for input in [
        "12:34:56.1234567891",
        "12:34:56.1234567890",
        "12:34:56.0000000000",
        "12:34:56.1234567891+02:00",
    ] {
        let error = xsd::parse_time(input).unwrap_err();
        assert!(
            error.to_string().contains("nanosecond precision"),
            "{input}: {error}"
        );
        assert!(matches!(
            xsd::parse(input, xsd::TIME),
            Err(xsd::ParseError::InvalidTemporal {
                datatype: xsd::PrimitiveType::Time,
                ..
            })
        ));
    }
    assert_eq!(
        xsd::parse_time("24:00:00.0000000000").unwrap().to_string(),
        "00:00:00"
    );
}

#[test]
fn time_rejects_malformed_fractions() {
    for input in [
        "12:34:56.",
        "12:34:56.+02:00",
        "12:34:56..1",
        "12:34:56.１２",
        "12:34:56.1e2",
    ] {
        assert!(xsd::parse_time(input).is_err(), "{input}");
        assert!(xsd::parse(input, xsd::TIME).is_err(), "{input}");
    }
}
