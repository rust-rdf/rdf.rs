#![cfg(feature = "jiff")]

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
