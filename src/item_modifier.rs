use crate::modifier_cost::ModifierCost;
use crate::modifier_gain::ModifierGain;

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ItemModifier {
    pub(crate) costs: Vec<ModifierCost>,
    pub(crate) gains: Vec<ModifierGain>,
}