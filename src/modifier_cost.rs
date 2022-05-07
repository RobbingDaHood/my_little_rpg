use serde::{Deserialize, Serialize};
use crate::attack_types::AttackType;
use crate::item_resource::ItemResourceType;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Eq, Hash)]
pub enum ModifierCost {
    FlatItemResource(ItemResourceType, u64),
    FlatMinAttackRequirement(AttackType, u64),
    FlatMaxAttackRequirement(AttackType, u64),
}