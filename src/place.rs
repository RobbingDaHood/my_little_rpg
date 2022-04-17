use std::collections::HashMap;
use crate::attack_types;
use crate::crafting_materials::CraftingMaterial;

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Place {
    resistance: HashMap<attack_types::AttackType, u64>,
    reward: HashMap<CraftingMaterial, u64>,
}

impl Place {
    pub fn claim_rewards(&self, attacks: HashMap<attack_types::AttackType, u64>) -> Option<HashMap<CraftingMaterial, u64>> {
        let are_all_resistance_defeated = self.resistance.iter()
            .all(|(resistance_type, resistance_value)|
                match attacks.get(resistance_type) {
                    None => false,
                    Some(attack_value) => {
                        attack_value >= resistance_value
                    }
                }
            );
        if are_all_resistance_defeated { Some(self.reward.clone()) } else { None }
    }
}

#[cfg(test)]
mod tests_int {
    use std::collections::HashMap;
    use crate::place::Place;

    #[test]
    fn claim_rewards_no_resistance() {
        let reward = HashMap::new();
        let place = Place {
            resistance: HashMap::new(),
            reward: reward.clone(),
        };

        assert_eq!(Some(reward), place.claim_rewards(HashMap::new()));
    }
}
