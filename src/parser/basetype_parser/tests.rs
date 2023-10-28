#[cfg(test)]
mod tests_int {
    use crate::{
        my_little_rpg_errors::{MyError, MyErrorKind},
        parser::basetype_parser::try_parse_usize,
    };

    #[test]
    fn test_try_parse_usize() {
        assert_eq!(100, try_parse_usize("100").unwrap());
    }

    #[test]
    fn test_try_parse_usize_error_negative_number() {
        assert_eq!(
            MyError {
                kind: MyErrorKind::ParseCommand {
                    error_message: Box::from(
                        "The following parameter -100, got the following error while parsing: \
                         ParseIntError { kind: InvalidDigit }"
                    ),
                },
            },
            try_parse_usize("-100").unwrap_err()
        );
    }

    #[test]
    fn test_try_parse_usize_error_not_a_number() {
        assert_eq!(
            MyError {
                kind: MyErrorKind::ParseCommand {
                    error_message: Box::from(
                        "The following parameter abd, got the following error while parsing: ParseIntError { kind: InvalidDigit }"
                    ),
                },
            },
            try_parse_usize("abd").unwrap_err()
        );
    }
}
