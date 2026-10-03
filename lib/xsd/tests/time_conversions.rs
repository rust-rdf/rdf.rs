#![cfg(all(feature = "jiff", any(feature = "serde", feature = "bson")))]

use xsd::{PrimitiveValue, TimezoneOffset, Value, primitive::Time};

fn times() -> impl Iterator<Item = Time> {
    [0, 1, 125_000_000, 999_999_999]
        .into_iter()
        .flat_map(|nanosecond| {
            [None, Some(-840), Some(0), Some(345), Some(840)]
                .into_iter()
                .map(move |minutes| {
                    Time::new(23, 59, 59, nanosecond)
                        .unwrap()
                        .with_timezone(minutes.map(|m| TimezoneOffset::from_minutes(m).unwrap()))
                })
        })
}

#[cfg(feature = "serde")]
#[test]
fn serde_time_round_trips_preserve_structure_and_datatype() {
    for time in times() {
        let primitive = PrimitiveValue::from(time);
        let value = Value::from(time);
        let json = serde_json::to_value(&value).unwrap();
        assert_eq!(serde_json::from_value::<Value>(json).unwrap(), value);
        let json = serde_json::to_value(&primitive).unwrap();
        assert_eq!(
            serde_json::from_value::<PrimitiveValue>(json.clone()).unwrap(),
            primitive
        );
        assert_eq!(
            json["Time"]["timezone"],
            serde_json::to_value(time.timezone()).unwrap()
        );
        assert_eq!(
            serde_json::from_value::<Time>(json["Time"].clone()).unwrap(),
            time
        );
    }
    assert!(
        serde_json::from_value::<PrimitiveValue>(serde_json::json!({"Time": "12:34:56"})).is_err()
    );
    for minutes in [-841, 841, 32768] {
        let json = serde_json::json!({"Time": {"civil": "12:34:56", "timezone": minutes}});
        assert!(serde_json::from_value::<PrimitiveValue>(json).is_err());
    }
}

#[cfg(feature = "serde")]
#[test]
fn explicit_time_json_preserves_nanoseconds_but_not_lexical_spelling() {
    for time in times() {
        let primitive = PrimitiveValue::from(time);
        let value = Value::from(time);
        let json = serde_json::Value::String(time.to_string());
        assert_eq!(primitive.to_json(), Some(json.clone()));
        assert_eq!(primitive.clone().into_json(), json);
        assert_eq!(value.to_json(), Some(json.clone()));
        assert_eq!(value.clone().into_json(), json);
        assert_eq!(
            xsd::parse(json.as_str().unwrap(), xsd::TIME).unwrap(),
            value
        );
        assert_ne!(serde_json::to_value(primitive).unwrap(), json);
    }
    let value = xsd::parse_time("24:00:00.000-00:00").unwrap();
    assert_eq!(value.into_json(), serde_json::json!("00:00:00Z"));
    let absent = xsd::parse_time("00:00:00").unwrap().into_json();
    let utc = xsd::parse_time("00:00:00Z").unwrap().into_json();
    assert_ne!(absent, utc);
}

#[cfg(feature = "bson")]
#[test]
fn explicit_time_bson_preserves_nanoseconds_and_timezone() {
    for time in times() {
        let primitive = PrimitiveValue::from(time);
        let value = Value::from(time);
        let bson = bson::Bson::String(time.to_string());
        assert_eq!(primitive.to_bson(), Some(bson.clone()));
        assert_eq!(primitive.into_bson(), bson);
        assert_eq!(value.to_bson(), Some(bson.clone()));
        assert_eq!(value.clone().into_bson(), bson);
        assert_eq!(
            xsd::parse(bson.as_str().unwrap(), xsd::TIME).unwrap(),
            value
        );
    }
    assert_eq!(
        xsd::parse_time("24:00:00.000-00:00").unwrap().into_bson(),
        bson::Bson::String("00:00:00Z".into())
    );
}
