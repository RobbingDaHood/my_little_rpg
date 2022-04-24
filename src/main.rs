
use crate::game::Game;
use crate::tcp_listener::Listener;

mod game;
mod attack_types;
mod treasure_types;
mod place;
mod tcp_listener;
mod commands;
mod place_generator;
mod game_generator;
mod modifier_cost;
mod modifier_gain;
mod item_modifier;
mod item;
mod command_move;
mod item_resource;
mod command_equip_swap;
mod command_create_new_item;
mod command_craft_reroll_modifier;
mod command_expand_places;
mod command_expand_elements;
mod command_expand_max_element;
mod command_expand_min_element;
mod command_expand_equipment_slots;
mod roll_modifier;
mod command_expand_modifier;
mod command_help;
mod command_expand_max_simultaneous_element;
mod command_expand_min_simultanius_element;
mod command_state;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Settings {
    #[structopt(default_value = "1337")]
    pub(crate) port: u16
}

fn main() {
    let opt = Settings::from_args();
    Listener::new(opt.port).listen()
}

