#![cfg(feature = "serde")]

use serde_json::{from_value, json, to_value};
use xsd::{PrimitiveValue, TimezoneOffset, Value, primitive::GMonthDay};

#[test]
fn serde_preserves_validated_fields_and_enum_tags() {
    for (month, day) in [(1, 1), (2, 29), (4, 30), (12, 31)] {
        for minutes in [None, Some(-840), Some(0), Some(330), Some(840)] {
            let value = GMonthDay::new(month, day)
                .unwrap()
                .with_timezone(minutes.map(|m| TimezoneOffset::from_minutes(m).unwrap()));
            let payload = json!({"month": month, "day": day, "timezone": minutes});
            assert_eq!(to_value(value).unwrap(), payload);
            assert_eq!(from_value::<GMonthDay>(payload.clone()).unwrap(), value);
            let primitive = PrimitiveValue::GMonthDay(value);
            let tagged = json!({"GMonthDay": payload});
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
fn serde_validates_combinations_not_just_individual_fields() {
    for month in 0..=13 {
        for day in 0..=32 {
            let expected = GMonthDay::new(month, day);
            let payload = json!({"month": month, "day": day});
            assert_eq!(from_value::<GMonthDay>(payload).ok(), expected);
        }
    }
    for payload in [
        json!([2, 29]),
        json!({"month": 2, "day": 30}),
        json!({"month": 4, "day": 31}),
        json!({"month": 1, "day": 255}),
        json!({"month": 1, "day": -1}),
        json!({"month": 1, "day": 1.5}),
        json!({"day": 1}),
        json!({"month": 1}),
        json!({"month": 2, "day": 29, "timezone": 841}),
        json!({"month": 2, "day": 29, "timezone": -841}),
    ] {
        assert!(
            from_value::<GMonthDay>(payload.clone()).is_err(),
            "{payload}"
        );
        assert!(from_value::<PrimitiveValue>(json!({"GMonthDay": payload.clone()})).is_err());
        assert!(from_value::<Value>(json!({"Primitive": {"GMonthDay": payload}})).is_err());
    }
}
