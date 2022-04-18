use serde::{Deserialize, Serialize};
use crate::item_resource::ItemResourceType::Mana;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Eq, Hash)]
pub enum ItemResourceType {
    Mana
}

impl ItemResourceType {
    pub fn get_all() -> Vec<ItemResourceType> {
        vec![
            Mana,
        ]
    }
}