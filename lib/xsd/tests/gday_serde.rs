#![cfg(feature = "serde")]

use serde_json::{from_value, json, to_value};
use xsd::{PrimitiveValue, TimezoneOffset, Value, primitive::GDay};

#[test]
fn serde_preserves_validated_fields_and_enum_tags() {
    for day in 1..=31 {
        for minutes in [None, Some(-840), Some(0), Some(330), Some(840)] {
            let value = GDay::new(day)
                .unwrap()
                .with_timezone(minutes.map(|m| TimezoneOffset::from_minutes(m).unwrap()));
            let payload = json!({"day": day, "timezone": minutes});
            assert_eq!(to_value(value).unwrap(), payload);
            assert_eq!(from_value::<GDay>(payload.clone()).unwrap(), value);
            let primitive = PrimitiveValue::GDay(value);
            let tagged = json!({"GDay": payload});
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
        json!({"day": 0}),
        json!({"day": 32}),
        json!({"day": 255}),
        json!({"day": -1}),
        json!({"day": 1.5}),
        json!({"timezone": 0}),
        json!({"day": 1, "timezone": 841}),
        json!({"day": 1, "timezone": -841}),
    ] {
        assert!(from_value::<GDay>(payload.clone()).is_err(), "{payload}");
        assert!(from_value::<PrimitiveValue>(json!({"GDay": payload.clone()})).is_err());
        assert!(from_value::<Value>(json!({"Primitive": {"GDay": payload}})).is_err());
    }
    let missing_offset: GDay = from_value(json!({"day": 31})).unwrap();
    assert_eq!(missing_offset.timezone(), None);
}
