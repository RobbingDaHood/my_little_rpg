use crate::place::Place;

use serde::{Deserialize, Serialize};
use crate::item::Item;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Game {
    places: Vec<Place>,
    equipped_items: Vec<Item>
}

impl Game {
    pub fn new(places: Vec<Place>, equipped_items: Vec<Item>) -> Self {
        Self { places, equipped_items }
    }
    pub fn places(&self) -> &Vec<Place> {
        &self.places
    }
    pub fn equipped_items(&self) -> &Vec<Item> {
        &self.equipped_items
    }
}