use std::collections::{BTreeSet, HashSet, hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};
use xsd::{TimezoneOffset, primitive::GMonth};

#[test]
fn collections_preserve_month_and_timezone_distinctions() {
    let mut values = Vec::new();
    for month in 1..=12 {
        for minutes in [None, Some(-840), Some(0), Some(840)] {
            values.push(
                GMonth::new(month)
                    .unwrap()
                    .with_timezone(minutes.map(|m| TimezoneOffset::from_minutes(m).unwrap())),
            );
        }
    }
    assert!(values.windows(2).all(|pair| pair[0] < pair[1]));
    assert_eq!(HashSet::<_>::from_iter(values.iter().copied()).len(), 48);
    assert_eq!(
        BTreeSet::<_>::from_iter(values.iter().copied())
            .into_iter()
            .collect::<Vec<_>>(),
        values
    );
    for value in values {
        let parsed = xsd::parse_g_month(value.to_string()).unwrap();
        let xsd::Value::Primitive(xsd::PrimitiveValue::GMonth(parsed)) = parsed else {
            panic!("wrong datatype")
        };
        assert_eq!(value, parsed);
        assert_eq!(value.cmp(&parsed), core::cmp::Ordering::Equal);
        let hash = |value: GMonth| {
            let mut state = DefaultHasher::new();
            value.hash(&mut state);
            state.finish()
        };
        assert_eq!(hash(value), hash(parsed));
    }
}
