#![cfg(feature = "jiff")]

#[test]
fn duration_precision_limits_are_explicit() {
    for input in ["PT0.1234567891S", "-PT0.0000000001S", "PT1.0000000000S"] {
        let error = xsd::parse_duration(input).unwrap_err();
        assert!(error.to_string().contains("precision"), "{error}");
        assert!(xsd::parse(input, xsd::DURATION).is_err());
    }
    for input in ["PT0.000000001S", "-PT0.123456789S", "PT1H2M3.999999999S"] {
        let value = xsd::parse_duration(input).unwrap();
        assert_eq!(value.to_string(), input);
        assert_eq!(xsd::parse(input, xsd::DURATION).unwrap(), value);
    }
}

#[test]
fn duration_overflow_returns_errors() {
    for input in ["PT9223372036854775808S", "PT999999999999999999999H"] {
        assert!(xsd::parse_duration(input).is_err(), "accepted {input}");
        assert!(xsd::parse(input, xsd::DURATION).is_err());
    }
}

#[test]
fn duration_allows_only_fractional_seconds() {
    for input in ["PT1.5H", "PT1.0H", "-PT1.5M", "PT1H2.5M", "PT1.S", "PT.5S"] {
        assert!(xsd::parse_duration(input).is_err(), "accepted {input}");
        assert!(xsd::parse(input, xsd::DURATION).is_err());
    }
    for input in ["PT1H30M", "-PT1M30S", "PT1H2M0.5S"] {
        let value = xsd::parse_duration(input).unwrap();
        assert_eq!(xsd::parse(value.to_string(), xsd::DURATION).unwrap(), value);
    }
}

#[test]
fn duration_requires_period_fractions() {
    for input in ["PT1,5S", "-PT0,125S", "PT1H2M3,5S"] {
        assert!(xsd::parse_duration(input).is_err(), "accepted {input}");
        assert!(xsd::parse(input, xsd::DURATION).is_err());
    }
    for input in ["PT1.5S", "-PT0.125S", "PT1H2M3.5S"] {
        let value = xsd::parse_duration(input).unwrap();
        assert_eq!(xsd::parse(value.to_string(), xsd::DURATION).unwrap(), value);
    }
}

#[test]
fn duration_rejects_leading_plus() {
    for input in ["+PT1S", "+PT0S", "+PT1H2M3S"] {
        assert!(xsd::parse_duration(input).is_err(), "accepted {input}");
        assert!(xsd::parse(input, xsd::DURATION).is_err());
    }
    for (input, expected) in [("PT1S", "PT1S"), ("-PT1S", "-PT1S")] {
        assert_eq!(xsd::parse_duration(input).unwrap().to_string(), expected);
    }
}

#[test]
fn duration_requires_xsd_designators() {
    for input in ["pt1s", "Pt1S", "PT1s", "1s", "00:00:01"] {
        assert!(xsd::parse_duration(input).is_err(), "accepted {input}");
        assert!(matches!(
            xsd::parse(input, xsd::DURATION),
            Err(xsd::ParseError::InvalidTemporal {
                datatype: xsd::PrimitiveType::Duration,
                ..
            })
        ));
    }
    for input in ["PT1S", "-PT1S", "PT0S", "PT1H2M3S"] {
        assert!(xsd::parse_duration(input).is_ok(), "rejected {input}");
    }
}
