use std::{fmt::Write, num::ParseIntError};

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}

#[cfg(test)]
mod tests_int {
    use crate::decode_hex;
    use crate::hex_encoder::encode_hex;

    #[test]
    fn hex_encode_decode() {
        let data : [u8; 16] = [1; 16];

        let encoded = encode_hex(&data);

        let decoded : [u8; 16] = decode_hex(encoded.as_str())
            .unwrap()
            .try_into()
            .unwrap_or_else(|v: Vec<u8>| panic!("Seed need exactly 16 comma seperated u8; It got {:?}", v));

        assert_eq!(data, decoded)

    }
}