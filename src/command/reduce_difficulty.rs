use std::cmp::{max, min};
use std::collections::HashMap;
use std::ops::Div;

use rand::prelude::SliceRandom;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::generator::place::new;
use crate::the_world::attack_types::get_random_attack_type_from_unlocked;
use crate::the_world::difficulty::Difficulty;
use crate::the_world::treasure_types::TreasureType;
use crate::the_world::treasure_types::TreasureType::Gold;
use crate::Game;

mod tests;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Report {
    new_difficulty: Difficulty,
    paid_cost: HashMap<TreasureType, u64>,
    new_cost: HashMap<TreasureType, u64>,
    leftover_spending_treasure: HashMap<TreasureType, u64>,
}

pub fn execute_json(game: &mut Game) -> Value {
    json!(execute(game))
}

pub fn execute(game: &mut Game) -> Report {
    //Add new element
    let attack_type = get_random_attack_type_from_unlocked(
        &mut game.random_generator_state,
        &game.difficulty.min_resistance,
    );

    let max_value = game.difficulty.max_resistance.get(&attack_type).unwrap();
    let new_max_value = max_value.div(2);

    if new_max_value < 4 && game.difficulty.max_resistance.len() > 1 {
        game.difficulty.max_resistance.remove(&attack_type);
        game.difficulty.min_resistance.remove(&attack_type);

        let max_simultaneous_resistances =
            usize::from(game.difficulty.max_simultaneous_resistances);
        if max_simultaneous_resistances > game.difficulty.max_resistance.len() {
            game.difficulty.max_simultaneous_resistances = min(
                game.difficulty.max_simultaneous_resistances,
                game.difficulty.max_resistance.len().try_into().unwrap(),
            );
        }

        if game.difficulty.min_simultaneous_resistances
            > game.difficulty.max_simultaneous_resistances
        {
            game.difficulty.min_simultaneous_resistances =
                game.difficulty.max_simultaneous_resistances;
        }
    } else {
        *game
            .difficulty
            .max_resistance
            .get_mut(&attack_type)
            .unwrap() = max(2, new_max_value);
        let new_min = min(
            max(1, new_max_value.div(2)),
            *game.difficulty.min_resistance.get(&attack_type).unwrap(),
        );
        *game
            .difficulty
            .min_resistance
            .get_mut(&attack_type)
            .unwrap() = min(max(1, new_max_value.div(2)), new_min);
    }

    let new_place = new(game);
    *game
        .places
        .choose_mut(&mut game.random_generator_state)
        .unwrap() = new_place;

    Report {
        new_difficulty: game.difficulty.clone(),
        paid_cost: execute_execute_reduce_difficulty_cost(),
        new_cost: execute_execute_reduce_difficulty_cost(),
        leftover_spending_treasure: game.treasure.clone(),
    }
}

pub fn execute_execute_reduce_difficulty_cost() -> HashMap<TreasureType, u64> {
    HashMap::from([(Gold, 0)])
}
