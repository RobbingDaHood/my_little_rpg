use crate::{my_little_rpg_errors::MyError, the_world::index_specifier::IndexSpecifier};

mod tests;

pub fn try_parse_usize(string_to_parse: &str) -> Result<usize, MyError> {
    string_to_parse.parse::<usize>().map_err(|error| {
        let error_message = format!(
            "The following parameter {string_to_parse}, got the following error while parsing: {error:?}"
        );
        MyError::create_parse_command_error(error_message)
    })
}

pub fn try_parse_possible_relative_indexes(
    command_parts: &str,
    relative_too: usize,
) -> Result<Vec<IndexSpecifier>, MyError> {
    command_parts.split(',').map(|index_specifier| {
        match index_specifier.chars().next() {
            Some('+') => {
                try_parse_possible_relative_indexes_get_absolute(
                    relative_too,
                    &index_specifier,
                    usize::checked_add,
                    "overflow",
                ).map(IndexSpecifier::RelativePositive)
            }
            Some('-') => {
                try_parse_possible_relative_indexes_get_absolute(
                    relative_too,
                    &index_specifier,
                    usize::checked_sub,
                    "underflow",
                ).map(IndexSpecifier::RelativeNegative)
            }
            _ => try_parse_usize(index_specifier).map(IndexSpecifier::Absolute),
        }
    }).collect()
}

fn try_parse_possible_relative_indexes_get_absolute(
    relative_too: usize,
    s: &&str,
    operation: fn(usize, usize) -> Option<usize>,
    flow_type: &str,
) -> Result<usize, MyError> {
    try_parse_usize(&s[1..s.len()]).map(|relative_index_diff| {
        operation(relative_too, relative_index_diff).map_or_else(
            || {
                Err(MyError::create_parse_command_error(format!(
                    "{relative_too}{s} created an {flow_type}!"
                )))
            },
            |_| Ok(relative_index_diff),
        )
    })?
}
