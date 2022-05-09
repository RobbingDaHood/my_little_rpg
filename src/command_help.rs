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
        Command::RerollModifier(_, _, _) => "RerollModifier X Y Z: Reroll the modifier at index Y oo item in inventory at index X. Z is a comma seperated list of indexes for items in the inventory to sacrifice to pay the cost of the command, each item need to have at least the same amount of modifiers as the index of the modifier being rerolled. Z can contain relative indexes prefixed with + or -, they are relative to X.",
        Command::ExpandPlaces => "ExpandPlaces: Expand the number of places you can move too.",
        Command::ExpandElements => "ExpandElements: Expand the amount of possible elements. This both affect newly rolled places and newly rolled modifiers.",
        Command::ExpandMaxElement => "ExpandMaxElement: Expand the maximum possible roll of a random element. This both affect newly rolled places and newly rolled modifiers.",
        Command::ExpandMinElement => "ExpandMinElement: Expand the minimum possible roll of a random element. This both affect newly rolled places and newly rolled modifiers. Minimum cannot go above maximum.",
        Command::ExpandMaxSimultaneousElement => "ExpandMaxSimultaneousElement: Expand the maximum possible simultaneous roll elements. This both affect newly rolled places and newly rolled modifiers.",
        Command::ExpandMinSimultaneousElement => "ExpandMinSimultaneousElement: Expand the minimum possible simultaneous roll elements. This both affect newly rolled places and newly rolled modifiers. Minimum cannot go above maximum.",
        Command::ExpandEquipmentSlots => "ExpandEquipmentSlots: Expand the amount of possible equipment slots. It will equip the last item in your inventory automatically. If there are no items in the inventory it will craft a new shiny item to be equipped.",
        Command::ReduceDifficulty => "ReduceDifficulty: reduce a random attack types max value in game difficulty, if that goes lower than min value then min value is reduced. If the max value gets too low then the element will be removed. Simultaneous elements could also be affected.",
        Command::AddModifier(_, _) => "AddModifier X Y: Give item in inventory at index X a new random modifier. Y is a comma seperated list of indexes for items in the inventory to sacrifice to pay the cost of the command, they each need to have at least the same amount of modifiers as the item being upgraded. Z can contain relative indexes prefixed with + or -, they are relative to X.",
        Command::Help => "Help: Get the help text that you are reading right now.",
        Command::ReorderInventory => "ReorderInventory: Arranges all items in the inventory so there are no more gaps in indexes between items.",
        Command::SaveTheWorld(_, _) => "SaveTheWorld X Optional(Y): Save the world! So you can load it later; Stay saved. X is save game name, Y is optional save game path."
    }
}