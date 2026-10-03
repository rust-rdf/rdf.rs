#![cfg(all(feature = "jiff", feature = "borsh"))]

use xsd::{TimezoneOffset, primitive::Time};

#[test]
fn time_borsh_v1_has_stable_bytes_and_distinguishes_utc() {
    let time = Time::new(12, 34, 56, 1).unwrap();
    for (offset, bytes) in [
        (None, vec![1, 12, 34, 56, 1, 0, 0, 0, 0]),
        (Some(0), vec![1, 12, 34, 56, 1, 0, 0, 0, 1, 0, 0]),
        (Some(345), vec![1, 12, 34, 56, 1, 0, 0, 0, 1, 89, 1]),
    ] {
        let time = time.with_timezone(offset.map(|m| TimezoneOffset::from_minutes(m).unwrap()));
        assert_eq!(borsh::to_vec(&time).unwrap(), bytes);
        assert_eq!(borsh::from_slice::<Time>(&bytes).unwrap(), time);
        for end in 0..bytes.len() {
            assert!(borsh::from_slice::<Time>(&bytes[..end]).is_err());
        }
        let mut trailing = bytes;
        trailing.push(0);
        assert!(borsh::from_slice::<Time>(&trailing).is_err());
    }
}

#[test]
fn time_borsh_round_trips_nanoseconds_and_all_offsets() {
    for (h, m, s, ns) in [(0, 0, 0, 0), (12, 34, 56, 1), (23, 59, 59, 999_999_999)] {
        for minutes in -840..=840 {
            let time = Time::new(h, m, s, ns)
                .unwrap()
                .with_timezone(TimezoneOffset::from_minutes(minutes));
            assert_eq!(
                borsh::from_slice::<Time>(&borsh::to_vec(&time).unwrap()).unwrap(),
                time
            );
        }
    }
}

#[test]
fn time_borsh_rejects_invalid_version_clock_and_timezone() {
    for (h, m, s, ns) in [
        (24i8, 0i8, 0i8, 0i32),
        (-1, 0, 0, 0),
        (0, 60, 0, 0),
        (0, 0, 60, 0),
        (0, 0, 0, -1),
        (0, 0, 0, 1_000_000_000),
    ] {
        let bytes = borsh::to_vec(&(1u8, h, m, s, ns, None::<i16>)).unwrap();
        assert_eq!(
            borsh::from_slice::<Time>(&bytes).unwrap_err().kind(),
            borsh::io::ErrorKind::InvalidData
        );
    }
    for offset in [-841i16, 841, i16::MIN, i16::MAX] {
        let bytes = borsh::to_vec(&(1u8, 0i8, 0i8, 0i8, 0i32, Some(offset))).unwrap();
        assert_eq!(
            borsh::from_slice::<Time>(&bytes).unwrap_err().kind(),
            borsh::io::ErrorKind::InvalidData
        );
    }
    for (index, byte) in [(0, 0), (0, 2), (8, 2)] {
        let mut bytes = vec![1, 12, 34, 56, 1, 0, 0, 0, 0];
        bytes[index] = byte;
        assert_eq!(
            borsh::from_slice::<Time>(&bytes).unwrap_err().kind(),
            borsh::io::ErrorKind::InvalidData
        );
    }
}
