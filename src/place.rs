use std::collections::HashMap;
use crate::attack_types;
use crate::treasure_types::TreasureType;

use serde::{Deserialize, Serialize};
use crate::difficulty::Difficulty;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Place {
    pub(crate) resistance: HashMap<attack_types::AttackType, u64>,
    pub(crate) reward: HashMap<TreasureType, u64>,
    pub(crate) item_reward_possible_rolls: Difficulty,
}

impl Place {
    pub fn claim_rewards(&self, attacks: &HashMap<attack_types::AttackType, u64>) -> Option<HashMap<TreasureType, u64>> {
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
    use crate::attack_types::AttackType;
    use crate::difficulty::Difficulty;
    use crate::place::Place;

    #[test]
    fn claim_rewards_no_resistance() {
        let reward = HashMap::new();
        let item_reward_possible_rolls = Difficulty {
            max_resistance: HashMap::new(),
            min_resistance: HashMap::new(),
            max_simultaneous_resistances: 0,
            min_simultaneous_resistances: 0,

        };
        let place = Place {
            resistance: HashMap::new(),
            reward: reward.clone(),
            item_reward_possible_rolls,
        };

        assert_eq!(Some(reward), place.claim_rewards(&HashMap::new()));
    }

    #[test]
    fn claim_rewards_all_resistance() {
        let reward = HashMap::new();
        let mut resistance = HashMap::new();

        resistance.insert(AttackType::Physical, 1);
        resistance.insert(AttackType::Fire, 2);
        resistance.insert(AttackType::Frost, 3);
        resistance.insert(AttackType::Lightning, 4);
        resistance.insert(AttackType::Light, 5);
        resistance.insert(AttackType::Darkness, 6);
        resistance.insert(AttackType::Nature, 7);
        resistance.insert(AttackType::Corruption, 8);
        resistance.insert(AttackType::Holy, 9);

        let item_reward_possible_rolls = Difficulty {
            max_resistance: HashMap::new(),
            min_resistance: HashMap::new(),
            max_simultaneous_resistances: 0,
            min_simultaneous_resistances: 0,

        };

        let place = Place {
            resistance: resistance.clone(),
            reward: reward.clone(),
            item_reward_possible_rolls,
        };

        assert_eq!(Some(reward), place.claim_rewards(&resistance));
    }

    #[test]
    fn claim_rewards_all_resistance_one_missing() {
        let reward = HashMap::new();
        let mut resistance = HashMap::new();

        resistance.insert(AttackType::Physical, 1);
        resistance.insert(AttackType::Fire, 2);
        resistance.insert(AttackType::Frost, 3);
        resistance.insert(AttackType::Lightning, 4);
        resistance.insert(AttackType::Light, 5);
        resistance.insert(AttackType::Darkness, 6);
        resistance.insert(AttackType::Nature, 7);
        resistance.insert(AttackType::Corruption, 8);
        resistance.insert(AttackType::Holy, 9);

        let item_reward_possible_rolls = Difficulty {
            max_resistance: HashMap::new(),
            min_resistance: HashMap::new(),
            max_simultaneous_resistances: 0,
            min_simultaneous_resistances: 0,

        };

        let place = Place {
            resistance: resistance.clone(),
            reward: reward.clone(),
            item_reward_possible_rolls,
        };

        resistance.remove(&AttackType::Physical);

        assert_eq!(None, place.claim_rewards(&resistance));
    }
}
