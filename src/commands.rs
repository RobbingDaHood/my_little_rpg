use crate::commands::Command::{AddModifier, Equip, ExpandElements, ExpandEquipmentSlots, ExpandMaxElement, ExpandMinElement, ExpandPlaces, Help, Move, ReduceDifficulty, RerollModifier, State, SwapEquipment};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Eq, Hash)]
pub enum Command {
    State,
    Move(usize),
    Equip(usize, usize),
    SwapEquipment(usize, usize),
    RerollModifier(usize, usize, Vec<usize>),
    ExpandPlaces,
    ExpandElements,
    ExpandMaxElement,
    ExpandMinElement,
    ExpandMaxSimultaneousElement,
    ExpandMinSimultaneousElement,
    ExpandEquipmentSlots,
    ReduceDifficulty,
    AddModifier(usize),
    Help,
}

impl Command {
    pub fn get_all() -> Vec<Command> {
        vec![
            State,
            Move(0),
            Equip(0, 0),
            SwapEquipment(0, 0),
            RerollModifier(0, 0, Vec::new()),
            ExpandPlaces,
            ExpandElements,
            ExpandMaxElement,
            ExpandMinElement,
            ExpandEquipmentSlots,
            ReduceDifficulty,
            AddModifier(0),
            Help,
        ]
    }
}

impl TryFrom<&String> for Command {
    type Error = String;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let command_parts = value.split(" ").collect::<Vec<&str>>();

        if command_parts.len() == 0 {
            return Err("Command is empty".to_string());
        }

        return match command_parts[0] {
            "State" => Ok(Command::State),
            "ExpandPlaces" => Ok(Command::ExpandPlaces),
            "ExpandElements" => Ok(Command::ExpandElements),
            "ExpandMaxElement" => Ok(Command::ExpandMaxElement),
            "ExpandMinElement" => Ok(Command::ExpandMinElement),
            "ExpandEquipmentSlots" => Ok(Command::ExpandEquipmentSlots),
            "ReduceDifficulty" => Ok(Command::ReduceDifficulty),
            "ExpandMaxSimultaneousElement" => Ok(Command::ExpandMaxSimultaneousElement),
            "ExpandMinSimultaneousElement" => Ok(Command::ExpandMinSimultaneousElement),
            "Help" => Ok(Command::Help),
            "Move" => {
                if command_parts.len() < 2 {
                    return Err(format!("Trouble parsing move command, it needs the index of the place. Got {:?}", command_parts));
                }

                return if let Ok(place_index) = command_parts[1].parse::<usize>() {
                    Ok(Move(place_index))
                } else {
                    Err(format!("Trouble parsing move command, it needs the index of the place. Got {:?}", command_parts))
                };
            }
            "AddModifier" => {
                if command_parts.len() < 2 {
                    return Err(format!("Trouble parsing AddModifier command, it needs the index of the item. Got {:?}", command_parts));
                }

                return if let Ok(inventory_position) = command_parts[1].parse::<usize>() {
                    Ok(AddModifier(inventory_position))
                } else {
                    return Err(format!("Trouble parsing AddModifier command, it needs the index of the item. Got {:?}", command_parts));
                };
            }
            "Equip" => {
                if command_parts.len() < 3 {
                    return Err(format!("Trouble parsing Equip command, it needs index of inventory and index of equipment slot. Got {:?}", command_parts));
                }

                return if let Ok(inventory_position) = command_parts[1].parse::<usize>() {
                    if let Ok(equipped_item_position) = command_parts[2].parse::<usize>() {
                        Ok(Equip(inventory_position, equipped_item_position))
                    } else {
                        Err(format!("Trouble parsing Equip command, it needs index of inventory and index of equipment slot. Got {:?}", command_parts))
                    }
                } else {
                    Err(format!("Trouble parsing Equip command, it needs index of inventory and index of equipment slot. Got {:?}", command_parts))
                };
            }
            "SwapEquipment" => {
                if command_parts.len() < 3 {
                    return Err(format!("Trouble parsing SwapEquipment command, it needs index of inventory and index of equipment slot. Got {:?}", command_parts));
                }

                return if let Ok(equipped_item_position_1) = command_parts[1].parse::<usize>() {
                    if let Ok(equipped_item_position_2) = command_parts[2].parse::<usize>() {
                        Ok(SwapEquipment(equipped_item_position_1, equipped_item_position_2))
                    } else {
                        Err(format!("Trouble parsing SwapEquipment command, it needs index of inventory and index of equipment slot. Got {:?}", command_parts))
                    }
                } else {
                    Err(format!("Trouble parsing SwapEquipment command, it needs index of inventory and index of equipment slot. Got {:?}", command_parts))
                };
            }
            "RerollModifier" => {
                let error_message = format!("Trouble parsing RerollModifier command, it needs index of inventory, index of modifier and a list comma seperated list of items to sacrifice. Got {:?}", command_parts);
                if command_parts.len() < 3 {
                    return Err(error_message.clone())
                }

                return if let Ok(inventory_index) = command_parts[1].parse::<usize>() {
                    if let Ok(modifier_index) = command_parts[2].parse::<usize>() {
                        let sacrifice_item_indexes = command_parts[3].split(",").into_iter()
                            .map(|s| s.parse::<usize>())
                            .collect();

                        if let Ok(sacrifice_item_indexes) = sacrifice_item_indexes {
                            Ok(RerollModifier(inventory_index, modifier_index, sacrifice_item_indexes))
                        } else {
                            Err(error_message.clone())
                        }
                    } else {
                        Err(error_message.clone())
                    }
                } else {
                    Err(error_message.clone())
                };
            }
            _ => Err(format!("Command not known. Got {:?}", command_parts))
        };
    }
}

