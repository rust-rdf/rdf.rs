//! Differential coverage of the shared XSD 1.1 dateTime domain.
//! Calendar and end-of-day semantics follow https://www.w3.org/TR/xmlschema11-2/#dateTime.
#![cfg(all(feature = "jiff", any(feature = "oxrdf", feature = "rudof")))]

use xsd::{PrimitiveValue, TimezoneOffset, Value};

fn compare(input: &str) {
    let ours = xsd::parse_datetime(input);
    let reference = input.parse::<oxsdatatypes::DateTime>();
    assert_eq!(ours.is_ok(), reference.is_ok(), "{input}");
    if let (Ok(ours), Ok(reference)) = (ours, reference) {
        let Value::Primitive(PrimitiveValue::DateTime(datetime)) = ours.clone() else {
            panic!("wrong datatype");
        };
        assert_eq!(
            datetime.timezone().map(|o| o.to_string()),
            reference.timezone_offset().map(|o| o.to_string()),
            "{input}"
        );
        // oxsdatatypes 0.2.3's formatter can carry fractional rounding into the
        // calendar (e.g. -9999-01-01T23:59:59.999999999). Do not use that spelling
        // as an oracle for fractional values; reparse our output in the reference
        // and compare its represented value instead.
        if datetime.subsec_nanosecond() == 0 {
            assert_eq!(
                xsd::parse_datetime(reference.to_string()).unwrap(),
                ours,
                "{input}"
            );
        }
        assert!(
            ours.to_string()
                .parse::<oxsdatatypes::DateTime>()
                .unwrap()
                .is_identical_with(reference),
            "{input}"
        );
    }
}

#[test]
fn datetime_calendar_and_clock_grid_matches_independent_parser() {
    for year in [
        "-9999", "-0400", "-0100", "-0001", "0000", "0001", "1900", "2000", "9999",
    ] {
        for month in [0, 1, 2, 4, 13] {
            for day in [0, 1, 28, 29, 30, 31, 32] {
                for clock in [
                    "00:00:00",
                    "23:59:59.999999999",
                    "23:59:60",
                    "24:00:00.000",
                    "24:00:01",
                ] {
                    for suffix in ["", "Z", "+05:45", "-14:00"] {
                        compare(&format!("{year}-{month:02}-{day:02}T{clock}{suffix}"));
                    }
                }
            }
        }
    }
}

#[test]
fn datetime_all_offsets_match_independent_parser() {
    for minutes in -840..=840 {
        let offset = TimezoneOffset::from_minutes(minutes).unwrap();
        for value in ["-0001-12-31T24:00:00", "2024-02-29T12:34:56.000000001"] {
            compare(&format!("{value}{offset}"));
        }
    }
    for suffix in [
        "+00:00", "-00:00", "+14:01", "-14:01", "+05:60", "+0500", "z", "Zjunk",
    ] {
        compare(&format!("2024-02-29T12:34:56{suffix}"));
    }
}

#[test]
fn datetime_representation_limits_are_explicit() {
    // Valid XSD values can exceed our bounded year or precision representation.
    for input in [
        "9999-12-31T24:00:00Z",
        "10000-01-01T00:00:00Z",
        "2026-01-02T12:34:56.0000000001Z",
    ] {
        assert!(input.parse::<oxsdatatypes::DateTime>().is_ok());
        assert!(xsd::parse_datetime(input).is_err());
    }
}

#[test]
fn fractional_seconds_do_not_advance_calendar_fields() {
    for date in ["-9999-01-01", "-0001-12-31", "0000-02-29", "9999-12-31"] {
        for suffix in ["", "Z", "+14:00", "-14:00"] {
            let input = format!("{date}T23:59:59.999999999{suffix}");
            assert_eq!(xsd::parse_datetime(&input).unwrap().to_string(), input);
            compare(&input);
        }
    }
}
