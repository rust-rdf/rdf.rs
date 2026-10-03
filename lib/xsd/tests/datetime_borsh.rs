#![cfg(all(feature = "jiff", feature = "borsh"))]

use xsd::{TimezoneOffset, primitive::DateTime};

#[test]
fn datetime_borsh_v1_has_stable_bytes_and_one_optional_offset() {
    let value = DateTime::new(2024, 2, 29, 12, 34, 56, 1).unwrap();
    for (offset, bytes) in [
        (None, vec![1, 232, 7, 2, 29, 12, 34, 56, 1, 0, 0, 0, 0]),
        (
            Some(0),
            vec![1, 232, 7, 2, 29, 12, 34, 56, 1, 0, 0, 0, 1, 0, 0],
        ),
        (
            Some(-840),
            vec![1, 232, 7, 2, 29, 12, 34, 56, 1, 0, 0, 0, 1, 184, 252],
        ),
    ] {
        let value = value.with_timezone(offset.map(|m| TimezoneOffset::from_minutes(m).unwrap()));
        assert_eq!(borsh::to_vec(&value).unwrap(), bytes);
        assert_eq!(borsh::from_slice::<DateTime>(&bytes).unwrap(), value);
        for end in 0..bytes.len() {
            assert!(borsh::from_slice::<DateTime>(&bytes[..end]).is_err());
        }
        let mut trailing = bytes;
        trailing.push(0);
        assert!(borsh::from_slice::<DateTime>(&trailing).is_err());
    }
}

#[test]
fn datetime_borsh_round_trips_boundaries_and_all_offsets() {
    for year in [-9999, -1, 0, 1, 9999] {
        for minutes in -840..=840 {
            let value = DateTime::new(year, 12, 31, 23, 59, 59, 999_999_999)
                .unwrap()
                .with_timezone(TimezoneOffset::from_minutes(minutes));
            assert_eq!(
                borsh::from_slice::<DateTime>(&borsh::to_vec(&value).unwrap()).unwrap(),
                value
            );
        }
    }
}

#[test]
fn datetime_borsh_rejects_invalid_version_fields_and_timezone() {
    let valid = vec![1, 232, 7, 2, 29, 12, 34, 56, 1, 0, 0, 0, 0];
    for (index, byte) in [
        (0, 0),
        (0, 2),
        (3, 0),
        (3, 13),
        (4, 0),
        (4, 30),
        (5, 24),
        (6, 60),
        (7, 60),
        (11, 255),
        (12, 2),
    ] {
        let mut bytes = valid.clone();
        bytes[index] = byte;
        assert_eq!(
            borsh::from_slice::<DateTime>(&bytes).unwrap_err().kind(),
            borsh::io::ErrorKind::InvalidData
        );
    }
    for year in [-10000i16, 10000, 2026] {
        let bytes =
            borsh::to_vec(&(1u8, year, 2i8, 29i8, 0i8, 0i8, 0i8, 0i32, None::<i16>)).unwrap();
        assert_eq!(
            borsh::from_slice::<DateTime>(&bytes).unwrap_err().kind(),
            borsh::io::ErrorKind::InvalidData
        );
    }
    for offset in [-841i16, 841, i16::MIN, i16::MAX] {
        let bytes =
            borsh::to_vec(&(1u8, 2024i16, 2i8, 29i8, 0i8, 0i8, 0i8, 0i32, Some(offset))).unwrap();
        assert_eq!(
            borsh::from_slice::<DateTime>(&bytes).unwrap_err().kind(),
            borsh::io::ErrorKind::InvalidData
        );
    }
}
