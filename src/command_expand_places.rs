use std::collections::HashMap;
use crate::Game;
use crate::place::Place;
use crate::place_generator::generate_place;
use crate::treasure_types::TreasureType::Gold;
use serde::{Deserialize, Serialize};
use crate::treasure_types::{pay_crafting_cost, TreasureType};

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ExecuteExpandPlacesReport {
    new_place: Place,
    paid_cost: HashMap<TreasureType, u64>,
    new_cost: HashMap<TreasureType, u64>,
    leftover_spending_treasure: HashMap<TreasureType, u64>,
}

pub fn execute_expand_places(game: &mut Game) -> Result<ExecuteExpandPlacesReport, String> {
    //Crafting cost
    let crafting_cost = execute_expand_places_calculate_cost(game);
    if let Err(error_message) = pay_crafting_cost(game, &crafting_cost) {
        return Err(error_message)
    };

    //Create new place
    let new_place = generate_place(game);
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
    use crate::command_expand_places::execute_expand_places;
    use crate::command_move::execute_move_command;
    use crate::game_generator::generate_testing_game;
    use crate::treasure_types::TreasureType::Gold;

    #[test]
    fn test_execute_expand_places() {
        let mut game = generate_testing_game(Some([1; 16]));
        assert_eq!(10, game.places.len());

        assert_eq!(Err("Cant pay the crafting cost, the cost is {Gold: 100} and you only have {}".to_string()), execute_expand_places(&mut game));

        assert!(execute_move_command(&mut game, 0).is_err());
        assert!(execute_move_command(&mut game, 0).is_ok());
        assert!(game.treasure.get(&Gold).unwrap() > &0);
        assert_eq!(10, game.places.len());

        let result = execute_expand_places(&mut game);

        assert!(result.is_ok());
        assert_eq!(11, game.places.len());
    }

    #[test]
    fn seeding_test() {
        let mut game = generate_testing_game(Some([1; 16]));
        game.treasure.insert(Gold, 1000);
        let original_result = execute_expand_places(&mut game);

        for _i in 1..1000 {
            let mut game = generate_testing_game(Some([1; 16]));
            game.treasure.insert(Gold, 1000);
            let result = execute_expand_places(&mut game);
            assert_eq!(original_result, result);
        }
    }

    #[test]
    fn many_runs_test() {
        let mut game = generate_testing_game(Some([1; 16]));
        game.treasure.insert(Gold, 999_999);

        for _i in 1..438 {
            assert!(execute_expand_places(&mut game).is_ok());
        }
    }
}