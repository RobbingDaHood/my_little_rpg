use crate::my_little_rpg_errors::MyError;

mod tests;

pub fn try_parse_usize(string_to_parse: &str) -> Result<usize, MyError> {
    string_to_parse.parse::<usize>().map_err(|error| {
        let error_message = format!(
            "The following parameter {}, got the following error while parsing: {:?}",
            string_to_parse, error
        );
        MyError::create_parse_command_error(error_message)
    })
}


