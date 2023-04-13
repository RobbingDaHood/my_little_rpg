#[cfg(test)]
mod tests_int {
    use std::collections::HashMap;

    use crate::{
        command::roll_modifier::execute_craft,
        generator::game::new_testing,
        the_world::{
            item_modifier::Modifier, item_resource::Type, modifier_cost::Cost, modifier_gain::Gain,
            treasure_types::TreasureType,
        },
    };

    #[test]
    fn basic_test() {
        let mut game = new_testing(Some([1; 16]));
        execute_craft(
            &mut game.random_generator_state,
            &game.inventory[0].as_ref().unwrap().crafting_info,
        );
    }

    #[test]
    fn seeding_test() {
        let mut game = new_testing(Some([1; 16]));
        let original_game = execute_craft(
            &mut game.random_generator_state,
            &game.inventory[0].as_ref().unwrap().crafting_info,
        );

        for _i in 1..1000 {
            let mut game = new_testing(Some([1; 16]));
            let result = execute_craft(
                &mut game.random_generator_state,
                &game.inventory[0].as_ref().unwrap().crafting_info,
            );
            assert_eq!(original_game, result);
        }
    }

    #[test]
    fn test_many_runs() {
        let mut game = new_testing(Some([1; 16]));
        let mut cost_modifiers: HashMap<Cost, u32> = HashMap::new();
        let mut gain_modifiers: HashMap<Gain, u32> = HashMap::new();

        for _i in 1..1000 {
            let result = execute_craft(
                &mut game.random_generator_state,
                &game.inventory[0].as_ref().unwrap().crafting_info,
            );

            setup_costs(&mut cost_modifiers, &result);

            setup_gains(&mut gain_modifiers, result);
        }

        assert_eq!(
            0,
            game.difficulty
                .min_resistance
                .keys()
                .cloned()
                .filter(|attack_type| {
                    cost_modifiers
                        .get(&Cost::FlatMinAttackRequirement(attack_type.clone(), 0))
                        .unwrap()
                        == &0
                })
                .count()
        );

        assert_eq!(
            0,
            game.difficulty
                .min_resistance
                .keys()
                .cloned()
                .filter(|attack_type| {
                    cost_modifiers
                        .get(&Cost::FlatMaxAttackRequirement(attack_type.clone(), 0))
                        .unwrap()
                        == &0
                })
                .count()
        );

        assert_ne!(
            0,
            *cost_modifiers
                .get(&Cost::FlatSumMinAttackRequirement(0))
                .unwrap()
        );
        assert_ne!(
            0,
            *cost_modifiers
                .get(&Cost::FlatSumMaxAttackRequirement(0))
                .unwrap()
        );

        assert_ne!(
            0,
            *cost_modifiers
                .get(&Cost::PlaceLimitedByIndexModulus(1, Vec::new()))
                .unwrap()
        );

        assert_eq!(
            0,
            Type::get_all()
                .into_iter()
                .filter(|item_resource| {
                    cost_modifiers
                        .get(&Cost::FlatItemResource(item_resource.clone(), 0))
                        .unwrap()
                        == &0
                })
                .count()
        );

        assert_eq!(
            0,
            Type::get_all()
                .into_iter()
                .filter(|item_resource| {
                    cost_modifiers
                        .get(&Cost::FlatMinItemResourceRequirement(
                            item_resource.clone(),
                            0,
                        ))
                        .unwrap()
                        == &0
                })
                .count()
        );

        assert_eq!(
            0,
            Type::get_all()
                .into_iter()
                .filter(|item_resource| {
                    cost_modifiers
                        .get(&Cost::FlatMaxItemResourceRequirement(
                            item_resource.clone(),
                            0,
                        ))
                        .unwrap()
                        == &0
                })
                .count()
        );

        assert_eq!(
            0,
            game.difficulty
                .min_resistance
                .keys()
                .cloned()
                .filter(|attack_type| {
                    cost_modifiers
                        .get(&Cost::FlatMinResistanceRequirement(attack_type.clone(), 0))
                        .unwrap()
                        == &0
                })
                .count()
        );

        assert_eq!(
            0,
            game.difficulty
                .min_resistance
                .keys()
                .cloned()
                .filter(|attack_type| {
                    cost_modifiers
                        .get(&Cost::FlatMaxResistanceRequirement(attack_type.clone(), 0))
                        .unwrap()
                        == &0
                })
                .count()
        );

        assert_ne!(
            0,
            *cost_modifiers
                .get(&Cost::FlatMinSumResistanceRequirement(0))
                .unwrap()
        );
        assert_ne!(
            0,
            *cost_modifiers
                .get(&Cost::FlatMaxSumResistanceRequirement(0))
                .unwrap()
        );
        assert_ne!(0, *cost_modifiers.get(&Cost::MinWinsInARow(0)).unwrap());
        assert_ne!(0, *cost_modifiers.get(&Cost::MaxWinsInARow(0)).unwrap());

        assert_eq!(
            0,
            game.difficulty
                .min_resistance
                .keys()
                .cloned()
                .filter(|attack_type| {
                    gain_modifiers
                        .get(&Gain::FlatDamage(attack_type.clone(), 0))
                        .unwrap()
                        == &0
                })
                .count()
        );

        assert_eq!(
            0,
            Type::get_all()
                .into_iter()
                .filter(|item_resource| {
                    gain_modifiers
                        .get(&Gain::FlatItemResource(item_resource.clone(), 0))
                        .unwrap()
                        == &0
                })
                .count()
        );

        assert_eq!(
            0,
            game.difficulty
                .min_resistance
                .keys()
                .cloned()
                .filter(|attack_type| {
                    gain_modifiers
                        .get(&Gain::PercentageIncreaseDamage(attack_type.clone(), 0))
                        .unwrap()
                        == &0
                })
                .count()
        );

        assert_eq!(
            0,
            game.difficulty
                .min_resistance
                .keys()
                .cloned()
                .filter(|attack_type| {
                    gain_modifiers
                        .get(&Gain::FlatResistanceReduction(attack_type.clone(), 0))
                        .unwrap()
                        == &0
                })
                .count()
        );

        assert_eq!(
            0,
            game.difficulty
                .min_resistance
                .keys()
                .cloned()
                .filter(|attack_type| {
                    gain_modifiers
                        .get(&Gain::PercentageIncreaseResistanceReduction(
                            attack_type.clone(),
                            0,
                        ))
                        .unwrap()
                        == &0
                })
                .count()
        );

        assert_ne!(
            0,
            *gain_modifiers
                .get(&Gain::FlatDamageAgainstHighestResistance(0))
                .unwrap()
        );
        assert_ne!(
            0,
            *gain_modifiers
                .get(&Gain::PercentageIncreaseDamageAgainstHighestResistance(0))
                .unwrap()
        );
        assert_ne!(
            0,
            *gain_modifiers
                .get(&Gain::FlatDamageAgainstLowestResistance(0))
                .unwrap()
        );
        assert_ne!(
            0,
            *gain_modifiers
                .get(&Gain::PercentageIncreaseDamageAgainstLowestResistance(0))
                .unwrap()
        );

        assert_eq!(
            0,
            TreasureType::get_all()
                .into_iter()
                .filter(|treasure_type| {
                    gain_modifiers
                        .get(&Gain::PercentageIncreaseTreasure(treasure_type.clone(), 0))
                        .unwrap()
                        == &0
                })
                .count()
        );

        assert_ne!(
            0,
            *gain_modifiers
                .get(&Gain::FlatIncreaseRewardedItems(0))
                .unwrap()
        );
    }

