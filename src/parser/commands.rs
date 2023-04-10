pub use crate::command::commands::Command;
use crate::command::commands::Command::{AddModifier, Equip, ExpandElements, ExpandEquipmentSlots, ExpandMaxElement, ExpandMaxSimultaneousElement, ExpandMinElement, ExpandMinSimultaneousElement, ExpandPlaces, Help, LoadTheWorld, Move, ReduceDifficulty, ReorderInventory, RerollModifier, SaveTheWorld, State, SwapEquipment};
use crate::my_little_rpg_errors::MyError;
use crate::the_world::index_specifier::IndexSpecifier;

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
            ExpandMaxSimultaneousElement,
            ExpandMinSimultaneousElement,
            ExpandEquipmentSlots,
            ReduceDifficulty,
            AddModifier(0, Vec::new()),
            Help,
            ReorderInventory,
            SaveTheWorld("String".into(), None),
            LoadTheWorld("String".into(), None),
        ]
    }

    fn try_parse_possible_relative_indexes(command_parts: &str, relative_too: usize) -> Result<Vec<IndexSpecifier>, MyError> {
        command_parts.split(',').into_iter()
            .map(|s| {
                match s.chars().next() {
                    Some('+') => {
                        Self::try_parse_usize(&s[1..s.len()])
                            .map(|relative_index_diff| match relative_too.checked_add(relative_index_diff) {
                                Some(_) => Ok(IndexSpecifier::RelativePositive(relative_index_diff)),
                                None => {
                                    let error_message = format!("{}{} created an overflow!", relative_too, s);
                                    Err(MyError::create_parse_command_error(error_message))
                                }
                            })
                            .and_then(|i| i)
                    }
                    Some('-') => {
                        Self::try_parse_usize(&s[1..s.len()])
                            .map(|relative_index_diff| match relative_too.checked_sub(relative_index_diff) {
                                Some(_) => Ok(IndexSpecifier::RelativeNegative(relative_index_diff)),
                                None => {
                                    let error_message = format!("{}{} created an underflow!", relative_too, s);
                                    Err(MyError::create_parse_command_error(error_message))
                                }
                            })
                            .and_then(|i| i)
                    }
                    _ => {
                        Self::try_parse_usize(s)
                            .map(IndexSpecifier::Absolute)
                    }
                }
            })
            .collect()
    }

    fn try_parse_move(command_parts: &Vec<&str>) -> Result<Command, MyError> {
        if command_parts.len() < 2 {
            let error_message = format!("Trouble parsing move command, it needs the index of the place. Got {:?}", command_parts);
            Err(MyError::create_parse_command_error(error_message))
        } else {
            Self::try_parse_usize(command_parts[1])
                .map(Move)
        }
    }

    //TODO consider if multiple try_parse can be done in one method
    fn try_parse_usize(string_to_parse: &str) -> Result<usize, MyError> {
        string_to_parse.parse::<usize>()
            .map_err(|error| {
                let error_message = format!("The following parameter {}, got the following error while parsing: {:?}", string_to_parse, error);
                MyError::create_parse_command_error(error_message)
            })
    }

    fn try_parse_string(string_to_parse: &str) -> Result<Box<str>, MyError> {
        string_to_parse.parse::<String>()
            .map_err(|error| {
                let error_message = format!("The following parameter {}, got the following error while parsing: {:?}", string_to_parse, error);
                MyError::create_parse_command_error(error_message)
            })
            .map(String::into)
    }

    fn try_parse_add_modifier(command_parts: &Vec<&str>) -> Result<Command, MyError> {
        if command_parts.len() < 2 {
            let error_message = format!("Trouble parsing AddModifier command, it needs the index of the item and a list comma seperated list of items to sacrifice. Got {:?}", command_parts);
            Err(MyError::create_parse_command_error(error_message))
        } else {
            let inventory_position = Self::try_parse_usize(command_parts[1])?;
            Self::try_parse_possible_relative_indexes(command_parts[2], inventory_position)
                .map(|parsed_sacrifice_item_indexes| AddModifier(inventory_position, parsed_sacrifice_item_indexes))
        }
    }

    fn try_parse_equip(command_parts: &Vec<&str>) -> Result<Command, MyError> {
        if command_parts.len() < 3 {
            let error_message = format!("Trouble parsing Equip command, it needs index of inventory and index of equipment slot. Got {:?}", command_parts);
            return Err(MyError::create_parse_command_error(error_message));
        }

        let inventory_position = Self::try_parse_usize(command_parts[1])?;
        let equipped_item_position = Self::try_parse_usize(command_parts[2])?;
        Ok(Equip(inventory_position, equipped_item_position))
    }

    fn try_parse_swap_equipment(command_parts: &Vec<&str>) -> Result<Command, MyError> {
        if command_parts.len() < 3 {
            let error_message = format!("Trouble parsing SwapEquipment command, it needs index of inventory and index of equipment slot. Got {:?}", command_parts);
            return Err(MyError::create_parse_command_error(error_message));
        }

        let equipped_item_position_1 = Self::try_parse_usize(command_parts[1])?;
        let equipped_item_position_2 = Self::try_parse_usize(command_parts[2])?;
        Ok(SwapEquipment(equipped_item_position_1, equipped_item_position_2))
    }

    fn try_parse_reroll_modifier(command_parts: &Vec<&str>) -> Result<Command, MyError> {
        if command_parts.len() < 4 {
            let error_message = format!("Trouble parsing RerollModifier command, it needs index of inventory, index of modifier and a list comma seperated list of items to sacrifice. Got {:?}", command_parts);
            return Err(MyError::create_parse_command_error(error_message));
        }

        let inventory_index = Self::try_parse_usize(command_parts[1])?;
        let modifier_index = Self::try_parse_usize(command_parts[2])?;
        let parsed_sacrifice_item_indexes = Self::try_parse_possible_relative_indexes(command_parts[3], inventory_index)?;
        Ok(RerollModifier(inventory_index, modifier_index, parsed_sacrifice_item_indexes))
    }

    fn try_parse_save_the_world(command_parts: &Vec<&str>) -> Result<Command, MyError> {
        if command_parts.len() < 2 {
            let error_message = format!("Trouble parsing SaveTheWorld command, it needs a save game name and optionally a path to the savegame (remember to end the path with /). Default location is ./save_games/. Got {:?}", command_parts);
            return Err(MyError::create_parse_command_error(error_message));
        }

        let save_game_name = Self::try_parse_string(command_parts[1])?;
        let save_game_path = if command_parts.len() < 3 {
            None
        } else {
            Some(Self::try_parse_string(command_parts[2])?)
        };
        Ok(SaveTheWorld(save_game_name, save_game_path))
    }

    fn try_parse_load_the_world(command_parts: &Vec<&str>) -> Result<Command, MyError> {
        if command_parts.len() < 2 {
            let error_message = format!("Trouble parsing LoadTheWorld command, it needs a save game name and optionally a path to the savegame (remember to end the path with /). Default location is ./save_games/. Got {:?}", command_parts);
            return Err(MyError::create_parse_command_error(error_message));
        }

        let save_game_name = Self::try_parse_string(command_parts[1])?;
        let save_game_path = if command_parts.len() < 3 {
            None
        } else {
            Some(Self::try_parse_string(command_parts[2])?)
        };
        Ok(LoadTheWorld(save_game_name, save_game_path))
    }
}

