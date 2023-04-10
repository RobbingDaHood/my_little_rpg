use serde::{Deserialize, Serialize};

use crate::the_world::index_specifier::IndexSpecifier;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Eq, Hash)]
pub enum Command {
    State,
    Move(usize),
    Equip(usize, usize),
    SwapEquipment(usize, usize),
    RerollModifier(usize, usize, Vec<IndexSpecifier>),
    ExpandPlaces,
    ExpandElements,
    ExpandMaxElement,
    ExpandMinElement,
    ExpandMaxSimultaneousElement,
    ExpandMinSimultaneousElement,
    ExpandEquipmentSlots,
    ReduceDifficulty,
    AddModifier(usize, Vec<IndexSpecifier>),
    Help,
    ReorderInventory,
    SaveTheWorld(Box<str>, Option<Box<str>>),
    LoadTheWorld(Box<str>, Option<Box<str>>),
}
