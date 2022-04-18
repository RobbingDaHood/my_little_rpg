
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
mod command_equip_unequip;
mod command_create_new_item;
mod command_craft_reroll_modifier;
mod command_expand_places;

fn main() {
    Listener::new().listen()
}

