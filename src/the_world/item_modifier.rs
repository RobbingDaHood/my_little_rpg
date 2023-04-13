use serde::{Deserialize, Serialize};

use crate::the_world::{modifier_cost::Cost, modifier_gain::Gain};

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Modifier {
    pub(crate) costs: Vec<Cost>,
    pub(crate) gains: Vec<Gain>,
}
