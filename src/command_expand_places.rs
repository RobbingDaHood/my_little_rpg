use std::collections::HashMap;
use crate::Game;
use crate::place::Place;
use crate::place_generator::generate_place;
use crate::treasure_types::TreasureType::Gold;
use serde::{Deserialize, Serialize};
use crate::treasure_types::TreasureType;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExecuteExpandPlacesReport {
    new_place: Place,
    paid_cost: HashMap<TreasureType, u64>,
    new_cost: HashMap<TreasureType, u64>,
    leftover_spending_treasure: HashMap<TreasureType, u64>,
}

pub fn execute_expand_places(game: &mut Game) -> Result<ExecuteExpandPlacesReport, String> {
    //Crafting cost
    let crafting_cost = execute_expand_places_calculate_cost(game);
    if *game.treasure.entry(Gold).or_insert(0) >= crafting_cost {
        *game.treasure.get_mut(&Gold).unwrap() -= crafting_cost;
    } else {
        return Err(format!("Cant pay the crafting cost for execute_expand_places, the cost is {} and you only have {:?}", crafting_cost, game.treasure.get(&Gold)));
    }

    //Create new place
    let new_place = generate_place(game);
    game.places.push(new_place.clone());

    Ok(ExecuteExpandPlacesReport {
        new_place: new_place.clone(),
        paid_cost: HashMap::from([(Gold, crafting_cost.clone())]),
        new_cost: HashMap::from([(Gold, execute_expand_places_calculate_cost(game))]),
        leftover_spending_treasure: game.treasure.clone(),
    })
}

pub fn execute_expand_places_calculate_cost(game: &mut Game) -> u64 {
    (game.places.len() * 10) as u64
}

#[cfg(test)]
mod tests_int {
    use crate::command_expand_places::execute_expand_places;
    use crate::command_move::execute_move_command;
    use crate::game_generator::generate_testing_game;
    use crate::treasure_types::TreasureType::Gold;

    #[test]
    fn test_execute_expand_places() {
        let mut game = generate_testing_game();
        assert_eq!(10, game.places.len());

        assert_eq!(Err("Cant pay the crafting cost for execute_expand_places, the cost is 100 and you only have Some(0)".to_string()), execute_expand_places(&mut game));

        assert!(execute_move_command(&mut game, 0).is_err());
        assert!(execute_move_command(&mut game, 0).is_ok());
        assert!(game.treasure.get(&Gold).unwrap() > &0);
        assert_eq!(10, game.places.len());

        let result = execute_expand_places(&mut game);

        assert!(result.is_ok());
        assert_eq!(11, game.places.len());
    }
}