    fn setup_gains(
        gain_modifiers: &mut HashMap<Gain, u32>,
        result: Modifier,
    ) {
        for gain in result.gains {
            match gain {
                Gain::FlatItemResource(item_resource, _) => {
                    let token = Gain::FlatItemResource(item_resource, 0);
                    *gain_modifiers.entry(token).or_insert(0) += 1;
                }
                Gain::FlatDamage(attack_type, _) => {
                    let token = Gain::FlatDamage(attack_type, 0);
                    *gain_modifiers.entry(token).or_insert(0) += 1;
                }
                Gain::PercentageIncreaseDamage(attack_type, _) => {
                    let token = Gain::PercentageIncreaseDamage(attack_type, 0);
                    *gain_modifiers.entry(token).or_insert(0) += 1;
                }
                Gain::FlatResistanceReduction(item_resource, _) => {
                    let token = Gain::FlatResistanceReduction(item_resource, 0);
                    *gain_modifiers.entry(token).or_insert(0) += 1;
                }
                Gain::PercentageIncreaseResistanceReduction(item_resource, _) => {
                    let token = Gain::PercentageIncreaseResistanceReduction(item_resource, 0);
                    *gain_modifiers.entry(token).or_insert(0) += 1;
                }
                Gain::FlatDamageAgainstHighestResistance(_) => {
                    let token = Gain::FlatDamageAgainstHighestResistance(0);
                    *gain_modifiers.entry(token).or_insert(0) += 1;
                }
                Gain::PercentageIncreaseDamageAgainstHighestResistance(_) => {
                    let token = Gain::PercentageIncreaseDamageAgainstHighestResistance(0);
                    *gain_modifiers.entry(token).or_insert(0) += 1;
                }
                Gain::FlatDamageAgainstLowestResistance(_) => {
                    let token = Gain::FlatDamageAgainstLowestResistance(0);
                    *gain_modifiers.entry(token).or_insert(0) += 1;
                }
                Gain::PercentageIncreaseDamageAgainstLowestResistance(_) => {
                    let token = Gain::PercentageIncreaseDamageAgainstLowestResistance(0);
                    *gain_modifiers.entry(token).or_insert(0) += 1;
                }
                Gain::PercentageIncreaseTreasure(treasure_type, _) => {
                    let token = Gain::PercentageIncreaseTreasure(treasure_type, 0);
                    *gain_modifiers.entry(token).or_insert(0) += 1;
                }
                Gain::FlatIncreaseRewardedItems(_) => {
                    let token = Gain::FlatIncreaseRewardedItems(0);
                    *gain_modifiers.entry(token).or_insert(0) += 1;
                }
            }
        }
    }

