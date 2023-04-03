use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::command::command_craft_expand_modifier::execute_craft_expand_modifiers_calculate_cost;
use crate::command::command_craft_reroll_modifier::execute_craft_reroll_modifier_calculate_cost;
use crate::command::command_expand_elements::execute_expand_elements_calculate_cost;
use crate::command::command_expand_equipment_slots::execute_expand_equipment_slots_calculate_cost;
use crate::command::command_expand_max_element::execute_expand_max_element_calculate_cost;
use crate::command::command_expand_max_simultaneous_element::execute_expand_max_simultaneous_element_calculate_cost;
use crate::command::command_expand_min_element::execute_expand_min_element_calculate_cost;
use crate::command::command_expand_min_simultanius_element::execute_expand_min_simultaneous_element_calculate_cost;
use crate::command::command_expand_places::execute_expand_places_calculate_cost;
use crate::command::command_reduce_difficulty::execute_execute_reduce_difficulty_cost;
use crate::Game;
use crate::parser::hex_encoder::encode_hex;
use crate::the_world::difficulty::Difficulty;
use crate::the_world::game_statistics::GameStatistics;
use crate::the_world::item::Item;
use crate::the_world::item_resource::Type;
use crate::the_world::place::Place;
use crate::the_world::treasure_types::TreasureType;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct PresentationGameState {
    pub(crate) places: Vec<PresentationPlace>,
    pub(crate) equipped_items: Vec<PresentationItem>,
    pub(crate) inventory: Vec<PresentationItem>,
    pub(crate) difficulty: Difficulty,
    pub(crate) treasure: HashMap<TreasureType, u64>,
    pub(crate) item_resources: HashMap<Type, u64>,
    pub(crate) crafting_action_costs: PlaceCosts,
    pub(crate) seed: String,
    pub(crate) game_statistics: GameStatistics,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct PlaceCosts {
    expand_places: HashMap<TreasureType, u64>,
    expand_elements: HashMap<TreasureType, u64>,
    expand_max_element: HashMap<TreasureType, u64>,
    expand_min_element: HashMap<TreasureType, u64>,
    expand_max_simultaneous_element: HashMap<TreasureType, u64>,
    expand_min_simultaneous_element: HashMap<TreasureType, u64>,
    expand_equipment_slots: HashMap<TreasureType, u64>,
    execute_reduce_difficulty: HashMap<TreasureType, u64>,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct PresentationPlace {
    index: usize,
    place: Place,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct PresentationItem {
    index: usize,
    item: Item,
    crafting_action_costs: Result<ItemCosts, String>, //TODO Insert this into items, in a way where we do not need to maintain a second item model
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ItemCosts {
    reroll_modifier: u16,
    add_modifier: usize,
}

pub fn execute_state(game: &mut Game) -> PresentationGameState {
    let places: Vec<PresentationPlace> = game.places.iter()
        .cloned()
        .enumerate()
        .map(|(index, place)| PresentationPlace {
            index,
            place,
        })
        .collect();
    let equipped_items: Vec<PresentationItem> = game.equipped_items.iter()
        .cloned()
        .enumerate()
        .map(|(index, item)| PresentationItem {
            index,
            item,
            crafting_action_costs: Err("Equipped items cannot be crafted on.".to_string()),
        })
        .collect();
    let inventory: Vec<PresentationItem> = game.inventory.iter()
        .enumerate()
        .filter(|item| item.1.is_some())
        .map(|(index, item)| PresentationItem {
            index,
            item: item.as_ref().unwrap().clone(),
            crafting_action_costs: Ok(calculate_item_cost(game, index)),
        })
        .collect();

    let crafting_actions = PlaceCosts {
        expand_places: execute_expand_places_calculate_cost(game),
        expand_elements: execute_expand_elements_calculate_cost(game),
        expand_max_element: execute_expand_max_element_calculate_cost(game),
        expand_min_element: execute_expand_min_element_calculate_cost(game),
        expand_max_simultaneous_element: execute_expand_max_simultaneous_element_calculate_cost(game),
        expand_min_simultaneous_element: execute_expand_min_simultaneous_element_calculate_cost(game),
        expand_equipment_slots: execute_expand_equipment_slots_calculate_cost(game),
        execute_reduce_difficulty: execute_execute_reduce_difficulty_cost(),
    };

    PresentationGameState {
        places,
        equipped_items,
        inventory,
        difficulty: game.difficulty.clone(),
        treasure: game.treasure.clone(),
        item_resources: game.item_resources.clone(),
        crafting_action_costs: crafting_actions,
        seed: encode_hex(&game.seed),
        game_statistics: game.game_statistics.clone(),
    }
}

fn calculate_item_cost(game: &Game, item_index: usize) -> ItemCosts {
    let add_modifier = execute_craft_expand_modifiers_calculate_cost(game, item_index);
    let reroll_modifier = execute_craft_reroll_modifier_calculate_cost(game, item_index);
    ItemCosts { reroll_modifier, add_modifier }
}