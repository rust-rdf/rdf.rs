#[cfg(feature = "jiff")]
#[test]
fn signed_duration_boundaries_round_trip() {
    use xsd::primitive::Duration;

    for duration in [
        Duration::new(i64::MIN, -999_999_999),
        Duration::new(i64::MAX, 999_999_999),
        Duration::new(0, -1),
        Duration::ZERO,
    ] {
        let value = xsd::Value::from(duration);
        assert_eq!(xsd::parse(value.to_string(), xsd::DURATION).unwrap(), value);
    }
    for input in ["PT9223372036854775808S", "-PT9223372036854775809S"] {
        assert!(xsd::parse(input, xsd::DURATION).is_err(), "{input}");
    }
}

#[cfg(feature = "jiff")]
#[test]
fn calendar_duration_components_are_unsupported() {
    for input in ["P1Y", "P1M", "P1D", "P1DT1S", "-P1M", "P1Y2M3DT4H5M6S"] {
        assert!(
            matches!(
                xsd::parse(input, xsd::DURATION),
                Err(xsd::ParseError::InvalidTemporal {
                    datatype: xsd::PrimitiveType::Duration,
                    ..
                })
            ),
            "{input}"
        );
    }
    assert_eq!(xsd::parse_duration("-PT0S").unwrap().to_string(), "PT0S");
}

#[cfg(not(feature = "jiff"))]
#[test]
fn unsigned_alias_does_not_enable_duration_parsing() {
    let duration: xsd::primitive::Duration = core::time::Duration::new(1, 1);
    assert_eq!(duration.as_nanos(), 1_000_000_001);
    for input in ["PT1S", "-PT1S", "PT0S"] {
        assert!(matches!(
            xsd::parse(input, xsd::DURATION),
            Err(xsd::ParseError::UnsupportedDatatype(datatype)) if datatype == xsd::DURATION
        ));
    }
}
