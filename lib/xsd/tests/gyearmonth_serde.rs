#![cfg(feature = "serde")]

use serde_json::{from_value, json, to_value};
use xsd::{PrimitiveValue, TimezoneOffset, Value, primitive::GYearMonth};

#[test]
fn serde_preserves_year_boundaries_offsets_and_enum_tags() {
    for year in [i32::MIN, -10000, -1, 0, 1, 10000, i32::MAX] {
        for month in [1, 12] {
            for minutes in [None, Some(-840), Some(0), Some(330), Some(840)] {
                let value = GYearMonth::new(year, month)
                    .unwrap()
                    .with_timezone(minutes.map(|m| TimezoneOffset::from_minutes(m).unwrap()));
                let payload = json!({"year": year, "month": month, "timezone": minutes});
                assert_eq!(to_value(value).unwrap(), payload);
                assert_eq!(from_value::<GYearMonth>(payload.clone()).unwrap(), value);
                let primitive = PrimitiveValue::GYearMonth(value);
                let tagged = json!({"GYearMonth": payload});
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
}

#[test]
fn serde_rejects_invalid_fields_and_legacy_payloads() {
    for payload in [
        json!([2026, 1]),
        json!({"year": 0, "month": 0}),
        json!({"year": 0, "month": 13}),
        json!({"year": 0, "month": 255}),
        json!({"year": 2147483648i64, "month": 1}),
        json!({"year": -2147483649i64, "month": 1}),
        json!({"year": 1.5, "month": 1}),
        json!({"year": 0}),
        json!({"month": 1}),
        json!({"year": 0, "month": 1, "timezone": 841}),
        json!({"year": 0, "month": 1, "timezone": -841}),
    ] {
        assert!(
            from_value::<GYearMonth>(payload.clone()).is_err(),
            "{payload}"
        );
        assert!(from_value::<PrimitiveValue>(json!({"GYearMonth": payload.clone()})).is_err());
        assert!(from_value::<Value>(json!({"Primitive": {"GYearMonth": payload}})).is_err());
    }
    let missing: GYearMonth = from_value(json!({"year": 0, "month": 1})).unwrap();
    assert_eq!(missing.timezone(), None);
}
