pub fn checksum<T>(data: T) -> u8
where
    T: AsRef<[u8]>,
{
    0u8.wrapping_sub(
        data.as_ref()
            .iter()
            .fold(0, |acc, &value| acc.wrapping_add(value)),
    )
}

pub fn is_data_valid<T>(data: T) -> bool
where
    T: AsRef<[u8]>,
{
    data.as_ref()
        .iter()
        .fold(0u8, |acc, &value| acc.overflowing_add(value).0) == 0x00
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(&[], 0x00 ; "data is empty")]
    #[test_case(&[0x00, 0x00, 0x00, 0x01], 0xFF ; "data is eof record")]
    #[test_case(&[0x02, 0x00, 0x00, 0x04, 0xFF, 0xFF], 0xFC ; "data is ela record")]
    #[test_case(&[0x04, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0xCD], 0x2A ; "data is sla record")]
    #[test_case(&[0x03, 0x00, 0x30, 0x00, 0x02, 0x33, 0x7A], 0x1E ; "wikipedia example")]
    fn test_checksum(data: &[u8], expected_result: u8) {
        assert_eq!(checksum(data), expected_result);
    }

    #[test_case(&[0x00, 0x00, 0x00, 0x01, 0xFF] ; "data is eof record")]
    #[test_case(&[0x02, 0x00, 0x00, 0x04, 0xFF, 0xFF, 0xFC] ; "data is ela record")]
    #[test_case(&[0x04, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0xCD, 0x2A] ; "data is sla record")]
    #[test_case(&[0x03, 0x00, 0x30, 0x00, 0x02, 0x33, 0x7A, 0x1E] ; "wikipedia example")]
    fn test_custom_checksum(data: &[u8]) {
        assert!(is_data_valid(data));
    }
}

#[cfg(kani)]
mod kani {
    use super::{checksum, is_data_valid};
    use kani::Arbitrary;

    // <len 1>,<address 2>,<record 1>,<data max 255>,<checksum 1>
    const MAX_DATA_LENGTH: usize = 1 + 2 + 1 + 255 + 1;

    #[kani::proof]
    fn kani_checksum_does_not_panic() {
        checksum(u8::any_array::<MAX_DATA_LENGTH>());
    }

    #[kani::proof]
    fn kani_valid_data_last_byte_is_checksum() {
        let data = u8::any_array::<MAX_DATA_LENGTH>();
        kani::assume(data.len() > 1);
        let last_byte = data.len() - 1;
        kani::assume(is_data_valid(data));
        assert_eq!(checksum(&data[..last_byte]), data[last_byte]);
    }
}
