use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::the_world::damage_types;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Difficulty {
    pub(crate) max_resistance: HashMap<damage_types::DamageType, u64>,
    pub(crate) min_resistance: HashMap<damage_types::DamageType, u64>,
    pub(crate) max_simultaneous_resistances: u8,
    pub(crate) min_simultaneous_resistances: u8,
}