#[cfg(test)]
mod tests_int {
    use crate::commands::Command;

    #[test]
    fn try_from() {
        assert_eq!(Command::State, Command::try_from(&"State".to_string()).unwrap());
        assert_eq!(Command::ExpandPlaces, Command::try_from(&"ExpandPlaces".to_string()).unwrap());
        assert_eq!(Command::ExpandElements, Command::try_from(&"ExpandElements".to_string()).unwrap());
        assert_eq!(Command::ExpandMaxElement, Command::try_from(&"ExpandMaxElement".to_string()).unwrap());
        assert_eq!(Command::ExpandMinElement, Command::try_from(&"ExpandMinElement".to_string()).unwrap());
        assert_eq!(Command::ExpandEquipmentSlots, Command::try_from(&"ExpandEquipmentSlots".to_string()).unwrap());
        assert_eq!(Command::ReduceDifficulty, Command::try_from(&"ReduceDifficulty".to_string()).unwrap());
        assert_eq!(Command::ExpandMaxSimultaneousElement, Command::try_from(&"ExpandMaxSimultaneousElement".to_string()).unwrap());
        assert_eq!(Command::ExpandMinSimultaneousElement, Command::try_from(&"ExpandMinSimultaneousElement".to_string()).unwrap());
        assert_eq!(Command::Help, Command::try_from(&"Help".to_string()).unwrap());

        assert_eq!(Command::Move(22), Command::try_from(&"Move 22".to_string()).unwrap());
        assert_eq!(Err("Trouble parsing move command, it needs the index of the place. Got [\"Move\"]".to_string()), Command::try_from(&"Move".to_string()));
        assert_eq!(Err("Trouble parsing move command, it needs the index of the place. Got [\"Move\", \"-1\"]".to_string()), Command::try_from(&"Move -1".to_string()));
        assert_eq!(Err("Trouble parsing move command, it needs the index of the place. Got [\"Move\", \"B\"]".to_string()), Command::try_from(&"Move B".to_string()));

        assert_eq!(Command::AddModifier(22), Command::try_from(&"AddModifier 22".to_string()).unwrap());
        assert_eq!(Err("Trouble parsing AddModifier command, it needs the index of the item. Got [\"AddModifier\"]".to_string()), Command::try_from(&"AddModifier".to_string()));
        assert_eq!(Err("Trouble parsing AddModifier command, it needs the index of the item. Got [\"AddModifier\", \"-1\"]".to_string()), Command::try_from(&"AddModifier -1".to_string()));
        assert_eq!(Err("Trouble parsing AddModifier command, it needs the index of the item. Got [\"AddModifier\", \"B\"]".to_string()), Command::try_from(&"AddModifier B".to_string()));

        assert_eq!(Command::Equip(21, 22), Command::try_from(&"Equip 21 22".to_string()).unwrap());
        assert_eq!(Err("Trouble parsing Equip command, it needs index of inventory and index of equipment slot. Got [\"Equip\"]".to_string()), Command::try_from(&"Equip".to_string()));
        assert_eq!(Err("Trouble parsing Equip command, it needs index of inventory and index of equipment slot. Got [\"Equip\", \"-1\", \"22\"]".to_string()), Command::try_from(&"Equip -1 22".to_string()));
        assert_eq!(Err("Trouble parsing Equip command, it needs index of inventory and index of equipment slot. Got [\"Equip\", \"21\", \"-1\"]".to_string()), Command::try_from(&"Equip 21 -1".to_string()));
        assert_eq!(Err("Trouble parsing Equip command, it needs index of inventory and index of equipment slot. Got [\"Equip\", \"B\", \"22\"]".to_string()), Command::try_from(&"Equip B 22".to_string()));
        assert_eq!(Err("Trouble parsing Equip command, it needs index of inventory and index of equipment slot. Got [\"Equip\", \"21\", \"B\"]".to_string()), Command::try_from(&"Equip 21 B".to_string()));

        assert_eq!(Command::SwapEquipment(21, 22), Command::try_from(&"SwapEquipment 21 22".to_string()).unwrap());
        assert_eq!(Err("Trouble parsing SwapEquipment command, it needs index of inventory and index of equipment slot. Got [\"SwapEquipment\"]".to_string()), Command::try_from(&"SwapEquipment".to_string()));
        assert_eq!(Err("Trouble parsing SwapEquipment command, it needs index of inventory and index of equipment slot. Got [\"SwapEquipment\", \"-1\", \"22\"]".to_string()), Command::try_from(&"SwapEquipment -1 22".to_string()));
        assert_eq!(Err("Trouble parsing SwapEquipment command, it needs index of inventory and index of equipment slot. Got [\"SwapEquipment\", \"21\", \"-1\"]".to_string()), Command::try_from(&"SwapEquipment 21 -1".to_string()));
        assert_eq!(Err("Trouble parsing SwapEquipment command, it needs index of inventory and index of equipment slot. Got [\"SwapEquipment\", \"B\", \"22\"]".to_string()), Command::try_from(&"SwapEquipment B 22".to_string()));
        assert_eq!(Err("Trouble parsing SwapEquipment command, it needs index of inventory and index of equipment slot. Got [\"SwapEquipment\", \"21\", \"B\"]".to_string()), Command::try_from(&"SwapEquipment 21 B".to_string()));

        assert_eq!(Command::RerollModifier(21, 22, vec![1,2,3]), Command::try_from(&"RerollModifier 21 22 1,2,3".to_string()).unwrap());
        assert_eq!(Err("Trouble parsing RerollModifier command, it needs index of inventory, index of modifier and a list comma seperated list of items to sacrifice. Got [\"RerollModifier\"]".to_string()), Command::try_from(&"RerollModifier".to_string()));
        assert_eq!(Err("Trouble parsing RerollModifier command, it needs index of inventory, index of modifier and a list comma seperated list of items to sacrifice. Got [\"RerollModifier\", \"-1\"]".to_string()), Command::try_from(&"RerollModifier -1".to_string()));
        assert_eq!(Err("Trouble parsing RerollModifier command, it needs index of inventory, index of modifier and a list comma seperated list of items to sacrifice. Got [\"RerollModifier\", \"-1\", \"22\"]".to_string()), Command::try_from(&"RerollModifier -1 22".to_string()));
        assert_eq!(Err("Trouble parsing RerollModifier command, it needs index of inventory, index of modifier and a list comma seperated list of items to sacrifice. Got [\"RerollModifier\", \"-1\", \"22\", \"1,2,3\"]".to_string()), Command::try_from(&"RerollModifier -1 22 1,2,3".to_string()));
        assert_eq!(Err("Trouble parsing RerollModifier command, it needs index of inventory, index of modifier and a list comma seperated list of items to sacrifice. Got [\"RerollModifier\", \"21\", \"-1\", \"1,2,3\"]".to_string()), Command::try_from(&"RerollModifier 21 -1 1,2,3".to_string()));
        assert_eq!(Err("Trouble parsing RerollModifier command, it needs index of inventory, index of modifier and a list comma seperated list of items to sacrifice. Got [\"RerollModifier\", \"B\", \"22\", \"1,2,3\"]".to_string()), Command::try_from(&"RerollModifier B 22 1,2,3".to_string()));
        assert_eq!(Err("Trouble parsing RerollModifier command, it needs index of inventory, index of modifier and a list comma seperated list of items to sacrifice. Got [\"RerollModifier\", \"21\", \"B\", \"1,2,3\"]".to_string()), Command::try_from(&"RerollModifier 21 B 1,2,3".to_string()));
        assert_eq!(Err("Trouble parsing RerollModifier command, it needs index of inventory, index of modifier and a list comma seperated list of items to sacrifice. Got [\"RerollModifier\", \"21\", \"B\", \"a\"]".to_string()), Command::try_from(&"RerollModifier 21 B a".to_string()));

        assert_eq!(Err("Command not known. Got [\"InvalidCommand\"]".to_string()), Command::try_from(&"InvalidCommand".to_string()));
    }
}