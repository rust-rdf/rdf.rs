use std::collections::{BTreeSet, HashSet, hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};
use xsd::{TimezoneOffset, primitive::GDay};

#[test]
fn collections_preserve_day_and_timezone_distinctions() {
    let mut values = Vec::new();
    for day in 1..=31 {
        for minutes in [None, Some(-840), Some(0), Some(840)] {
            values.push(
                GDay::new(day)
                    .unwrap()
                    .with_timezone(minutes.map(|m| TimezoneOffset::from_minutes(m).unwrap())),
            );
        }
    }
    assert!(values.windows(2).all(|pair| pair[0] < pair[1]));
    assert_eq!(HashSet::<_>::from_iter(values.iter().copied()).len(), 124);
    assert_eq!(
        BTreeSet::<_>::from_iter(values.iter().copied())
            .into_iter()
            .collect::<Vec<_>>(),
        values
    );
    for value in values {
        let parsed = xsd::parse_g_day(value.to_string()).unwrap();
        let xsd::Value::Primitive(xsd::PrimitiveValue::GDay(parsed)) = parsed else {
            panic!("wrong datatype")
        };
        assert_eq!(value, parsed);
        assert_eq!(value.cmp(&parsed), core::cmp::Ordering::Equal);
        let hash = |value: GDay| {
            let mut state = DefaultHasher::new();
            value.hash(&mut state);
            state.finish()
        };
        assert_eq!(hash(value), hash(parsed));
    }
}
