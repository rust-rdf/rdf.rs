#![cfg(feature = "jiff")]

use xsd::{PrimitiveValue, TimezoneOffset, Value, primitive::DateTime};

#[test]
fn datetime_retains_offset_in_formatting_and_components() {
    for year in [-9999, -1, 0, 1, 9999] {
        for nanosecond in [0, 1, 999_999_999] {
            let civil = DateTime::new(year, 2, 28, 23, 59, 59, nanosecond).unwrap();
            assert_eq!(civil.timezone(), None);
            for minutes in [-840, -330, 0, 345, 840] {
                let offset = TimezoneOffset::from_minutes(minutes).unwrap();
                let value = civil.with_timezone(Some(offset));
                assert_eq!(value.civil(), civil.civil());
                assert_eq!(value.date().timezone(), Some(offset));
                assert_eq!(value.time().timezone(), Some(offset));
                assert_eq!(value.date().civil(), civil.civil().date());
                assert_eq!(value.time().civil(), civil.civil().time());
                assert_ne!(value, civil);
                assert_eq!(value.with_timezone(None), civil);
                let expected = format!("{civil}{offset}");
                assert_eq!(value.to_string(), expected);
                assert_eq!(Value::from(value).to_string(), expected);
                assert_eq!(PrimitiveValue::from(value).to_string(), expected);
            }
        }
    }
    assert!(DateTime::new(2026, 2, 29, 0, 0, 0, 0).is_err());
    assert!(DateTime::new(0, 2, 29, 0, 0, 0, 0).is_ok());
}

#[cfg(feature = "serde")]
#[test]
fn datetime_struct_encoding_validates_fields() {
    let value = DateTime::new(2026, 1, 2, 12, 34, 56, 1)
        .unwrap()
        .with_timezone(Some(TimezoneOffset::UTC));
    let json = serde_json::json!({"civil": "2026-01-02T12:34:56.000000001", "timezone": 0});
    assert_eq!(serde_json::to_value(value).unwrap(), json);
    assert_eq!(serde_json::from_value::<DateTime>(json).unwrap(), value);
    for civil in [
        "2026-01-02T12:34:56+02:00",
        "2026-01-02T23:59:60",
        "2026-01-02T12:34:56[UTC]",
    ] {
        assert!(
            serde_json::from_value::<DateTime>(
                serde_json::json!({"civil": civil, "timezone": null})
            )
            .is_err(),
            "{civil}"
        );
    }
    assert!(
        serde_json::from_value::<DateTime>(
            serde_json::json!({"civil": "2026-01-02T12:34:56", "timezone": 841})
        )
        .is_err()
    );
}
