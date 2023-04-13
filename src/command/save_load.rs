use std::{fs, fs::create_dir_all};

use serde_json::{json, Value};

use crate::{my_little_rpg_errors::MyError, Game};

mod tests;

pub fn execute_save_command_json(
    game: &Game,
    save_name: &str,
    save_path: Option<Box<str>>,
) -> Value {
    match execute_save_command(game, save_name, save_path) {
        Ok(result) => json!(result),
        Err(result) => json!(result),
    }
}

pub fn execute_save_command(
    game: &Game,
    save_name: &str,
    save_path: Option<Box<str>>,
) -> Result<Box<str>, MyError> {
    let file_path = &*get_file_path(save_name, save_path)?;
    return match fs::write(file_path, format!("{}", json!(game)).as_bytes()) {
        Err(error_message) => {
            Err(MyError::create_save_load_error(format!(
                "Failed saving the world! Reason: {}",
                error_message
            )))
        }
        Ok(_) => Ok("You saved the world!".into()),
    };
}

pub fn execute_load_command_json(
    game: &mut Game,
    save_name: &str,
    save_path: Option<Box<str>>,
) -> Value {
    match execute_load_command(save_name, save_path) {
        Ok(new_game) => {
            *game = new_game;
            json!("Game is loaded!")
        }
        Err(result) => json!(result),
    }
}

pub fn execute_load_command(
    save_name: &str,
    save_path: Option<Box<str>>,
) -> Result<Game, MyError> {
    let file_path = &*get_file_path(save_name, save_path)?;
    //TODO better way of flattening this?!
    fs::read(file_path)
        .map_err(|error| {
            let error_message = format!("Failed loading the world! Reason: {}", error);
            MyError::create_save_load_error(error_message)
        })
        .map(|data| serde_json::from_slice::<Game>(data.as_slice()))
        .and_then(|result| {
            result.map_err(|error| {
                let error_message = format!("Failed loading the world! Reason: {}", error);
                MyError::create_save_load_error(error_message)
            })
        })
}

fn get_file_path(
    save_name: &str,
    save_path: Option<Box<str>>,
) -> Result<Box<str>, MyError> {
    let save_path: Box<str> = match save_path {
        Some(path) => path,
        None => "./save_games/".into(),
    };

    match create_dir_all(save_path.as_ref()) {
        Err(error_message) => {
            Err(MyError::create_save_load_error(format!(
                "Failed creating the folder for the save games, Reason: {}",
                error_message
            )))
        }
        Ok(_) => Ok(format!("{}{}.json", save_path, save_name).into()),
    }
}
