mod tests;

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
