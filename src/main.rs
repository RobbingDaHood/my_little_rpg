
use crate::game::Game;
use crate::tcp_listener::Listener;

mod game;
mod attack_types;
mod crafting_materials;
mod place;
mod command_state;
mod tcp_listener;
mod commands;
mod place_generator;
mod game_generator;

fn main() {
    Listener::new().listen()
}

