use std::collections::HashMap;
use crate::place::Place;
use crate::place_generator::{PlaceGeneratorInput};
use serde::{Deserialize, Serialize};
use crate::Game;
use crate::item::Item;
use crate::item_resource::ItemResourceType;
use crate::treasure_types::TreasureType;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct PresentationGameState {
    pub(crate) places: Vec<(usize, Place)>,
    pub(crate) equipped_items: Vec<(usize, Item)>,
    pub(crate) inventory: Vec<(usize, Item)>,
    pub(crate) place_generator_input: PlaceGeneratorInput,
    pub(crate) treasure: HashMap<TreasureType, u64>,
    pub(crate) item_resources: HashMap<ItemResourceType, u64>,
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
    let inventory: Vec<(usize, Item)> = game.inventory.iter()
        .map(|item| item.clone())
        .enumerate()
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