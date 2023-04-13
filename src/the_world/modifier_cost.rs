use serde::{Deserialize, Serialize};

use crate::the_world::{attack_types::AttackType, item_resource::Type};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Eq, Hash)]
pub enum Cost {
    FlatItemResource(Type, u64),
    FlatMinItemResourceRequirement(Type, u64),
    FlatMaxItemResourceRequirement(Type, u64),
    FlatMinAttackRequirement(AttackType, u64),
    FlatMaxAttackRequirement(AttackType, u64),
    FlatSumMinAttackRequirement(u64),
    //TODO Handle summing issue; x * u64 could be more than 64; That is still a high number though. Maybe it is fine that attacks are u32.
    FlatSumMaxAttackRequirement(u64),
    //TODO add the same for reduce resistance
    FlatMinResistanceRequirement(AttackType, u64),
    FlatMaxResistanceRequirement(AttackType, u64),
    FlatMinSumResistanceRequirement(u64),
    FlatMaxSumResistanceRequirement(u64),
    PlaceLimitedByIndexModulus(u8, Vec<u8>),
    //TODO replace with struct so easier to read
    MinWinsInARow(u8),
    MaxWinsInARow(u8),
}
