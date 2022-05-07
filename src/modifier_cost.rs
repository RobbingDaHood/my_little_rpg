use serde::{Deserialize, Serialize};
use crate::attack_types::AttackType;
use crate::item_resource::ItemResourceType;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Eq, Hash)]
pub enum ModifierCost {
    FlatItemResource(ItemResourceType, u64),
    FlatMinAttackRequirement(AttackType, u64),
    FlatMaxAttackRequirement(AttackType, u64),
    //TODO Add Attack min/max sum of all elements
    //TODO Add all the above on the places too
    PlaceLimitedByIndexModulus(u8, Vec<u8>),
    //TODO same as above but with counting
    //TODO Same as above but with a capacity
    //TODO fixed list of indexes
    //TODO Wins/loses in row
}