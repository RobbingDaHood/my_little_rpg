use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::the_world::{damage_types, difficulty::Difficulty, treasure_types::TreasureType};

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Place {
    pub(crate) resistance: HashMap<damage_types::DamageType, u64>,
    pub(crate) reward: HashMap<TreasureType, u64>,
    pub(crate) item_reward_possible_rolls: Difficulty,
}

impl Place {
    //TODO consider moving function and tests
    pub fn claim_rewards(
        &self,
        attacks: &HashMap<&damage_types::DamageType, u64>,
    ) -> Option<HashMap<TreasureType, u64>> {
        let are_all_resistance_defeated =
            self.resistance
                .iter()
                .all(|(resistance_type, resistance_value)| {
                    match attacks.get(resistance_type) {
                        None => false,
                        Some(attack_value) => attack_value >= resistance_value,
                    }
                });
        if are_all_resistance_defeated {
            Some(self.reward.clone())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests_int {
    use std::collections::HashMap;

    use crate::the_world::{damage_types::DamageType, difficulty::Difficulty, place::Place};

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

        resistance.insert(DamageType::Physical, 1);
        resistance.insert(DamageType::Fire, 2);
        resistance.insert(DamageType::Frost, 3);
        resistance.insert(DamageType::Lightning, 4);
        resistance.insert(DamageType::Light, 5);
        resistance.insert(DamageType::Darkness, 6);
        resistance.insert(DamageType::Nature, 7);
        resistance.insert(DamageType::Corruption, 8);
        resistance.insert(DamageType::Holy, 9);

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

        let attacks: HashMap<&DamageType, u64> = resistance
            .iter()
            .map(|(attack_stype, amount)| (attack_stype, *amount))
            .collect();

        assert_eq!(Some(reward), place.claim_rewards(&attacks));
    }

    #[test]
    fn claim_rewards_all_resistance_one_missing() {
        let reward = HashMap::new();
        let mut resistance = HashMap::new();

        resistance.insert(DamageType::Physical, 1);
        resistance.insert(DamageType::Fire, 2);
        resistance.insert(DamageType::Frost, 3);
        resistance.insert(DamageType::Lightning, 4);
        resistance.insert(DamageType::Light, 5);
        resistance.insert(DamageType::Darkness, 6);
        resistance.insert(DamageType::Nature, 7);
        resistance.insert(DamageType::Corruption, 8);
        resistance.insert(DamageType::Holy, 9);

        let item_reward_possible_rolls = Difficulty {
            max_resistance: HashMap::new(),
            min_resistance: HashMap::new(),
            max_simultaneous_resistances: 0,
            min_simultaneous_resistances: 0,
        };

        let place = Place {
            resistance: resistance.clone(),
            reward,
            item_reward_possible_rolls,
        };

        resistance.remove(&DamageType::Physical);

        let attacks: HashMap<&DamageType, u64> = resistance
            .iter()
            .map(|(attack_stype, amount)| (attack_stype, *amount))
            .collect();

        assert_eq!(None, place.claim_rewards(&attacks));
    }
}
