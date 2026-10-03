#![cfg(feature = "serde")]

use serde_json::{from_value, json, to_value};
use xsd::{PrimitiveValue, TimezoneOffset, Value, primitive::GMonth};

#[test]
fn serde_preserves_validated_fields_and_enum_tags() {
    for month in 1..=12 {
        for minutes in [None, Some(-840), Some(0), Some(330), Some(840)] {
            let value = GMonth::new(month)
                .unwrap()
                .with_timezone(minutes.map(|m| TimezoneOffset::from_minutes(m).unwrap()));
            let payload = json!({"month": month, "timezone": minutes});
            assert_eq!(to_value(value).unwrap(), payload);
            assert_eq!(from_value::<GMonth>(payload.clone()).unwrap(), value);
            let primitive = PrimitiveValue::GMonth(value);
            let tagged = json!({"GMonth": payload});
            assert_eq!(to_value(&primitive).unwrap(), tagged);
            assert_eq!(
                from_value::<PrimitiveValue>(tagged.clone()).unwrap(),
                primitive
            );
            let wrapped = Value::from(primitive);
            let tagged = json!({"Primitive": tagged});
            assert_eq!(to_value(&wrapped).unwrap(), tagged);
            assert_eq!(from_value::<Value>(tagged).unwrap(), wrapped);
        }
    }
}

#[test]
fn serde_rejects_invalid_fields_and_legacy_payloads() {
    for payload in [
        json!(1),
        json!({"month": 0}),
        json!({"month": 13}),
        json!({"month": 255}),
        json!({"month": -1}),
        json!({"month": 1.5}),
        json!({"timezone": 0}),
        json!({"month": 1, "timezone": 841}),
        json!({"month": 1, "timezone": -841}),
    ] {
        assert!(from_value::<GMonth>(payload.clone()).is_err(), "{payload}");
        assert!(from_value::<PrimitiveValue>(json!({"GMonth": payload})).is_err());
    }
    let missing_offset: GMonth = from_value(json!({"month": 1})).unwrap();
    assert_eq!(missing_offset.timezone(), None);
}
