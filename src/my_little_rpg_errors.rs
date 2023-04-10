use std::fmt;
use std::fmt::Formatter;

use serde::{Deserialize, Serialize};

//TODO consider if we need the struct wrapper
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Eq, Hash)]
pub enum MyErrorKind {
    ParseCommand { error_message: Box<str> },
    SaveLoad { error_message: Box<str> },
    ExecuteCommand { error_message: Box<str> },
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Eq, Hash)]
pub struct MyError {
    pub kind: MyErrorKind,
}

//TODO Add general help message to all errors
impl fmt::Display for MyErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MyErrorKind::ExecuteCommand { error_message } |MyErrorKind::SaveLoad { error_message } | MyErrorKind::ParseCommand { error_message } => write!(f, "Got the following error while trying to parse the given command: {}", error_message),
        }
    }
}

//TODO use the other methods instead; or consider if this from can take two parameters
impl From<String> for MyError {
    fn from(em: String) -> Self {
        MyError { kind: MyErrorKind::ParseCommand { error_message: em.into() } }
    }
}

impl MyError {
    pub fn create_parse_command_error(error_message: String) -> MyError {
        MyError { kind: MyErrorKind::ParseCommand { error_message: error_message.into() } }
    }
    pub fn create_save_load_error(error_message: String) -> MyError {
        MyError { kind: MyErrorKind::SaveLoad { error_message: error_message.into() } }
    }
    pub fn create_execute_command_error(error_message: String) -> MyError {
        MyError { kind: MyErrorKind::ExecuteCommand { error_message: error_message.into() } }
    }
}