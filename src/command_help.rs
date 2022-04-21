use crate::commands::Command;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ExecuteHelpReport {
    tutorial: String,
    commands: Vec<String>,
}

pub fn execute_help() -> ExecuteHelpReport {
    let tutorial = "Use state, see what you like. Then get Moving, try Move 0. Do some more movement, earn some treasure. Use the treasure to craft items and expand difficulty. Have fun.".to_string();

    let mut commands = Vec::new();
    for command in Command::get_all() {
        commands.push(execute_help_for_command(command).to_string());
    }

    ExecuteHelpReport {
        tutorial,
        commands,
    }
}

fn execute_help_for_command(command: Command) -> &'static str {
    match command {
        Command::State => "State: Get complete state of the game.",
        Command::Move(_) => "Move X: Move to place with index X. The game will tell you if you win the place and get the reward, with a lot more info too. After the move the place would be rerolled. You can move to the same place as many times in a row you want.",
        Command::Equip(_, _) => "Equip X Y: Equip item from inventory at index X and exchange it with the item currently equipped at index Y.",
        Command::SwapEquipment(_, _) => "SwapEquipment X Y: Swap equipped item at index X with equipped item at index Y.",
        Command::CreateItem => "CreateItem: Create a new shiny item.",
        Command::RerollModifier(_, _) => "RerollModifier X Y: Reroll the modifier at index Y oo item in inventory at index X.",
        Command::ExpandPlaces => "ExpandPlaces: Expand the number of places you can move too.",
        Command::ExpandElements => "ExpandElements: Expand the amount of possible elements. This both affect newly rolled places and newly rolled modifiers.",
        Command::ExpandMaxElement => "ExpandMaxElement: Expand the maximum possible roll of a random element. This both affect newly rolled places and newly rolled modifiers.",
        Command::ExpandMinElement => "ExpandMinElement: Expand the minimum possible roll of a random element. This both affect newly rolled places and newly rolled modifiers. Minimum cannot go above maximum.",
        Command::ExpandMaxSimultaneousElement => "ExpandMaxSimultaneousElement: Expand the maximum possible simultaneous roll elements. This both affect newly rolled places and newly rolled modifiers.",
        Command::ExpandMinSimultaneousElement => "ExpandMinSimultaneousElement: Expand the minimum possible simultaneous roll elements. This both affect newly rolled places and newly rolled modifiers. Minimum cannot go above maximum.",
        Command::ExpandEquipmentSlots => "ExpandEquipmentSlots: Expand the amount of possible equipment slots. It will equip the last item in your inventory automatically. If there are no items in the inventory it will craft a new shiny item to be equipped.",
        Command::AddModifier(_) => "AddModifier X: Give item in inventory at index X a new random modifier.",
        Command::Help => "Help: Get the help text that you are reading right now."
    }
}