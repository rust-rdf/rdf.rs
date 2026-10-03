//! Differential coverage of the shared, bounded XSD 1.1 date domain.
//! Gregorian validity and year zero follow https://www.w3.org/TR/xmlschema11-2/#date.
//! This does not equate the two libraries' structural/semantic equality policies.
#![cfg(all(feature = "jiff", any(feature = "oxrdf", feature = "rudof")))]

use xsd::{PrimitiveValue, TimezoneOffset, Value};

fn compare(input: &str) {
    let ours = xsd::parse_date(input);
    let reference = input.parse::<oxsdatatypes::Date>();
    assert_eq!(ours.is_ok(), reference.is_ok(), "{input}");
    if let (Ok(ours), Ok(reference)) = (ours, reference) {
        let Value::Primitive(PrimitiveValue::Date(date)) = ours.clone() else {
            panic!("wrong datatype");
        };
        assert_eq!(i64::from(date.year()), reference.year(), "{input}");
        assert_eq!(date.month() as u8, reference.month(), "{input}");
        assert_eq!(date.day() as u8, reference.day(), "{input}");
        assert_eq!(
            date.timezone().map(|o| o.to_string()),
            reference.timezone_offset().map(|o| o.to_string()),
            "{input}"
        );
        assert_eq!(
            xsd::parse_date(reference.to_string()).unwrap(),
            ours,
            "{input}"
        );
        assert!(
            ours.to_string()
                .parse::<oxsdatatypes::Date>()
                .unwrap()
                .is_identical_with(reference),
            "{input}"
        );
    }
}

#[test]
fn date_calendar_grid_matches_independent_parser() {
    for year in [
        "-9999", "-0400", "-0100", "-0004", "-0001", "0000", "0001", "1900", "2000", "2024", "9999",
    ] {
        for month in 0..=13 {
            for day in [0, 1, 28, 29, 30, 31, 32] {
                for suffix in ["", "Z", "+05:45", "-14:00"] {
                    compare(&format!("{year}-{month:02}-{day:02}{suffix}"));
                }
            }
        }
    }
}

#[test]
fn date_all_offsets_match_independent_parser() {
    for minutes in -840..=840 {
        let offset = TimezoneOffset::from_minutes(minutes).unwrap();
        compare(&format!("2024-02-29{offset}"));
    }
    for input in [
        "2024-02-29+00:00",
        "2024-02-29-00:00",
        "2024-02-29+14:01",
        "2024-02-29-14:01",
        "2024-02-29+05:60",
        "2024-02-29+0500",
        "2024-02-29z",
        "2024-02-29Zjunk",
    ] {
        compare(input);
    }
}
