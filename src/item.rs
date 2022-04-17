use crate::item_modifier::ItemModifier;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Item {
    modifiers: Vec<ItemModifier>
}

impl Item {
    pub fn new(modifiers: Vec<ItemModifier>) -> Self {
        Self { modifiers }
    }


    pub fn modifiers(&self) -> &Vec<ItemModifier> {
        &self.modifiers
    }
}