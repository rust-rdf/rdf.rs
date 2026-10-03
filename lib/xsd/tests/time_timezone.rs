#![cfg(feature = "jiff")]

use xsd::{PrimitiveValue, TimezoneOffset, Value, primitive::Time};

#[test]
fn time_retains_optional_timezone_and_nanoseconds() {
    for nanosecond in [0, 1, 125_000_000, 999_999_999] {
        let civil = Time::new(23, 59, 59, nanosecond).unwrap();
        assert_eq!(civil.timezone(), None);
        assert_eq!(civil.subsec_nanosecond(), nanosecond);
        for minutes in [-840, -330, 0, 345, 840] {
            let offset = TimezoneOffset::from_minutes(minutes).unwrap();
            let time = civil.with_timezone(Some(offset));
            assert_eq!(time.civil(), civil.civil());
            assert_eq!(time.timezone(), Some(offset));
            assert_ne!(time, civil);
            assert_eq!(time.with_timezone(None), civil);
            let expected = format!("{civil}{offset}");
            assert_eq!(time.to_string(), expected);
            assert_eq!(PrimitiveValue::from(time).to_string(), expected);
            assert_eq!(Value::from(time).to_string(), expected);
        }
    }
    for (h, m, s, ns) in [
        (24, 0, 0, 0),
        (0, 60, 0, 0),
        (0, 0, 60, 0),
        (0, 0, 0, -1),
        (0, 0, 0, 1_000_000_000),
    ] {
        assert!(Time::new(h, m, s, ns).is_err());
    }
}

#[cfg(feature = "serde")]
#[test]
fn time_struct_encoding_validates_fields_and_offsets() {
    let time = Time::new(12, 34, 56, 1)
        .unwrap()
        .with_timezone(Some(TimezoneOffset::UTC));
    let json = serde_json::json!({"civil": "12:34:56.000000001", "timezone": 0});
    assert_eq!(serde_json::to_value(time).unwrap(), json);
    assert_eq!(serde_json::from_value::<Time>(json).unwrap(), time);
    assert!(
        serde_json::from_value::<Time>(serde_json::json!({"civil": "12:34:56", "timezone": 841}))
            .is_err()
    );
    for civil in [
        "12:34:56+02:00",
        "12:34:56[UTC]",
        "23:59:60",
        "2026-01-02T12:34:56",
    ] {
        assert!(
            serde_json::from_value::<Time>(serde_json::json!({"civil": civil, "timezone": null}))
                .is_err(),
            "{civil}"
        );
    }
}
