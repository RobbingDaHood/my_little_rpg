use std::fmt::Write;
use std::num::ParseIntError;
use std::ops::Not;

pub fn decode_hex(s: &str) -> Result<Vec<u8>, String> {
    match s.chars().find(is_not_ascii_hexdigit) {
        Some(offending_char) => Err(format!("{} is not a hexdigit!", offending_char)),
        None => decode_hex_unsafe(&s)
    }
}

const fn is_not_ascii_hexdigit(c: &char) -> bool {
    !char::is_ascii_hexdigit(c)
}

fn decode_hex_unsafe(s: &&str) -> Result<Vec<u8>, String> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .map(|i| i.map_err(|s| s.to_string()))
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
        let data: [u8; 16] = [1; 16];

        let encoded = encode_hex(&data);

        let decoded: [u8; 16] = decode_hex(encoded.as_str())
            .unwrap()
            .try_into()
            .unwrap_or_else(|v: Vec<u8>| panic!("Seed need exactly 16 comma seperated u8; It got {:?}", v));

        assert_eq!(data, decoded);
    }
}