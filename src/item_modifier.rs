use serde::{Deserialize, Serialize};

use crate::modifier_cost::Cost;
use crate::modifier_gain::Gain;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Modifier {
    pub(crate) costs: Vec<Cost>,
    pub(crate) gains: Vec<Gain>,
}