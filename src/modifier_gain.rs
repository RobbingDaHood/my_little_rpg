use crate::attack_types::AttackType;

use serde::{Deserialize, Serialize};
use crate::item_resource::ItemResourceType;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum ModifierGain {
    FlatDamage(AttackType, u64),
    FlatItemResource(ItemResourceType, u64),
}