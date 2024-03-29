use serde::{Deserialize, Serialize};

use crate::the_world::{
    damage_types::DamageType,
    item_resource::Type,
    modifier_gain::Gain::{
        FlatDamage, FlatDamageAgainstHighestResistance, FlatDamageAgainstLowestResistance,
        FlatIncreaseRewardedItems, FlatItemResource, FlatResistanceReduction,
        PercentageIncreaseDamage, PercentageIncreaseDamageAgainstHighestResistance,
        PercentageIncreaseDamageAgainstLowestResistance, PercentageIncreaseResistanceReduction,
        PercentageIncreaseTreasure,
    },
    treasure_types::TreasureType,
};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Eq, Hash)]
pub enum Gain {
    FlatDamage(DamageType, u64),
    PercentageIncreaseDamage(DamageType, u16),
    FlatItemResource(Type, u64),
    FlatResistanceReduction(DamageType, u64),
    PercentageIncreaseResistanceReduction(DamageType, u16),
    FlatDamageAgainstHighestResistance(u64),
    PercentageIncreaseDamageAgainstHighestResistance(u16),
    FlatDamageAgainstLowestResistance(u64),
    PercentageIncreaseDamageAgainstLowestResistance(u16),
    PercentageIncreaseTreasure(TreasureType, u16),
    FlatIncreaseRewardedItems(u16),
}

impl Gain {
    //TODO Consider if the code could be simplified with default implementation
    pub fn get_all_given_attack_types(attack_types: Vec<DamageType>) -> Vec<Gain> {
        let mut result = Vec::new();

        for attack_type in attack_types.clone() {
            result.push(FlatDamage(attack_type, 0));
        }

        for attack_type in attack_types.clone() {
            result.push(PercentageIncreaseDamage(attack_type, 0));
        }

        for item_resource in Type::get_all() {
            result.push(FlatItemResource(item_resource, 0));
        }

        for attack_type in attack_types.clone() {
            result.push(FlatResistanceReduction(attack_type, 0));
        }

        for attack_type in attack_types {
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
