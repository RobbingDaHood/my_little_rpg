use serde::{Deserialize, Serialize};
use crate::attack_types::AttackType;
use crate::item_resource::ItemResourceType;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Eq, Hash)]
pub enum ModifierCost {
    FlatItemResource(ItemResourceType, u64),
    FlatMinItemResourceRequirement(ItemResourceType, u64),
    FlatMaxItemResourceRequirement(ItemResourceType, u64),
    FlatMinAttackRequirement(AttackType, u64),
    FlatMaxAttackRequirement(AttackType, u64),
    FlatSumMinAttackRequirement(u64), //TODO Handle summing issue; x * u64 could be more than 64; That is still a high number though. Maybe it is fine that attacks are u32.
    FlatSumMaxAttackRequirement(u64),
    FlatMinResistanceRequirement(AttackType, u64),
    FlatMaxResistanceRequirement(AttackType, u64),
    FlatMinSumResistanceRequirement(u64),
    FlatMaxSumResistanceRequirement(u64),
    PlaceLimitedByIndexModulus(u8, Vec<u8>),
    MinWinsInARow(u8)
    //TODO Wins/loses in row
}