//TODO Could be interesting to move parsing of individual commands out of this file
impl TryFrom<Box<str>> for Command {
    type Error = MyError;

    fn try_from(value: Box<str>) -> Result<Self, Self::Error> {
        let command_parts = value.trim().split(' ').collect::<Vec<&str>>();

        if command_parts.is_empty() {
            let error_message = "The given command String were empty. Try the help command for options.";
            Err(MyError::create_parse_command_error(error_message.into()))
        } else {
            match *command_parts.first().unwrap() {
                "State" => Ok(State),
                "ExpandPlaces" => Ok(ExpandPlaces),
                "ExpandElements" => Ok(ExpandElements),
                "ExpandMaxElement" => Ok(ExpandMaxElement),
                "ExpandMinElement" => Ok(ExpandMinElement),
                "ExpandEquipmentSlots" => Ok(ExpandEquipmentSlots),
                "ReduceDifficulty" => Ok(ReduceDifficulty),
                "ExpandMaxSimultaneousElement" => Ok(ExpandMaxSimultaneousElement),
                "ExpandMinSimultaneousElement" => Ok(ExpandMinSimultaneousElement),
                "Help" => Ok(Help),
                "ReorderInventory" => Ok(ReorderInventory),
                "Move" => Self::try_parse_move(&command_parts),
                "AddModifier" => Self::try_parse_add_modifier(&command_parts),
                "Equip" => Self::try_parse_equip(&command_parts),
                "SwapEquipment" => Self::try_parse_swap_equipment(&command_parts),
                "RerollModifier" => Self::try_parse_reroll_modifier(&command_parts),
                "SaveTheWorld" => Self::try_parse_save_the_world(&command_parts),
                "LoadTheWorld" => Self::try_parse_load_the_world(&command_parts),
                _ => {
                    let error_message = format!("Command not known. Got {:?}", command_parts);
                    Err(MyError::create_parse_command_error(error_message))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests_int {
    use crate::my_little_rpg_errors::MyError;
    use crate::parser::commands::Command;
    use crate::the_world::index_specifier::IndexSpecifier;

    #[test]
    fn try_from() {
        //TODO can parameter be simplified?
        assert_eq!(Command::State, Command::try_from(Into::<Box<str>>::into("State")).unwrap());
        assert_eq!(Command::ExpandPlaces, Command::try_from(Into::<Box<str>>::into("ExpandPlaces")).unwrap());
        assert_eq!(Command::ExpandElements, Command::try_from(Into::<Box<str>>::into("ExpandElements")).unwrap());
        assert_eq!(Command::ExpandMaxElement, Command::try_from(Into::<Box<str>>::into("ExpandMaxElement")).unwrap());
        assert_eq!(Command::ExpandMinElement, Command::try_from(Into::<Box<str>>::into("ExpandMinElement")).unwrap());
        assert_eq!(Command::ExpandEquipmentSlots, Command::try_from(Into::<Box<str>>::into("ExpandEquipmentSlots")).unwrap());
        assert_eq!(Command::ReduceDifficulty, Command::try_from(Into::<Box<str>>::into("ReduceDifficulty")).unwrap());
        assert_eq!(Command::ExpandMaxSimultaneousElement, Command::try_from(Into::<Box<str>>::into("ExpandMaxSimultaneousElement")).unwrap());
        assert_eq!(Command::ExpandMinSimultaneousElement, Command::try_from(Into::<Box<str>>::into("ExpandMinSimultaneousElement")).unwrap());
        assert_eq!(Command::Help, Command::try_from(Into::<Box<str>>::into("Help")).unwrap());
        assert_eq!(Command::ReorderInventory, Command::try_from(Into::<Box<str>>::into("ReorderInventory")).unwrap());

        assert_eq!(Command::Move(22), Command::try_from(Into::<Box<str>>::into("Move 22")).unwrap());
        assert_eq!(Err(MyError::create_parse_command_error("Trouble parsing move command, it needs the index of the place. Got [\"Move\"]".to_string())), Command::try_from(Into::<Box<str>>::into("Move")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter -1, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("Move -1")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter B, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("Move B")));

        assert_eq!(Command::AddModifier(22, vec![IndexSpecifier::Absolute(1), IndexSpecifier::Absolute(2), IndexSpecifier::Absolute(3)]), Command::try_from(Into::<Box<str>>::into("AddModifier 22 1,2,3")).unwrap());
        //TODO remove all the boxes and replace with pure into
        assert_eq!(Command::AddModifier(22, vec![IndexSpecifier::RelativePositive(1), IndexSpecifier::RelativeNegative(2), IndexSpecifier::Absolute(3)]), Command::try_from(Into::<Box<str>>::into("AddModifier 22 +1,-2,3")).unwrap());
        assert_eq!(Err(MyError::create_parse_command_error("Trouble parsing AddModifier command, it needs the index of the item and a list comma seperated list of items to sacrifice. Got [\"AddModifier\"]".to_string())), Command::try_from(Into::<Box<str>>::into("AddModifier")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter -1, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("AddModifier -1")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter -1, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("AddModifier -1  1,2,3")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter B, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("AddModifier B 1,2,3")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter b, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("AddModifier 1 b")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter b, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("AddModifier 1 +b")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter b, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("AddModifier 1 -b")));
        assert_eq!(Err(MyError::create_parse_command_error("1-22 created an underflow!".to_string())), Command::try_from(Into::<Box<str>>::into("AddModifier 1 -22")));

        assert_eq!(Command::Equip(21, 22), Command::try_from(Into::<Box<str>>::into("Equip 21 22")).unwrap());
        assert_eq!(Err(MyError::create_parse_command_error("Trouble parsing Equip command, it needs index of inventory and index of equipment slot. Got [\"Equip\"]".to_string())), Command::try_from(Into::<Box<str>>::into("Equip")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter -1, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("Equip -1 22")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter -1, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("Equip 21 -1")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter B, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("Equip B 22")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter B, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("Equip 21 B")));

        assert_eq!(Command::SwapEquipment(21, 22), Command::try_from(Into::<Box<str>>::into("SwapEquipment 21 22")).unwrap());
        assert_eq!(Err(MyError::create_parse_command_error("Trouble parsing SwapEquipment command, it needs index of inventory and index of equipment slot. Got [\"SwapEquipment\"]".to_string())), Command::try_from(Into::<Box<str>>::into("SwapEquipment")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter -1, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("SwapEquipment -1 22")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter -1, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("SwapEquipment 21 -1")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter B, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("SwapEquipment B 22")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter B, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("SwapEquipment 21 B")));

        assert_eq!(Command::RerollModifier(21, 22, vec![IndexSpecifier::Absolute(1), IndexSpecifier::Absolute(2), IndexSpecifier::Absolute(3)]), Command::try_from(Into::<Box<str>>::into("RerollModifier 21 22 1,2,3")).unwrap());
        assert_eq!(Command::RerollModifier(21, 22, vec![IndexSpecifier::RelativePositive(1), IndexSpecifier::RelativeNegative(2), IndexSpecifier::Absolute(3)]), Command::try_from(Into::<Box<str>>::into("RerollModifier 21 22 +1,-2,3")).unwrap());
        assert_eq!(Err(MyError::create_parse_command_error("Trouble parsing RerollModifier command, it needs index of inventory, index of modifier and a list comma seperated list of items to sacrifice. Got [\"RerollModifier\"]".to_string())), Command::try_from(Into::<Box<str>>::into("RerollModifier")));
        assert_eq!(Err(MyError::create_parse_command_error("Trouble parsing RerollModifier command, it needs index of inventory, index of modifier and a list comma seperated list of items to sacrifice. Got [\"RerollModifier\", \"1\"]".to_string())), Command::try_from(Into::<Box<str>>::into("RerollModifier 1")));
        assert_eq!(Err(MyError::create_parse_command_error("Trouble parsing RerollModifier command, it needs index of inventory, index of modifier and a list comma seperated list of items to sacrifice. Got [\"RerollModifier\", \"1\", \"1\"]".to_string())), Command::try_from(Into::<Box<str>>::into("RerollModifier 1 1")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter -1, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("RerollModifier -1 22 1,2,3")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter -1, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("RerollModifier 21 -1 1,2,3")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter B, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("RerollModifier B 22 1,2,3")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter B, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("RerollModifier 21 B 1,2,3")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter a, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("RerollModifier 21 22 a")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter a, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("RerollModifier 21 22 -a")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter a, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("RerollModifier 21 22 +a")));
        assert_eq!(Err(MyError::create_parse_command_error("21-23 created an underflow!".to_string())), Command::try_from(Into::<Box<str>>::into("RerollModifier 21 22 -23")));

        assert_eq!(Command::SaveTheWorld("a".into(), Some("b".into())), Command::try_from(Into::<Box<str>>::into("SaveTheWorld a b")).unwrap());
        assert_eq!(Command::SaveTheWorld("a".into(), None), Command::try_from(Into::<Box<str>>::into("SaveTheWorld a")).unwrap());
        assert_eq!(Err(MyError::create_parse_command_error("Trouble parsing SaveTheWorld command, it needs a save game name and optionally a path to the savegame (remember to end the path with /). Default location is ./save_games/. Got [\"SaveTheWorld\"]".to_string())), Command::try_from(Into::<Box<str>>::into("SaveTheWorld")));

        assert_eq!(Command::LoadTheWorld("a".into(), Some("b".into())), Command::try_from(Into::<Box<str>>::into("LoadTheWorld a b")).unwrap());
        assert_eq!(Command::LoadTheWorld("a".into(), None), Command::try_from(Into::<Box<str>>::into("LoadTheWorld a")).unwrap());
        assert_eq!(Err(MyError::create_parse_command_error("Trouble parsing LoadTheWorld command, it needs a save game name and optionally a path to the savegame (remember to end the path with /). Default location is ./save_games/. Got [\"LoadTheWorld\"]".to_string())), Command::try_from(Into::<Box<str>>::into("LoadTheWorld")));

        assert_eq!(Err(MyError::create_parse_command_error("Command not known. Got [\"InvalidCommand\"]".to_string())), Command::try_from(Into::<Box<str>>::into("InvalidCommand")));
    }
}
