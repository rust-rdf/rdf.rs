#![cfg(feature = "jiff")]

use xsd::{TimezoneOffset, primitive::DateTime};

#[test]
fn timestamps_require_an_explicit_timezone() {
    let value = DateTime::new(2026, 1, 2, 12, 34, 56, 1).unwrap();
    assert!(
        value
            .to_timestamp()
            .unwrap_err()
            .to_string()
            .contains("explicit timezone")
    );
    assert!(jiff::Timestamp::try_from(value).is_err());
    let utc = value.with_timezone(Some(TimezoneOffset::UTC));
    assert_eq!(
        utc.to_timestamp().unwrap(),
        "2026-01-02T12:34:56.000000001Z"
            .parse::<jiff::Timestamp>()
            .unwrap()
    );
}

#[test]
fn timestamps_apply_offsets_without_losing_nanoseconds() {
    let civil = DateTime::new(2024, 3, 1, 0, 0, 0, 1).unwrap();
    for minutes in -840..=840 {
        let offset = TimezoneOffset::from_minutes(minutes).unwrap();
        let local = civil.with_timezone(Some(offset));
        let timestamp = local.to_timestamp().unwrap();
        assert_eq!(jiff::Timestamp::try_from(local).unwrap(), timestamp);
        assert_eq!(
            jiff::tz::Offset::from(offset).to_datetime(timestamp),
            civil.civil()
        );
    }
    let local = civil.with_timezone(TimezoneOffset::from_minutes(345));
    let utc = DateTime::new(2024, 2, 29, 18, 15, 0, 1)
        .unwrap()
        .with_timezone(Some(TimezoneOffset::UTC));
    assert_ne!(local, utc); // Structural identity still includes calendar and offset.
    assert_eq!(local.to_timestamp().unwrap(), utc.to_timestamp().unwrap());
}

#[test]
fn timestamp_range_limits_return_errors() {
    for civil in [jiff::civil::DateTime::MIN, jiff::civil::DateTime::MAX] {
        let value = DateTime::from(civil).with_timezone(Some(TimezoneOffset::UTC));
        assert!(value.to_timestamp().is_err());
        assert!(jiff::Timestamp::try_from(value).is_err());
    }
    for timestamp in [jiff::Timestamp::MIN, jiff::Timestamp::MAX] {
        for minutes in [-840, 0, 840] {
            let offset = TimezoneOffset::from_minutes(minutes).unwrap();
            let civil = jiff::tz::Offset::from(offset).to_datetime(timestamp);
            let value = DateTime::from(civil).with_timezone(Some(offset));
            assert_eq!(value.to_timestamp().unwrap(), timestamp);
        }
    }
}
