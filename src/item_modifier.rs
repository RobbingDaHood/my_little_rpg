use crate::modifier_cost::ModifierCost;
use crate::modifier_gain::ModifierGain;

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ItemModifier {
    costs: Vec<ModifierCost>,
    gains: Vec<ModifierGain>,
}

impl ItemModifier {
    pub fn new(costs: Vec<ModifierCost>, gains: Vec<ModifierGain>) -> Self {
        Self { costs, gains }
    }
}