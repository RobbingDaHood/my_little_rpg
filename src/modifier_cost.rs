use serde::{Deserialize, Serialize};
use crate::attack_types::AttackType;
use crate::item_resource::ItemResourceType;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum ModifierCost {
    FlatItemResource(ItemResourceType, u64),
    FlatMinAttackRequirement(AttackType, u64),
}