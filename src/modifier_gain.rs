use crate::attack_types::AttackType;

use serde::{Deserialize, Serialize};
use crate::item_resource::ItemResourceType;
use crate::modifier_gain::ModifierGain::{FlatDamage, FlatDamageAgainstHighestResistance, FlatDamageAgainstLowestResistance, FlatIncreaseRewardedItems, FlatItemResource, FlatResistanceReduction, PercentageIncreaseDamage, PercentageIncreaseDamageAgainstHighestResistance, PercentageIncreaseDamageAgainstLowestResistance, PercentageIncreaseResistanceReduction, PercentageIncreaseTreasure};
use crate::treasure_types::TreasureType;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Eq, Hash)]
pub enum ModifierGain {
    FlatDamage(AttackType, u64),
    PercentageIncreaseDamage(AttackType, u16),
    FlatItemResource(ItemResourceType, u64),
    FlatResistanceReduction(AttackType, u64),
    PercentageIncreaseResistanceReduction(AttackType, u16),
    FlatDamageAgainstHighestResistance(u64),
    PercentageIncreaseDamageAgainstHighestResistance(u16),
    FlatDamageAgainstLowestResistance(u64),
    PercentageIncreaseDamageAgainstLowestResistance(u16),
    PercentageIncreaseTreasure(TreasureType, u16),
    FlatIncreaseRewardedItems(u16),
}

impl ModifierGain {
    pub fn get_all_given_attack_types(attack_types: Vec<AttackType>) -> Vec<ModifierGain> {
        let mut result = Vec::new();

        for attack_type in attack_types.clone() {
            result.push(FlatDamage(attack_type, 0));
        }

        for attack_type in attack_types.clone() {
            result.push(PercentageIncreaseDamage(attack_type, 0));
        }

        for item_resource in ItemResourceType::get_all() {
            result.push(FlatItemResource(item_resource, 0));
        }

        for attack_type in attack_types.clone() {
            result.push(FlatResistanceReduction(attack_type, 0));
        }

        for attack_type in attack_types.clone() {
            result.push(PercentageIncreaseResistanceReduction(attack_type, 0));
        }

        result.push(FlatDamageAgainstHighestResistance(0));
        result.push(PercentageIncreaseDamageAgainstHighestResistance(0));
        result.push(FlatDamageAgainstLowestResistance(0));
        result.push(PercentageIncreaseDamageAgainstLowestResistance(0));

        for treasure_type in TreasureType::get_all() {
            result.push(PercentageIncreaseTreasure(treasure_type, 0));
        }

        result.push(FlatIncreaseRewardedItems(0));

        result
    }

}