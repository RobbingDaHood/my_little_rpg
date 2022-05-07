use std::collections::HashMap;
use crate::attack_types;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Difficulty {
    pub(crate) max_resistance: HashMap<attack_types::AttackType, u64>,
    pub(crate) min_resistance: HashMap<attack_types::AttackType, u64>,
    pub(crate) max_simultaneous_resistances: u8,
    pub(crate) min_simultaneous_resistances: u8,
}