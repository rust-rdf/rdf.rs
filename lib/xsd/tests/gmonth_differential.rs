#![cfg(any(feature = "oxrdf", feature = "rudof"))]

#[test]
fn month_lexical_values_agree_with_oxsdatatypes() {
    for month in 1..=12 {
        for suffix in [
            "", "Z", "+00:00", "-00:00", "+14:00", "-14:00", "+05:30", "-00:01",
        ] {
            let input = format!("--{month:02}{suffix}");
            let ours = xsd::parse_g_month(&input).unwrap();
            let reference = input.parse::<oxsdatatypes::GMonth>().unwrap();
            assert_eq!(ours.to_string(), reference.to_string(), "{input}");
            assert_eq!(
                ours.to_string().parse::<oxsdatatypes::GMonth>().unwrap(),
                reference
            );
            assert_eq!(xsd::parse_g_month(reference.to_string()).unwrap(), ours);
        }
    }
}

#[test]
fn invalid_months_and_offsets_are_rejected_by_both_parsers() {
    for input in [
        "--00",
        "--13",
        "--99",
        "--1",
        "--001",
        "--01--",
        "--01z",
        "--01+14:01",
        "--01-14:01",
        "--01+00:60",
        "--01+0100",
        "--01Zjunk",
        "--０1",
        " --01",
    ] {
        assert!(xsd::parse_g_month(input).is_err(), "{input}");
        assert!(input.parse::<oxsdatatypes::GMonth>().is_err(), "{input}");
    }
}
