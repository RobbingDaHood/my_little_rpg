use crate::attack_types::AttackType;

use serde::{Deserialize, Serialize};
use crate::item_resource::ItemResourceType;
use crate::modifier_gain::ModifierGain::{FlatDamage, FlatItemResource};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Eq, Hash)]
pub enum ModifierGain {
    FlatDamage(AttackType, u64),
    FlatItemResource(ItemResourceType, u64),
}

impl ModifierGain {
    pub fn get_all_given_attack_types(attack_types: Vec<AttackType>) -> Vec<ModifierGain> {
        let mut result = Vec::new();

        for attack_type in attack_types {
            result.push(FlatDamage(attack_type, 0));
        }

        for item_resource in ItemResourceType::get_all() {
            result.push(FlatItemResource(item_resource, 0));
        }

        result
    }

}