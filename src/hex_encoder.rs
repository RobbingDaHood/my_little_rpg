use std::fmt::Write;

pub fn decode_hex(s: &str) -> Result<Vec<u8>, String> {
    let offending_chars = list_non_ascii_hexdigit_chars(s);

    if offending_chars.is_empty() {
        decode_hex_unsafe(&s)
    } else {
        Err(format!("{:?} is not hexdigit(s)!", offending_chars))
    }
}

fn list_non_ascii_hexdigit_chars(s: &str) -> Vec<char> {
    s.chars()
        .filter(|c| !char::is_ascii_hexdigit(c))
        .collect()
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
            .unwrap();

        assert_eq!(data, decoded);
    }
}