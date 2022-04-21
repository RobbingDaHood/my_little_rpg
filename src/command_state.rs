use std::collections::HashMap;
use crate::place::Place;
use crate::place_generator::{PlaceGeneratorInput};
use serde::{Deserialize, Serialize};
use crate::command_craft_reroll_modifier::execute_craft_reroll_modifier_calculate_cost;
use crate::command_expand_modifier::execute_expand_modifiers_calculate_cost;
use crate::Game;
use crate::item::Item;
use crate::item_resource::ItemResourceType;
use crate::treasure_types::TreasureType;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct PresentationGameState {
    pub(crate) places: Vec<(usize, Place)>,
    pub(crate) equipped_items: Vec<(usize, Item)>,
    pub(crate) inventory: Vec<PresentationItem>,
    pub(crate) place_generator_input: PlaceGeneratorInput,
    pub(crate) treasure: HashMap<TreasureType, u64>,
    pub(crate) item_resources: HashMap<ItemResourceType, u64>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct PresentationItem {
    index: usize,
    item: Item,
    crafting_actions: ItemCosts,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ItemCosts {
    reroll_modifier: Vec<CostsInList>,
    add_modifier: u64,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct CostsInList {
    index: usize,
    cost: u64,
}


pub fn execute_state(game: &mut Game) -> PresentationGameState {
    let places: Vec<(usize, Place)> = game.places.iter()
        .map(|item| item.clone())
        .enumerate()
        .collect();
    let equipped_items: Vec<(usize, Item)> = game.equipped_items.iter()
        .map(|item| item.clone())
        .enumerate()
        .collect();
    let inventory: Vec<PresentationItem> = game.inventory.iter()
        .map(|item| item.clone())
        .enumerate()
        .map(|(index, item)| PresentationItem {
            index,
            item: item.clone(),
            crafting_actions: calculate_item_cost(game, &item, index)
        })
        .collect();

    PresentationGameState {
        places,
        equipped_items,
        inventory,
        place_generator_input: game.place_generator_input.clone(),
        treasure: game.treasure.clone(),
        item_resources: game.item_resources.clone(),
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