use crate::{
    command::commands::Command::{
        AddModifier, Equip, ExpandElements, ExpandEquipmentSlots, ExpandMaxElement,
        ExpandMaxSimultaneousElement, ExpandMinElement, ExpandMinSimultaneousElement, ExpandPlaces,
        Help, LoadTheWorld, Move, ReduceDifficulty, ReorderInventory, RerollModifier, SaveTheWorld,
        State, SwapEquipment,
    },
    my_little_rpg_errors::MyError,
};
pub use crate::command::commands::Command;
use crate::parser::basetype_parser::{try_parse_possible_relative_indexes, try_parse_usize};

mod tests;

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

    fn try_parse_move(command_parts: &Vec<&str>) -> Result<Command, MyError> {
        if command_parts.len() < 2 {
            let error_message = format!(
                "Trouble parsing move command, it needs the index of the place. Got \
                 {command_parts:?}"
            );
            Err(MyError::create_parse_command_error(error_message))
        } else {
            try_parse_usize(command_parts[1]).map(Move)
        }
    }

    fn try_parse_add_modifier(command_parts: &Vec<&str>) -> Result<Command, MyError> {
        if command_parts.len() < 2 {
            let error_message = format!(
                "Trouble parsing AddModifier command, it needs the index of the item and a list \
                 comma seperated list of items to sacrifice. Got {command_parts:?}"
            );
            Err(MyError::create_parse_command_error(error_message))
        } else {
            let inventory_position = try_parse_usize(command_parts[1])?;
            try_parse_possible_relative_indexes(command_parts[2], inventory_position).map(
                |parsed_sacrifice_item_indexes| {
                    AddModifier(inventory_position, parsed_sacrifice_item_indexes)
                },
            )
        }
    }

    fn try_parse_equip(command_parts: &Vec<&str>) -> Result<Command, MyError> {
        if command_parts.len() < 3 {
            let error_message = format!(
                "Trouble parsing Equip command, it needs index of inventory and index of \
                 equipment slot. Got {command_parts:?}"
            );
            return Err(MyError::create_parse_command_error(error_message));
        }

        let inventory_position = try_parse_usize(command_parts[1])?;
        let equipped_item_position = try_parse_usize(command_parts[2])?;
        Ok(Equip(inventory_position, equipped_item_position))
    }

    fn try_parse_swap_equipment(command_parts: &Vec<&str>) -> Result<Command, MyError> {
        if command_parts.len() < 3 {
            let error_message = format!(
                "Trouble parsing SwapEquipment command, it needs index of inventory and index of \
                 equipment slot. Got {command_parts:?}",
            );
            return Err(MyError::create_parse_command_error(error_message));
        }

        let equipped_item_position_1 = try_parse_usize(command_parts[1])?;
        let equipped_item_position_2 = try_parse_usize(command_parts[2])?;
        Ok(SwapEquipment(
            equipped_item_position_1,
            equipped_item_position_2,
        ))
    }

    fn try_parse_reroll_modifier(command_parts: &Vec<&str>) -> Result<Command, MyError> {
        if command_parts.len() < 4 {
            let error_message = format!(
                "Trouble parsing RerollModifier command, it needs index of inventory, index of \
                 modifier and a list comma seperated list of items to sacrifice. Got {command_parts:?}"
            );
            return Err(MyError::create_parse_command_error(error_message));
        }

        let inventory_index = try_parse_usize(command_parts[1])?;
        let modifier_index = try_parse_usize(command_parts[2])?;
        let parsed_sacrifice_item_indexes = try_parse_possible_relative_indexes(command_parts[3], inventory_index)?;
        Ok(RerollModifier(
            inventory_index,
            modifier_index,
            parsed_sacrifice_item_indexes,
        ))
    }

    fn try_parse_save_the_world(command_parts: &Vec<&str>) -> Result<Command, MyError> {
        if command_parts.len() < 2 {
            let error_message = format!(
                "Trouble parsing SaveTheWorld command, it needs a save game name and optionally a \
                 path to the savegame (remember to end the path with /). Default location is \
                 ./save_games/. Got {command_parts:?}"
            );
            return Err(MyError::create_parse_command_error(error_message));
        }

        let save_game_name = Box::from(command_parts[1]);
        let save_game_path = if command_parts.len() < 3 {
            None
        } else {
            Some(Box::from(command_parts[2]))
        };
        Ok(SaveTheWorld(save_game_name, save_game_path))
    }

    fn try_parse_load_the_world(command_parts: &Vec<&str>) -> Result<Command, MyError> {
        if command_parts.len() < 2 {
            let error_message = format!(
                "Trouble parsing LoadTheWorld command, it needs a save game name and optionally a \
                 path to the savegame (remember to end the path with /). Default location is \
                 ./save_games/. Got {command_parts:?}"
            );
            return Err(MyError::create_parse_command_error(error_message));
        }

        let save_game_name = Box::from(command_parts[1]);
        let save_game_path = if command_parts.len() < 3 {
            None
        } else {
            Some(Box::from(command_parts[2]))
        };
        Ok(LoadTheWorld(save_game_name, save_game_path))
    }
}

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
                    let error_message = format!("Command not known. Got {command_parts:?}");
                    Err(MyError::create_parse_command_error(error_message))
                }
            }
        }
    }
}
