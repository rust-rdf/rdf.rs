// This is free and unencumbered software released into the public domain.

#![cfg(feature = "jiff")]

#[test]
fn datetime_fractions_preserve_nanoseconds_and_signed_years() {
    for input in [
        "2024-02-29T12:34:56.1",
        "2024-02-29T12:34:56.123456789",
        "-2024-02-29T00:00:00.000000001",
    ] {
        let value = xsd::parse_datetime(input).unwrap();
        assert_eq!(value.to_string(), input);
        assert_eq!(xsd::parse(input, xsd::DATE_TIME).unwrap(), value);
        assert_eq!(xsd::parse_datetime(value.to_string()).unwrap(), value);
    }
}

#[test]
fn datetime_precision_limits_are_explicit() {
    for input in [
        "2024-02-29T12:34:56.1234567891",
        "2024-02-29T12:34:56.1234567890",
        "2024-02-29T12:34:56.0000000000",
        "-2024-02-29T12:34:56.1234567891+02:00",
    ] {
        let error = xsd::parse_datetime(input).unwrap_err();
        assert!(
            error.to_string().contains("nanosecond precision"),
            "{input}: {error}"
        );
        assert!(matches!(
            xsd::parse(input, xsd::DATE_TIME),
            Err(xsd::ParseError::InvalidTemporal {
                datatype: xsd::PrimitiveType::DateTime,
                ..
            })
        ));
    }
    assert_eq!(
        xsd::parse_datetime("2024-02-29T24:00:00.0000000000")
            .unwrap()
            .to_string(),
        "2024-03-01T00:00:00"
    );
}

#[test]
fn datetime_rejects_malformed_fractions() {
    for input in [
        "2024-02-29T12:34:56.",
        "2024-02-29T12:34:56.+02:00",
        "2024-02-29T12:34:56..1",
        "2024-02-29T12:34:56.１２",
        "2024-02-29T12:34:56.1e2",
    ] {
        assert!(xsd::parse_datetime(input).is_err(), "{input}");
        assert!(xsd::parse(input, xsd::DATE_TIME).is_err(), "{input}");
    }
}
