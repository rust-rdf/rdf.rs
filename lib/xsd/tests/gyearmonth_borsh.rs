#![cfg(feature = "borsh")]

use xsd::{TimezoneOffset, primitive::GYearMonth};

#[test]
fn versioned_year_month_encoding_preserves_boundaries() {
    for year in [i32::MIN, -1, 0, 1, i32::MAX] {
        for month in [1, 12] {
            for minutes in [None, Some(-840i16), Some(0), Some(330), Some(840)] {
                let value = GYearMonth::new(year, month)
                    .unwrap()
                    .with_timezone(minutes.map(|m| TimezoneOffset::from_minutes(m).unwrap()));
                let mut expected = vec![1];
                expected.extend(year.to_le_bytes());
                expected.extend([month, u8::from(minutes.is_some())]);
                if let Some(minutes) = minutes {
                    expected.extend(minutes.to_le_bytes());
                }
                assert_eq!(borsh::to_vec(&value).unwrap(), expected);
                assert_eq!(borsh::from_slice::<GYearMonth>(&expected).unwrap(), value);
                for end in 0..expected.len() {
                    assert!(borsh::from_slice::<GYearMonth>(&expected[..end]).is_err());
                }
                expected.push(0);
                assert!(borsh::from_slice::<GYearMonth>(&expected).is_err());
            }
        }
    }
}

#[test]
fn invalid_encodings_and_legacy_pairs_are_rejected() {
    for bytes in [
        &[0, 0, 0, 0, 0, 1, 0][..],
        &[2, 0, 0, 0, 0, 1, 0],
        &[1, 0, 0, 0, 0, 0, 0],
        &[1, 0, 0, 0, 0, 13, 0],
        &[1, 0, 0, 0, 0, 1, 2],
        &[1, 0, 0, 0, 0, 1, 1, 73, 3],
        &[1, 0, 0, 0, 0, 1, 1, 183, 252],
    ] {
        assert_eq!(
            borsh::from_slice::<GYearMonth>(bytes).unwrap_err().kind(),
            borsh::io::ErrorKind::InvalidData
        );
    }
    for year in [i32::MIN, -1, 0, 1, i32::MAX] {
        for month in 0..=255u8 {
            let legacy = borsh::to_vec(&(year, month)).unwrap();
            assert!(borsh::from_slice::<GYearMonth>(&legacy).is_err());
            let (y, m) = borsh::from_slice::<(i32, u8)>(&legacy).unwrap();
            let migrated = GYearMonth::new(y, m);
            assert_eq!(migrated.is_some(), (1..=12).contains(&month));
            if let Some(value) = migrated {
                assert_eq!(
                    borsh::from_slice::<GYearMonth>(&borsh::to_vec(&value).unwrap()).unwrap(),
                    value
                );
            }
        }
    }
}
