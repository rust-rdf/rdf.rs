//! Differential coverage of XSD 1.1 times within the shared nanosecond domain.
//! End-of-day semantics follow https://www.w3.org/TR/xmlschema11-2/#time.
#![cfg(all(feature = "jiff", any(feature = "oxrdf", feature = "rudof")))]

use xsd::{PrimitiveValue, TimezoneOffset, Value};

fn compare(input: &str) {
    let ours = xsd::parse_time(input);
    let reference = input.parse::<oxsdatatypes::Time>();
    assert_eq!(ours.is_ok(), reference.is_ok(), "{input}");
    if let (Ok(ours), Ok(reference)) = (ours, reference) {
        let Value::Primitive(PrimitiveValue::Time(time)) = ours.clone() else {
            panic!("wrong datatype");
        };
        assert_eq!(
            time.timezone().map(|o| o.to_string()),
            reference.timezone_offset().map(|o| o.to_string()),
            "{input}"
        );
        assert_eq!(
            xsd::parse_time(reference.to_string()).unwrap(),
            ours,
            "{input}"
        );
        assert!(
            ours.to_string()
                .parse::<oxsdatatypes::Time>()
                .unwrap()
                .is_identical_with(reference),
            "{input}"
        );
    }
}

#[test]
fn time_clock_grid_matches_independent_parser() {
    for hour in 0..=25 {
        for minute in [0, 1, 59, 60] {
            for second in [0, 1, 59, 60] {
                for fraction in ["", ".0", ".000000001", ".999999999"] {
                    for suffix in ["", "Z", "+05:45", "-14:00"] {
                        compare(&format!(
                            "{hour:02}:{minute:02}:{second:02}{fraction}{suffix}"
                        ));
                    }
                }
            }
        }
    }
}

#[test]
fn time_all_offsets_match_independent_parser() {
    for minutes in -840..=840 {
        let offset = TimezoneOffset::from_minutes(minutes).unwrap();
        for clock in ["12:34:56.123456789", "24:00:00.000"] {
            compare(&format!("{clock}{offset}"));
        }
    }
    for input in [
        "24:00:00+00:00",
        "24:00:00-00:00",
        "12:34:56+14:01",
        "12:34:56-14:01",
        "12:34:56+05:60",
        "12:34:56+0500",
        "12:34:56z",
        "12:34:56Zjunk",
    ] {
        compare(input);
    }
}

#[test]
fn nanosecond_limit_is_a_representation_limit_not_an_xsd_rule() {
    // These are valid XSD values in the reference's higher-precision domain.
    // Our documented policy rejects excess digits rather than rounding them.
    for input in ["12:34:56.0000000001Z", "12:34:56.0000000000+02:00"] {
        assert!(input.parse::<oxsdatatypes::Time>().is_ok());
        assert!(
            xsd::parse_time(input)
                .unwrap_err()
                .to_string()
                .contains("nanosecond precision")
        );
    }
}
