use std::collections::HashMap;
use crate::place::Place;

use serde::{Deserialize, Serialize};
use crate::treasure_types::TreasureType;
use crate::item::Item;
use crate::item_resource::ItemResourceType;
use crate::place_generator::PlaceGeneratorInput;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Game {
    pub(crate) places: Vec<Place>,
    pub(crate) equipped_items: Vec<Item>,
    pub(crate) place_generator_input: PlaceGeneratorInput,
    pub(crate) treasure: HashMap<TreasureType, u64>,
    pub(crate) item_resources: HashMap<ItemResourceType, u64>,
}