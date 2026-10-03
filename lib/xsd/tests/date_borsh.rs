#![cfg(all(feature = "jiff", feature = "borsh"))]

use xsd::{TimezoneOffset, primitive::Date};

#[test]
fn date_borsh_v1_has_stable_bytes_and_distinguishes_utc() {
    let date = Date::new(2024, 2, 29).unwrap();
    for (offset, bytes) in [
        (None, vec![1, 232, 7, 2, 29, 0]),
        (Some(0), vec![1, 232, 7, 2, 29, 1, 0, 0]),
        (Some(-840), vec![1, 232, 7, 2, 29, 1, 184, 252]),
    ] {
        let date = date.with_timezone(offset.map(|m| TimezoneOffset::from_minutes(m).unwrap()));
        assert_eq!(borsh::to_vec(&date).unwrap(), bytes);
        assert_eq!(borsh::from_slice::<Date>(&bytes).unwrap(), date);
        for end in 0..bytes.len() {
            assert!(borsh::from_slice::<Date>(&bytes[..end]).is_err());
        }
        let mut trailing = bytes;
        trailing.push(0);
        assert!(borsh::from_slice::<Date>(&trailing).is_err());
    }
}

#[test]
fn date_borsh_round_trips_signed_years_and_all_offsets() {
    for year in [-9999, -1, 0, 1, 9999] {
        for minutes in -840..=840 {
            let date = Date::new(year, 12, 31)
                .unwrap()
                .with_timezone(TimezoneOffset::from_minutes(minutes));
            assert_eq!(
                borsh::from_slice::<Date>(&borsh::to_vec(&date).unwrap()).unwrap(),
                date
            );
        }
    }
}

#[test]
fn date_borsh_rejects_invalid_version_calendar_and_timezone() {
    for bytes in [
        vec![0, 232, 7, 2, 29, 0],
        vec![2, 232, 7, 2, 29, 0],
        vec![1, 232, 7, 2, 29, 2],
    ] {
        assert_eq!(
            borsh::from_slice::<Date>(&bytes).unwrap_err().kind(),
            borsh::io::ErrorKind::InvalidData
        );
    }
    for (year, month, day) in [
        (2026i16, 2i8, 29i8),
        (0, 2, 30),
        (-1, 4, 31),
        (2026, 0, 1),
        (2026, 13, 1),
        (2026, 1, 0),
        (2026, 1, 32),
        (-10000, 1, 1),
        (10000, 1, 1),
    ] {
        let bytes = borsh::to_vec(&(1u8, year, month, day, None::<i16>)).unwrap();
        assert_eq!(
            borsh::from_slice::<Date>(&bytes).unwrap_err().kind(),
            borsh::io::ErrorKind::InvalidData
        );
    }
    for offset in [-841i16, 841, i16::MIN, i16::MAX] {
        let bytes = borsh::to_vec(&(1u8, 2024i16, 2i8, 29i8, Some(offset))).unwrap();
        assert_eq!(
            borsh::from_slice::<Date>(&bytes).unwrap_err().kind(),
            borsh::io::ErrorKind::InvalidData
        );
    }
}
