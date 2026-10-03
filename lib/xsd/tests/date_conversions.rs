#![cfg(all(feature = "jiff", any(feature = "serde", feature = "bson")))]

use xsd::{PrimitiveValue, TimezoneOffset, Value, primitive::Date};

fn dates() -> impl Iterator<Item = Date> {
    [-9999, -1, 0, 1, 9999].into_iter().flat_map(|year| {
        [None, Some(-840), Some(0), Some(345), Some(840)]
            .into_iter()
            .map(move |minutes| {
                Date::new(year, 2, 28)
                    .unwrap()
                    .with_timezone(minutes.map(|m| TimezoneOffset::from_minutes(m).unwrap()))
            })
    })
}

#[cfg(feature = "serde")]
#[test]
fn serde_date_round_trips_preserve_structure_and_datatype() {
    for date in dates() {
        let primitive = PrimitiveValue::from(date);
        let value = Value::from(date);
        let json = serde_json::to_value(&value).unwrap();
        assert_eq!(serde_json::from_value::<Value>(json).unwrap(), value);
        let json = serde_json::to_value(&primitive).unwrap();
        assert_eq!(
            serde_json::from_value::<PrimitiveValue>(json.clone()).unwrap(),
            primitive
        );
        assert_eq!(
            json["Date"]["timezone"],
            serde_json::to_value(date.timezone()).unwrap()
        );
        assert_eq!(
            serde_json::from_value::<Date>(json["Date"].clone()).unwrap(),
            date
        );
    }
    // The enum tag survives, but the old alias's bare-string payload is replaced.
    assert!(
        serde_json::from_value::<PrimitiveValue>(serde_json::json!({"Date": "2026-01-02"}))
            .is_err()
    );
    for civil in [
        "2026-02-29",
        "2026-01-02T12:00:00",
        "2026-01-02[UTC]",
        "2026-01-02Z",
        "2026-01-02+02:00",
    ] {
        assert!(
            serde_json::from_value::<Date>(serde_json::json!({"civil": civil, "timezone": null}))
                .is_err(),
            "{civil}"
        );
    }
}

#[cfg(feature = "serde")]
#[test]
fn explicit_date_json_requires_the_datatype_for_round_trips() {
    for date in dates() {
        let primitive = PrimitiveValue::from(date);
        let value = Value::from(date);
        let json = serde_json::Value::String(date.to_string());
        assert_eq!(primitive.to_json(), Some(json.clone()));
        assert_eq!(primitive.clone().into_json(), json);
        assert_eq!(value.to_json(), Some(json.clone()));
        assert_eq!(value.clone().into_json(), json);
        assert_eq!(
            xsd::parse(json.as_str().unwrap(), xsd::DATE).unwrap(),
            value
        );
        assert_ne!(serde_json::to_value(primitive).unwrap(), json);
    }
}

#[cfg(feature = "bson")]
#[test]
fn explicit_date_bson_preserves_timezone_and_calendar_fields() {
    for date in dates() {
        let primitive = PrimitiveValue::from(date);
        let value = Value::from(date);
        let bson = bson::Bson::String(date.to_string());
        assert_eq!(primitive.to_bson(), Some(bson.clone()));
        assert_eq!(primitive.into_bson(), bson);
        assert_eq!(value.to_bson(), Some(bson.clone()));
        assert_eq!(value.clone().into_bson(), bson);
        assert_eq!(
            xsd::parse(bson.as_str().unwrap(), xsd::DATE).unwrap(),
            value
        );
    }
}