    fn setup_costs(
        cost_modifiers: &mut HashMap<Cost, u32>,
        result: &Modifier,
    ) {
        for cost in result.costs.clone() {
            match cost {
                Cost::FlatItemResource(item_resource, _) => {
                    let token = Cost::FlatItemResource(item_resource, 0);
                    *cost_modifiers.entry(token).or_insert(0) += 1;
                }
                Cost::FlatMinItemResourceRequirement(item_resource, _) => {
                    let token = Cost::FlatMinItemResourceRequirement(item_resource, 0);
                    *cost_modifiers.entry(token).or_insert(0) += 1;
                }
                Cost::FlatMaxItemResourceRequirement(item_resource, _) => {
                    let token = Cost::FlatMaxItemResourceRequirement(item_resource, 0);
                    *cost_modifiers.entry(token).or_insert(0) += 1;
                }
                Cost::FlatMinAttackRequirement(attack_type, _) => {
                    let token = Cost::FlatMinAttackRequirement(attack_type, 0);
                    *cost_modifiers.entry(token).or_insert(0) += 1;
                }
                Cost::FlatMaxAttackRequirement(attack_type, _) => {
                    let token = Cost::FlatMaxAttackRequirement(attack_type, 0);
                    *cost_modifiers.entry(token).or_insert(0) += 1;
                }
                Cost::PlaceLimitedByIndexModulus(_, _) => {
                    let token = Cost::PlaceLimitedByIndexModulus(1, Vec::new());
                    *cost_modifiers.entry(token).or_insert(0) += 1;
                }
                Cost::FlatSumMinAttackRequirement(_) => {
                    let token = Cost::FlatSumMinAttackRequirement(0);
                    *cost_modifiers.entry(token).or_insert(0) += 1;
                }
                Cost::FlatSumMaxAttackRequirement(_) => {
                    let token = Cost::FlatSumMaxAttackRequirement(0);
                    *cost_modifiers.entry(token).or_insert(0) += 1;
                }
                Cost::FlatMinResistanceRequirement(attack_type, _) => {
                    let token = Cost::FlatMinResistanceRequirement(attack_type, 0);
                    *cost_modifiers.entry(token).or_insert(0) += 1;
                }
                Cost::FlatMaxResistanceRequirement(attack_type, _) => {
                    let token = Cost::FlatMaxResistanceRequirement(attack_type, 0);
                    *cost_modifiers.entry(token).or_insert(0) += 1;
                }
                Cost::FlatMinSumResistanceRequirement(_) => {
                    let token = Cost::FlatMinSumResistanceRequirement(0);
                    *cost_modifiers.entry(token).or_insert(0) += 1;
                }
                Cost::FlatMaxSumResistanceRequirement(_) => {
                    let token = Cost::FlatMaxSumResistanceRequirement(0);
                    *cost_modifiers.entry(token).or_insert(0) += 1;
                }
                Cost::MinWinsInARow(_) => {
                    let token = Cost::MinWinsInARow(0);
                    *cost_modifiers.entry(token).or_insert(0) += 1;
                }
                Cost::MaxWinsInARow(_) => {
                    let token = Cost::MaxWinsInARow(0);
                    *cost_modifiers.entry(token).or_insert(0) += 1;
                }
            }
        }
    }
}
