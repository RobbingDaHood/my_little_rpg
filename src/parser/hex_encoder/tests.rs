#[cfg(test)]
mod tests_int {
    use crate::decode_hex;
    use crate::parser::hex_encoder::encode_hex;

    //TODO add more tests
    #[test]
    fn hex_encode_decode() {
        let data: [u8; 16] = [1; 16];

        let encoded = encode_hex(&data);

        let decoded: [u8; 16] = decode_hex(&*encoded).unwrap().try_into().unwrap();

        assert_eq!(data, decoded);
    }
}
