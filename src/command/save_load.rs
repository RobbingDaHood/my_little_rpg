use std::fs;
use std::fs::create_dir_all;

use serde_json::{json, Value};

use crate::Game;
use crate::my_little_rpg_errors::MyError;

pub fn execute_save_command_json(game: &Game, save_name: &str, save_path: Option<String>) -> Value {
    match execute_save_command(game, save_name, save_path) {
        Ok(result) | Err(result)=> json!(result)
    }
}

pub fn execute_save_command(game: &Game, save_name: &str, save_path: Option<String>) -> Result<String, String> {
    let file_path = get_file_path(save_name, save_path)?;
    return match fs::write(
        file_path,
        format!("{}", json!(game)).as_bytes(),
    ) {
        Err(error_message) => Err(format!("Failed saving the world! Reason: {}", error_message)),
        Ok(_) => Ok("You saved the world!".to_string())
    };
}

pub fn execute_load_command_json(game: &mut Game, save_name: &str, save_path: Option<String>) -> Value {
    match execute_load_command(save_name, save_path) {
        Ok(new_game) => {
            *game = new_game;
            json!("Game is loaded!")
        }
        Err(result) => json!(result)
    }
}

pub fn execute_load_command(save_name: &str, save_path: Option<String>) -> Result<Game, MyError> {
    let file_path = get_file_path(save_name, save_path)?;
    //TODO better way of flattening this?!
    fs::read(file_path)
        .map_err(|error| {
            let error_message = format!("Failed loading the world! Reason: {}", error);
            MyError::create_save_load_error(error_message)
        })
        .map(|data| serde_json::from_slice::<Game>(data.as_slice()))
        .and_then(|result| result
            .map_err(|error| {
                let error_message = format!("Failed loading the world! Reason: {}", error);
                MyError::create_save_load_error(error_message)
            })
        )
}

fn get_file_path(save_name: &str, save_path: Option<String>) -> Result<String, String> {
    let save_path = match save_path {
        Some(path) => path,
        None => "./save_games/".to_string()
    };

    match create_dir_all(&save_path) {
        Err(error_message) => Err(format!("Failed creating the folder for the save games, Reason: {}", error_message)),
        Ok(_) => Ok(format!("{}{}.json", save_path, save_name))
    }
}


#[cfg(test)]
mod tests_int {
    use std::fs;

    use crate::command::expand_max_element::execute;
    use crate::command::save_load::{execute_load_command, execute_save_command};
    use crate::generator::game::new_testing;
    use crate::the_world::treasure_types::TreasureType::Gold;

    #[test]
    fn seeding_test() {
        let mut game = new_testing(Some([1; 16]));
        game.treasure.insert(Gold, 1000);
        let original_result = execute(&mut game);

        for _i in 1..1000 {
            let mut game = new_testing(Some([1; 16]));
            game.treasure.insert(Gold, 1000);
            execute_save_command(&game, "save_load_seeding_test", Some("./testing/".to_string())).unwrap();
            let mut parsed_game = execute_load_command("save_load_seeding_test", Some("./testing/".to_string())).unwrap();

            assert_eq!(game, parsed_game);

            let result = execute(&mut parsed_game);
            assert_eq!(original_result, result);
        }

        //Cleanup
        fs::remove_dir_all("./testing/").expect("Had trouble cleanup after save_load_time");
    }
}