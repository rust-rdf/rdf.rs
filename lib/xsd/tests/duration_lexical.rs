#![cfg(feature = "jiff")]

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
