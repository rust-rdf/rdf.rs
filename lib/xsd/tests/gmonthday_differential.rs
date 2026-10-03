#![cfg(any(feature = "oxrdf", feature = "rudof"))]

#[test]
fn calendar_combinations_agree_with_oxsdatatypes() {
    for month in 0..=13 {
        for day in 0..=32 {
            for suffix in [
                "", "Z", "+00:00", "-00:00", "+14:00", "-14:00", "+05:30", "-00:01",
            ] {
                let input = format!("--{month:02}-{day:02}{suffix}");
                let ours = xsd::parse_g_month_day(&input);
                let reference = input.parse::<oxsdatatypes::GMonthDay>();
                // XSD 1.1 §3.3.12 explicitly permits February 29.
                // oxsdatatypes 0.2.3 rejects it; the standard is authoritative.
                // https://www.w3.org/TR/xmlschema11-2/#gMonthDay
                if month == 2 && day == 29 {
                    let ours = ours.unwrap();
                    assert!(
                        reference.is_err(),
                        "review the resolved reference limitation"
                    );
                    assert_eq!(xsd::parse_g_month_day(ours.to_string()).unwrap(), ours);
                    continue;
                }
                assert_eq!(ours.is_ok(), reference.is_ok(), "{input}");
                if let (Ok(ours), Ok(reference)) = (ours, reference) {
                    assert_eq!(ours.to_string(), reference.to_string(), "{input}");
                    assert_eq!(
                        ours.to_string().parse::<oxsdatatypes::GMonthDay>().unwrap(),
                        reference
                    );
                    assert_eq!(xsd::parse_g_month_day(reference.to_string()).unwrap(), ours);
                }
            }
        }
    }
}

#[test]
fn invalid_lexical_forms_are_rejected_by_both_parsers() {
    for input in [
        "--2-29",
        "--02-9",
        "--0229",
        "--02-029",
        "--02-29z",
        "--02-29+14:01",
        "--02-29-14:01",
        "--02-29+00:60",
        "--02-29+0100",
        "--02-29Zjunk",
        "--０2-29",
        " --02-29",
    ] {
        assert!(xsd::parse_g_month_day(input).is_err(), "{input}");
        assert!(input.parse::<oxsdatatypes::GMonthDay>().is_err(), "{input}");
    }
}
