#![cfg(feature = "jiff")]

use xsd::{PrimitiveValue, TimezoneOffset, Value, primitive::Date};

#[test]
fn date_parser_preserves_offsets_and_signed_years() {
    for year in ["-9999", "-0001", "0000", "0001", "9999"] {
        for suffix in ["", "Z", "+00:00", "-00:00", "+14:00", "-14:00", "+05:45"] {
            let input = format!("{year}-02-28{suffix}");
            let value = xsd::parse(&input, xsd::DATE).unwrap();
            let Value::Primitive(PrimitiveValue::Date(date)) = value.clone() else {
                panic!("wrong datatype");
            };
            let offset = if suffix.is_empty() {
                None
            } else {
                Some(suffix.parse().unwrap())
            };
            assert_eq!(date.timezone(), offset);
            assert_eq!(xsd::parse_date(value.to_string()).unwrap(), value);
        }
    }
}

#[test]
fn date_parser_rejects_invalid_offsets_and_lexical_forms() {
    assert!(xsd::parse_datetime("2026-01-02ZT24:00:00").is_err());
    assert!(xsd::parse_datetime("2026-01-02+02:00T24:00:00").is_err());
    for input in [
        "2026-01-02+14:01",
        "2026-01-02-14:01",
        "2026-01-02+15:00",
        "2026-01-02+01:60",
        "2026-01-02+02",
        "2026-01-02+0200",
        "2026-01-02+02:00:00",
        "2026-01-02z",
        "2026-01-02Z ",
        " 2026-01-02",
        "2026-01-02Zjunk",
        "2026-01-02[UTC]",
        "-0000-01-02Z",
        "+2026-01-02Z",
        "02026-01-02Z",
        "2026-+1-02Z",
        "2026-01-+2Z",
        "2026-01-🦀",
        "2026-02-29Z",
        "2026-01-02T00:00:00Z",
        "10000-01-02Z",
    ] {
        assert!(xsd::parse_date(input).is_err(), "{input}");
        assert!(
            matches!(
                xsd::parse(input, xsd::DATE),
                Err(xsd::ParseError::InvalidTemporal {
                    datatype: xsd::PrimitiveType::Date,
                    ..
                })
            ),
            "{input}"
        );
    }
}

#[test]
fn date_retains_optional_timezone() {
    for year in [-9999, -1, 0, 1, 9999] {
        let civil = Date::new(year, 2, 28).unwrap();
        assert_eq!(civil.timezone(), None);
        for minutes in [-840, -330, 0, 345, 840] {
            let offset = TimezoneOffset::from_minutes(minutes).unwrap();
            let date = civil.with_timezone(Some(offset));
            assert_eq!(date.civil(), civil.civil());
            assert_eq!(date.timezone(), Some(offset));
            assert_ne!(date, civil);
            assert_eq!(date.with_timezone(None), civil);
            let expected = format!("{civil}{offset}");
            assert_eq!(date.to_string(), expected);
            assert_eq!(PrimitiveValue::from(date).to_string(), expected);
            assert_eq!(Value::from(date).to_string(), expected);
        }
    }
    assert!(Date::new(2026, 2, 29).is_err());
    assert!(Date::new(0, 2, 29).is_ok());
}

#[cfg(feature = "serde")]
#[test]
fn date_struct_encoding_validates_offsets() {
    let date = Date::new(2026, 1, 2)
        .unwrap()
        .with_timezone(Some(TimezoneOffset::UTC));
    let json = serde_json::json!({"civil": "2026-01-02", "timezone": 0});
    assert_eq!(serde_json::to_value(date).unwrap(), json);
    assert_eq!(serde_json::from_value::<Date>(json).unwrap(), date);
    assert!(
        serde_json::from_value::<Date>(serde_json::json!({
            "civil": "2026-01-02", "timezone": 841
        }))
        .is_err()
    );
}
