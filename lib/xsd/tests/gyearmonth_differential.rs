#![cfg(any(feature = "oxrdf", feature = "rudof"))]

#[test]
fn year_month_fields_agree_with_oxsdatatypes() {
    for year in [
        "-2147483648",
        "-10000",
        "-0001",
        "0000",
        "0001",
        "10000",
        "2147483647",
    ] {
        for month in 0..=13 {
            for suffix in ["", "Z", "+00:00", "-00:00", "+14:00", "-14:00", "+05:30"] {
                let input = format!("{year}-{month:02}{suffix}");
                let ours = xsd::parse_g_year_month(&input);
                let reference = input.parse::<oxsdatatypes::GYearMonth>();
                assert_eq!(ours.is_ok(), reference.is_ok(), "{input}");
                if let (Ok(ours), Ok(reference)) = (ours, reference) {
                    assert_eq!(ours.to_string(), reference.to_string(), "{input}");
                    assert_eq!(
                        ours.to_string()
                            .parse::<oxsdatatypes::GYearMonth>()
                            .unwrap(),
                        reference
                    );
                    assert_eq!(
                        xsd::parse_g_year_month(reference.to_string()).unwrap(),
                        ours
                    );
                }
            }
        }
    }
}

#[test]
fn lexical_rules_and_bounded_year_policy_are_explicit() {
    for input in [
        "2026-1",
        "202601",
        "2026-001",
        "+2026-01",
        "02026-01",
        "2026-01z",
        "2026-01+14:01",
        "2026-01-14:01",
        "2026-01+00:60",
        "2026-01+0100",
        "2026-01Zjunk",
        "2026-０1",
        " 2026-01",
    ] {
        assert!(xsd::parse_g_year_month(input).is_err(), "{input}");
        assert!(
            input.parse::<oxsdatatypes::GYearMonth>().is_err(),
            "{input}"
        );
    }
    for input in ["2147483648-01Z", "-2147483649-12Z"] {
        assert!(input.parse::<oxsdatatypes::GYearMonth>().is_ok());
        assert_eq!(
            xsd::parse_g_year_month(input).unwrap_err(),
            xsd::ParseCalendarError::OutOfRange
        );
    }
}

#[test]
fn negative_zero_years_follow_xsd_11_lexical_mapping() {
    // https://www.w3.org/TR/xmlschema11-2/#nt-yearFrag
    // https://www.w3.org/TR/xmlschema11-2/#f-yearFragValue
    for suffix in ["", "Z", "+14:00", "-14:00", "+00:00", "-00:00"] {
        let input = format!("-0000-01{suffix}");
        let ours = xsd::parse_g_year_month(&input).unwrap();
        let reference = input.parse::<oxsdatatypes::GYearMonth>().unwrap();
        assert_eq!(ours.to_string(), reference.to_string());
        assert_eq!(
            ours,
            xsd::parse_g_year_month(format!("0000-01{suffix}")).unwrap()
        );
        assert_eq!(xsd::parse(&input, xsd::G_YEAR_MONTH).unwrap(), ours);
    }
}
