use crate::attack_types::AttackType;

use serde::{Deserialize, Serialize};
use crate::item_resource::ItemResourceType;
use crate::modifier_gain::ModifierGain::{FlatDamage, FlatDamageAgainstHighestResistance, FlatItemResource, FlatResistanceReduction, PercentageIncreaseDamage, PercentageIncreaseResistanceReduction};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Eq, Hash)]
pub enum ModifierGain {
    FlatDamage(AttackType, u64),
    PercentageIncreaseDamage(AttackType, u16),
    FlatItemResource(ItemResourceType, u64),
    FlatResistanceReduction(AttackType, u64),
    PercentageIncreaseResistanceReduction(AttackType, u16),
    FlatDamageAgainstHighestResistance(u64),
//TODO: Half unsatisfied defence, double unsatisfied attack, double tressure, doulbe items
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

        result
    }

}