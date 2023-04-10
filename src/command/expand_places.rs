use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::Game;
use crate::generator::place::new;
use crate::my_little_rpg_errors::MyError;
use crate::the_world::place::Place;
use crate::the_world::treasure_types::{pay_crafting_cost, TreasureType};
use crate::the_world::treasure_types::TreasureType::Gold;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ExecuteExpandPlacesReport {
    new_place: Place,
    paid_cost: HashMap<TreasureType, u64>,
    new_cost: HashMap<TreasureType, u64>,
    leftover_spending_treasure: HashMap<TreasureType, u64>,
}

pub fn execute_json(game: &mut Game) -> Value {
    match execute(game) {
        Ok(result) => json!(result),
        Err(result) => json!(result)
    }
}

pub fn execute(game: &mut Game) -> Result<ExecuteExpandPlacesReport, MyError> {
    //Crafting cost
    let crafting_cost = execute_expand_places_calculate_cost(game);
    pay_crafting_cost(game, &crafting_cost)?;

    //Create new place
    let new_place = new(game);
    game.places.push(new_place.clone());

    Ok(ExecuteExpandPlacesReport {
        new_place,
        paid_cost: crafting_cost.clone(),
        new_cost: execute_expand_places_calculate_cost(game),
        leftover_spending_treasure: game.treasure.clone(),
    })
}

pub fn execute_expand_places_calculate_cost(game: &mut Game) -> HashMap<TreasureType, u64> {
    HashMap::from([(Gold, (game.places.len() * 10) as u64)])
}

#[cfg(test)]
mod tests_int {
    use crate::command::expand_places::execute;
    use crate::command::r#move::execute as execute_move_command;
    use crate::generator::game::new_testing;
    use crate::my_little_rpg_errors::MyError;
    use crate::the_world::treasure_types::TreasureType::Gold;

    #[test]
    fn test_execute_expand_places() {
        let mut game = new_testing(Some([1; 16]));
        assert_eq!(10, game.places.len());

        assert_eq!(Err(MyError::create_execute_command_error("Cant pay the crafting cost, the cost is {Gold: 100} and you only have {}".to_string())), execute(&mut game));

        assert!(execute_move_command(&mut game, 0).is_err());
        assert!(execute_move_command(&mut game, 0).is_ok());
        assert!(game.treasure.get(&Gold).unwrap() > &0);
        assert_eq!(10, game.places.len());

        let result = execute(&mut game);

        assert!(result.is_ok());
        assert_eq!(11, game.places.len());
    }

    #[test]
    fn seeding_test() {
        let mut game = new_testing(Some([1; 16]));
        game.treasure.insert(Gold, 1000);
        let original_result = execute(&mut game);

        for _i in 1..1000 {
            let mut game = new_testing(Some([1; 16]));
            game.treasure.insert(Gold, 1000);
            let result = execute(&mut game);
            assert_eq!(original_result, result);
        }
    }

    #[test]
    fn many_runs_test() {
        let mut game = new_testing(Some([1; 16]));
        game.treasure.insert(Gold, 999_999);

        for _i in 1..438 {
            assert!(execute(&mut game).is_ok());
        }
    }
}