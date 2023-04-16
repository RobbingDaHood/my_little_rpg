use std::{
    cmp::{max, min},
    collections::HashMap,
    ops::Div,
};

use rand::prelude::SliceRandom;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    generator::place::new,
    my_little_rpg_errors::MyError,
    the_world::{
        damage_types::get_mut_random_attack_type,
        difficulty::Difficulty,
        treasure_types::{TreasureType, TreasureType::Gold},
    },
    Game,
};

mod tests;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Report {
    new_difficulty: Difficulty,
    paid_cost: HashMap<TreasureType, u64>,
    new_cost: HashMap<TreasureType, u64>,
    leftover_spending_treasure: HashMap<TreasureType, u64>,
}

pub fn execute_json(game: &mut Game) -> Value {
    match execute(game) {
        Ok(result) => json!(result),
        Err(result) => json!(result),
    }
}

pub fn execute(game: &mut Game) -> Result<Report, MyError> {
    //Add new element
    let mut random_min_entry = get_mut_random_attack_type(
        &mut game.random_generator_state,
        &mut game.difficulty.min_resistance,
        &|_, _| true,
    )?;
    let attack_type = random_min_entry.key();
    let new_max_value = random_min_entry.get().div(2);

    if new_max_value < 4 && game.difficulty.max_resistance.len() > 1 {
        game.difficulty.max_resistance.remove(&attack_type);
        random_min_entry.remove();

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
        let new_min = min(max(1, new_max_value.div(2)), *random_min_entry.get());
        random_min_entry.insert(min(max(1, new_max_value.div(2)), new_min));
    }

    let new_place = new(game);
    *game
        .places
        .choose_mut(&mut game.random_generator_state)
        .unwrap() = new_place;

    Ok(Report {
        new_difficulty: game.difficulty.clone(),
        paid_cost: execute_execute_reduce_difficulty_cost(),
        new_cost: execute_execute_reduce_difficulty_cost(),
        leftover_spending_treasure: game.treasure.clone(),
    })
}

pub fn execute_execute_reduce_difficulty_cost() -> HashMap<TreasureType, u64> {
    HashMap::from([(Gold, 0)])
}
