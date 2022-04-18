use crate::item_modifier::ItemModifier;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Item {
    pub(crate) modifiers: Vec<ItemModifier>,
}