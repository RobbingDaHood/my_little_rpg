use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    command::{
        craft_expand_modifier::execute_craft_expand_modifiers_calculate_cost,
        craft_reroll_modifier::execute_craft_reroll_modifier_calculate_cost,
        expand_elements::execute_expand_elements_calculate_cost,
        expand_equipment_slots::execute_expand_equipment_slots_calculate_cost,
        expand_max_element::execute_expand_max_element_calculate_cost,
        expand_max_simultaneous_element::execute_expand_max_simultaneous_element_calculate_cost,
        expand_min_element::execute_expand_min_element_calculate_cost,
        expand_min_simultanius_element::execute_expand_min_simultaneous_element_calculate_cost,
        expand_places::execute_expand_places_calculate_cost,
        reduce_difficulty::execute_execute_reduce_difficulty_cost,
    },
    parser::hex_encoder::encode_hex,
    the_world::{
        difficulty::Difficulty, game_statistics::GameStatistics, item::Item, item_resource::Type,
        place::Place, treasure_types::TreasureType,
    },
    Game,
};

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct PresentationGameState {
    pub(crate) places: Vec<PresentationPlace>,
    pub(crate) equipped_items: Vec<PresentationItem>,
    pub(crate) inventory: Vec<PresentationItem>,
    pub(crate) difficulty: Difficulty,
    pub(crate) treasure: HashMap<TreasureType, u64>,
    //TODO use type alias or new type; instead of u64 create a treasure(u64) type
    pub(crate) item_resources: HashMap<Type, u64>,
    pub(crate) crafting_action_costs: PlaceCosts,
    pub(crate) seed: Box<str>,
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
    crafting_action_costs: Result<ItemCosts, Box<str>>, //TODO Insert this into items, in a way where we do not need to maintain a second item model
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ItemCosts {
    reroll_modifier: u16,
    add_modifier: usize,
}

pub fn execute_presentation_game_state_json(game: &mut Game) -> Value {
    json!(execute(game))
}

pub fn execute(game: &mut Game) -> PresentationGameState {
    let places: Vec<PresentationPlace> = game
        .places
        .iter()
        .cloned()
        .enumerate()
        .map(|(index, place)| PresentationPlace { index, place })
        .collect();
    let equipped_items: Vec<PresentationItem> = game
        .equipped_items
        .iter()
        .cloned()
        .enumerate()
        .map(|(index, item)| {
            PresentationItem {
                index,
                item,
                crafting_action_costs: Err("Equipped items cannot be crafted on.".into()), //TODD make this nicer; it is just an info and not an error.
            }
        })
        .collect();
    let inventory: Vec<PresentationItem> = game
        .inventory
        .iter()
        .enumerate()
        .filter(|item| item.1.is_some())
        .map(|(index, item)| {
            PresentationItem {
                index,
                item: item.as_ref().unwrap().clone(),
                crafting_action_costs: Ok(calculate_item_cost(game, index)),
            }
        })
        .collect();

    let crafting_actions = PlaceCosts {
        expand_places: execute_expand_places_calculate_cost(game),
        expand_elements: execute_expand_elements_calculate_cost(game),
        expand_max_element: execute_expand_max_element_calculate_cost(game),
        expand_min_element: execute_expand_min_element_calculate_cost(game),
        expand_max_simultaneous_element: execute_expand_max_simultaneous_element_calculate_cost(
            game,
        ),
        expand_min_simultaneous_element: execute_expand_min_simultaneous_element_calculate_cost(
            game,
        ),
        expand_equipment_slots: execute_expand_equipment_slots_calculate_cost(game),
        execute_reduce_difficulty: execute_execute_reduce_difficulty_cost(),
    };

    //TODO can I do all these presentation models without cloning so much?
    PresentationGameState {
        places,
        equipped_items,
        inventory,
        difficulty: game.difficulty.clone(),
        treasure: game.treasure.clone(),
        item_resources: game.item_resources.clone(),
        crafting_action_costs: crafting_actions,
        seed: encode_hex(&game.seed),
        game_statistics: game.statistics.clone(),
    }
}

fn calculate_item_cost(
    game: &Game,
    item_index: usize,
) -> ItemCosts {
    let add_modifier = execute_craft_expand_modifiers_calculate_cost(game, item_index);
    let reroll_modifier = execute_craft_reroll_modifier_calculate_cost(game, item_index);
    ItemCosts {
        reroll_modifier,
        add_modifier,
    }
}
