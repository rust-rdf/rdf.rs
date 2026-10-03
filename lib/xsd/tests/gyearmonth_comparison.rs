use std::collections::{BTreeSet, HashSet, hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};
use xsd::{TimezoneOffset, primitive::GYearMonth};

#[test]
fn collections_preserve_signed_years_months_and_timezones() {
    let mut values = Vec::new();
    for year in [i32::MIN, -1, 0, 1, i32::MAX] {
        for month in [1, 12] {
            for minutes in [None, Some(-840), Some(0), Some(840)] {
                values.push(
                    GYearMonth::new(year, month)
                        .unwrap()
                        .with_timezone(minutes.map(|m| TimezoneOffset::from_minutes(m).unwrap())),
                );
            }
        }
    }
    assert!(values.windows(2).all(|pair| pair[0] < pair[1]));
    assert_eq!(HashSet::<_>::from_iter(values.iter().copied()).len(), 40);
    assert_eq!(
        BTreeSet::<_>::from_iter(values.iter().copied())
            .into_iter()
            .collect::<Vec<_>>(),
        values
    );
    for value in values {
        let parsed = xsd::parse_g_year_month(value.to_string()).unwrap();
        let xsd::Value::Primitive(xsd::PrimitiveValue::GYearMonth(parsed)) = parsed else {
            panic!("wrong datatype")
        };
        assert_eq!(value, parsed);
        assert_eq!(value.cmp(&parsed), core::cmp::Ordering::Equal);
        let hash = |value: GYearMonth| {
            let mut state = DefaultHasher::new();
            value.hash(&mut state);
            state.finish()
        };
        assert_eq!(hash(value), hash(parsed));
    }
}
