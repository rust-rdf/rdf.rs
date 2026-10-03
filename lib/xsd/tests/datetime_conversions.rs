#![cfg(all(feature = "jiff", any(feature = "serde", feature = "bson")))]

use xsd::{PrimitiveValue, TimezoneOffset, Value, primitive::DateTime};

fn values() -> impl Iterator<Item = DateTime> {
    [-9999, -1, 0, 1, 9999].into_iter().flat_map(|year| {
        [0, 1, 999_999_999].into_iter().flat_map(move |ns| {
            [None, Some(-840), Some(0), Some(345), Some(840)]
                .into_iter()
                .map(move |minutes| {
                    DateTime::new(year, 2, 28, 23, 59, 59, ns)
                        .unwrap()
                        .with_timezone(minutes.map(|m| TimezoneOffset::from_minutes(m).unwrap()))
                })
        })
    })
}

#[cfg(feature = "serde")]
#[test]
fn serde_datetime_round_trips_preserve_structure_and_datatype() {
    for datetime in values() {
        let value = Value::from(datetime);
        assert_eq!(
            serde_json::from_value::<Value>(serde_json::to_value(&value).unwrap()).unwrap(),
            value
        );
        let primitive = PrimitiveValue::from(datetime);
        let json = serde_json::to_value(&primitive).unwrap();
        assert_eq!(
            serde_json::from_value::<PrimitiveValue>(json.clone()).unwrap(),
            primitive
        );
        assert_eq!(
            json["DateTime"]["timezone"],
            serde_json::to_value(datetime.timezone()).unwrap()
        );
        assert_eq!(
            serde_json::from_value::<DateTime>(json["DateTime"].clone()).unwrap(),
            datetime
        );
    }
    // Preserve the enum tag but reject the replaced alias's bare-string payload.
    assert!(
        serde_json::from_value::<PrimitiveValue>(
            serde_json::json!({"DateTime": "2026-01-02T12:34:56"})
        )
        .is_err()
    );
    for minutes in [-841, 841, 32768] {
        assert!(serde_json::from_value::<PrimitiveValue>(serde_json::json!({"DateTime": {"civil": "2026-01-02T12:34:56", "timezone": minutes}})).is_err());
    }
}

#[cfg(feature = "serde")]
#[test]
fn explicit_datetime_json_preserves_values_without_lexical_identity() {
    for datetime in values() {
        let value = Value::from(datetime);
        let primitive = PrimitiveValue::from(datetime);
        let json = serde_json::json!(datetime.to_string());
        assert_eq!(primitive.to_json(), Some(json.clone()));
        assert_eq!(primitive.clone().into_json(), json);
        assert_eq!(value.to_json(), Some(json.clone()));
        assert_eq!(value.clone().into_json(), json);
        assert_eq!(
            xsd::parse(json.as_str().unwrap(), xsd::DATE_TIME).unwrap(),
            value
        );
        assert_ne!(serde_json::to_value(primitive).unwrap(), json);
    }
    assert_eq!(
        xsd::parse_datetime("-0001-12-31T24:00:00.000-00:00")
            .unwrap()
            .into_json(),
        serde_json::json!("0000-01-01T00:00:00Z")
    );
}

#[cfg(feature = "bson")]
#[test]
fn explicit_datetime_bson_preserves_nanoseconds_and_timezone() {
    for datetime in values() {
        let value = Value::from(datetime);
        let primitive = PrimitiveValue::from(datetime);
        let bson = bson::Bson::String(datetime.to_string());
        assert_eq!(primitive.to_bson(), Some(bson.clone()));
        assert_eq!(primitive.into_bson(), bson);
        assert_eq!(value.to_bson(), Some(bson.clone()));
        assert_eq!(value.clone().into_bson(), bson);
        assert_eq!(
            xsd::parse(bson.as_str().unwrap(), xsd::DATE_TIME).unwrap(),
            value
        );
    }
}
