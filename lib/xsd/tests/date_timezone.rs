#![cfg(feature = "jiff")]

use xsd::{PrimitiveValue, TimezoneOffset, Value, primitive::Date};

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
