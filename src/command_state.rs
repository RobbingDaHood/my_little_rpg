
use std::collections::HashMap;
use crate::place::Place;
use crate::place_generator::{Difficulty};
use serde::{Deserialize, Serialize};
use crate::command_craft_reroll_modifier::execute_craft_reroll_modifier_calculate_cost;
use crate::command_expand_elements::execute_expand_elements_calculate_cost;
use crate::command_expand_equipment_slots::execute_expand_equipment_slots_calculate_cost;
use crate::command_expand_max_element::execute_expand_max_element_calculate_cost;
use crate::command_expand_max_simultaneous_element::execute_expand_max_simultaneous_element_calculate_cost;
use crate::command_expand_min_element::execute_expand_min_element_calculate_cost;
use crate::command_expand_min_simultanius_element::execute_expand_min_simultaneous_element_calculate_cost;
use crate::command_expand_modifier::execute_expand_modifiers_calculate_cost;
use crate::command_expand_places::execute_expand_places_calculate_cost;
use crate::Game;
use crate::hex_encoder::encode_hex;
use crate::item::Item;
use crate::item_resource::ItemResourceType;
use crate::treasure_types::TreasureType;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct PresentationGameState {
    pub(crate) places: Vec<PresentationPlace>,
    pub(crate) equipped_items: Vec<PresentationItem>,
    pub(crate) inventory: Vec<PresentationItem>,
    pub(crate) difficulty: Difficulty,
    pub(crate) treasure: HashMap<TreasureType, u64>,
    pub(crate) item_resources: HashMap<ItemResourceType, u64>,
    pub(crate) crafting_action_costs: PlaceCosts,
    pub(crate) seed: String,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct PlaceCosts {
    expand_places: HashMap<TreasureType, u64>,
    expand_elements: HashMap<TreasureType, u64>,
    expand_max_element: HashMap<TreasureType, u64>,
    expand_min_element: HashMap<TreasureType, u64>,
    expand_max_simultaneous_element: HashMap<TreasureType, u64>,
    expand_min_simultaneous_element: HashMap<TreasureType, u64>,
    expand_equipment_slots: HashMap<TreasureType, u64>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct PresentationPlace {
    index: usize,
    place: Place,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct PresentationItem {
    index: usize,
    item: Item,
    crafting_action_costs: Result<ItemCosts, String>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ItemCosts {
    reroll_modifier: Vec<CostsInList>,
    add_modifier: HashMap<TreasureType, u64>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct CostsInList {
    index: usize,
    cost: HashMap<TreasureType, u64>,
}

pub fn execute_state(game: &mut Game) -> PresentationGameState {
    let places: Vec<PresentationPlace> = game.places.iter()
        .map(|item| item.clone())
        .enumerate()
        .map(|(index, place)| PresentationPlace {
            index,
            place: place.clone(),
        })
        .collect();
    let equipped_items: Vec<PresentationItem> = game.equipped_items.iter()
        .map(|item| item.clone())
        .enumerate()
        .map(|(index, item)| PresentationItem {
            index,
            item: item.clone(),
            crafting_action_costs: Err("Equipped items cannot be crafted on.".to_string()),
        })
        .collect();
    let inventory: Vec<PresentationItem> = game.inventory.iter()
        .map(|item| item.clone())
        .enumerate()
        .map(|(index, item)| PresentationItem {
            index,
            item: item.clone(),
            crafting_action_costs: Ok(calculate_item_cost(game, &item, index)),
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
    }
}

fn calculate_item_cost(game: &Game, item: &Item, item_index: usize) -> ItemCosts {
    let reroll_modifier = item.modifiers.iter()
        .enumerate()
        .map(|(modifier_index, _)| CostsInList {
            index: modifier_index,
            cost: execute_craft_reroll_modifier_calculate_cost(game, item_index, modifier_index),
        })
        .collect();

    let add_modifier = execute_expand_modifiers_calculate_cost(game, item_index);

    ItemCosts { reroll_modifier, add_modifier }
}