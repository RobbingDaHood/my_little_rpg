use serde::{Deserialize, Serialize};
use crate::item_resource::Type::Mana;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Eq, Hash)]
pub enum Type {
    Mana
}

impl Type {
    pub fn get_all() -> Vec<Type> {
        vec![
            Mana,
        ]
    }
}