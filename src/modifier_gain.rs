use crate::attack_types::AttackType;

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum ModifierGain {
    FlatDamage(AttackType, u64)
}