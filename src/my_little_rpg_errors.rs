use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Eq, Hash)]
pub enum MyError {
    ParseCommand { error_message: Box<str> },
    Network { error_message: Box<str> },
    SaveLoad { error_message: Box<str> },
    ExecuteCommand { error_message: Box<str> },
    MoveCommand {
        error_message: Box<str>,
        item_report: Box<str>,
    },
}

impl From<MyError> for Box<str> {
    fn from(error: MyError) -> Self {
        format!("{error:?}").into()
    }
}

impl MyError {
    pub fn create_parse_command_error(error_message: String) -> MyError {
        MyError::ParseCommand {
            error_message: error_message.into(),
        }
    }

    pub fn create_network_error(error_message: String) -> MyError {
        MyError::Network {
            error_message: error_message.into(),
        }
    }

    pub fn create_save_load_error(error_message: String) -> MyError {
        MyError::SaveLoad {
            error_message: error_message.into(),
        }
    }

    pub fn create_move_command_error(
        error_message: String,
        item_report: String,
    ) -> MyError {
        MyError::MoveCommand {
            error_message: error_message.into(),
            item_report: item_report.into(),
        }
    }

    pub fn create_execute_command_error(error_message: String) -> MyError {
        MyError::ExecuteCommand {
            error_message: error_message.into(),
        }
    }
}
