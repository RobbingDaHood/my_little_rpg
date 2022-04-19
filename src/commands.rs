use crate::commands::Command::{Equip, Move, RerollModifier, SwapEquipment};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Eq, Hash)]
pub enum Command {
    State,
    Move(usize),
    Equip(usize, usize),
    SwapEquipment(usize, usize),
    CreateItem,
    RerollModifier(usize, usize),
    ExpandPlaces,
    ExpandElements,
    ExpandMaxElement,
    ExpandMinElement,
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
            "CreateItem" => Ok(Command::CreateItem),
            "ExpandPlaces" => Ok(Command::ExpandPlaces),
            "ExpandElements" => Ok(Command::ExpandElements),
            "ExpandMaxElement" => Ok(Command::ExpandMaxElement),
            "ExpandMinElement" => Ok(Command::ExpandMinElement),
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
                if command_parts.len() < 3 {
                    return Err(format!("Trouble parsing RerollModifier command, it needs index of inventory and index of modifier. Got {:?}", command_parts));
                }

                return if let Ok(inventory_index) = command_parts[1].parse::<usize>() {
                    if let Ok(modifier_index) = command_parts[2].parse::<usize>() {
                        Ok(RerollModifier(inventory_index, modifier_index))
                    } else {
                        Err(format!("Trouble parsing RerollModifier command, it needs index of inventory and index of modifier. Got {:?}", command_parts))
                    }
                } else {
                    Err(format!("Trouble parsing RerollModifier command, it needs index of inventory and index of modifier. Got {:?}", command_parts))
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
        assert_eq!(Command::CreateItem, Command::try_from(&"CreateItem".to_string()).unwrap());
        assert_eq!(Command::ExpandPlaces, Command::try_from(&"ExpandPlaces".to_string()).unwrap());
        assert_eq!(Command::ExpandElements, Command::try_from(&"ExpandElements".to_string()).unwrap());
        assert_eq!(Command::ExpandMaxElement, Command::try_from(&"ExpandMaxElement".to_string()).unwrap());
        assert_eq!(Command::ExpandMinElement, Command::try_from(&"ExpandMinElement".to_string()).unwrap());

        assert_eq!(Command::Move(22), Command::try_from(&"Move 22".to_string()).unwrap());
        assert_eq!(Err("Trouble parsing move command, it needs the index of the place. Got [\"Move\"]".to_string()), Command::try_from(&"Move".to_string()));
        assert_eq!(Err("Trouble parsing move command, it needs the index of the place. Got [\"Move\", \"-1\"]".to_string()), Command::try_from(&"Move -1".to_string()));
        assert_eq!(Err("Trouble parsing move command, it needs the index of the place. Got [\"Move\", \"B\"]".to_string()), Command::try_from(&"Move B".to_string()));

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

        assert_eq!(Command::RerollModifier(21, 22), Command::try_from(&"RerollModifier 21 22".to_string()).unwrap());
        assert_eq!(Err("Trouble parsing RerollModifier command, it needs index of inventory and index of modifier. Got [\"RerollModifier\"]".to_string()), Command::try_from(&"RerollModifier".to_string()));
        assert_eq!(Err("Trouble parsing RerollModifier command, it needs index of inventory and index of modifier. Got [\"RerollModifier\", \"-1\", \"22\"]".to_string()), Command::try_from(&"RerollModifier -1 22".to_string()));
        assert_eq!(Err("Trouble parsing RerollModifier command, it needs index of inventory and index of modifier. Got [\"RerollModifier\", \"21\", \"-1\"]".to_string()), Command::try_from(&"RerollModifier 21 -1".to_string()));
        assert_eq!(Err("Trouble parsing RerollModifier command, it needs index of inventory and index of modifier. Got [\"RerollModifier\", \"B\", \"22\"]".to_string()), Command::try_from(&"RerollModifier B 22".to_string()));
        assert_eq!(Err("Trouble parsing RerollModifier command, it needs index of inventory and index of modifier. Got [\"RerollModifier\", \"21\", \"B\"]".to_string()), Command::try_from(&"RerollModifier 21 B".to_string()));

        assert_eq!(Err("Command not known. Got [\"InvalidCommand\"]".to_string()), Command::try_from(&"InvalidCommand".to_string()));
    }
}