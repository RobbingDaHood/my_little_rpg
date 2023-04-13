use std::fmt::Write;

use crate::my_little_rpg_errors::MyError;

mod tests;

pub fn decode_hex(s: &str) -> Result<Vec<u8>, MyError> {
    let offending_chars = list_non_ascii_hexdigit_chars(s);

    if offending_chars.is_empty() {
        decode_hex_unsafe(&s)
    } else {
        Err(MyError::create_parse_command_error(format!(
            "{offending_chars:?} is not hexdigit(s)!"
        )))
    }
}

fn list_non_ascii_hexdigit_chars(s: &str) -> Vec<char> {
    s.chars().filter(|c| !char::is_ascii_hexdigit(c)).collect()
}

fn decode_hex_unsafe(s: &&str) -> Result<Vec<u8>, MyError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .map(|i| i.map_err(|s| MyError::create_parse_command_error(s.to_string())))
        .collect()
}

pub fn encode_hex(bytes: &[u8]) -> Box<str> {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{b:02x}").unwrap();
    }
    s.into()
}
