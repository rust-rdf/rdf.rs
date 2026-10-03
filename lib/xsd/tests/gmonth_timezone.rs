use xsd::{PrimitiveValue, TimezoneOffset, Value, primitive::GMonth};

#[test]
fn every_month_and_offset_round_trips() {
    for month in 1..=12 {
        let bare = GMonth::new(month).unwrap();
        assert_eq!(
            xsd::parse_g_month(bare.to_string()).unwrap(),
            Value::from(PrimitiveValue::GMonth(bare))
        );
        for minutes in -840..=840 {
            let offset = TimezoneOffset::from_minutes(minutes).unwrap();
            let value = bare.with_timezone(Some(offset));
            let lexical = value.to_string();
            let parsed = xsd::parse(&lexical, xsd::G_MONTH).unwrap();
            assert_eq!(parsed, Value::from(PrimitiveValue::GMonth(value)));
            assert_eq!(parsed.r#type(), xsd::G_MONTH);
            assert_eq!(value.timezone(), Some(offset));
            assert_eq!(value.with_timezone(None), bare);
        }
    }
    for input in ["--01Z", "--01+00:00", "--01-00:00"] {
        assert_eq!(xsd::parse_g_month(input).unwrap().to_string(), "--01Z");
    }
}

#[test]
fn malformed_suffixes_are_rejected_without_panics() {
    for suffix in [
        "z",
        "+14:01",
        "-14:01",
        "+15:00",
        "+00:60",
        "+01",
        "+0100",
        "+01:00:00",
        "ZZ",
        "Z+01:00",
        "[UTC]",
        " ",
        "é",
        "+０1:00",
    ] {
        assert!(
            xsd::parse_g_month(format!("--01{suffix}")).is_err(),
            "{suffix}"
        );
    }
}
