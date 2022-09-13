extern crate core;

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
mod command_craft_reroll_modifier;
mod command_expand_places;
mod command_expand_elements;
mod command_expand_max_element;
mod command_expand_min_element;
mod command_expand_equipment_slots;
mod roll_modifier;
mod command_craft_expand_modifier;
mod command_help;
mod command_expand_max_simultaneous_element;
mod command_expand_min_simultanius_element;
mod command_state;
mod hex_encoder;
mod command_reduce_difficulty;
mod command_reorder_inventory;
mod index_specifier;
mod difficulty;
mod game_statistics;
mod command_save_load;

use structopt::StructOpt;
use crate::hex_encoder::decode_hex;

#[derive(Debug, StructOpt)]
pub struct Settings {
    #[structopt(short, long, default_value = "1337", help = "A u16 representing the port used to listen for incoming commands.")]
    pub(crate) port: u16,

    #[structopt(short, long, parse(try_from_str = parse_seed), help = "16 hexidecimal representation of the seed. Example: e66832fd2e73fec455149e08b9c08bc1")]
    pub(crate) seed: Option<[u8; 16]>,
}

fn parse_seed(src: &str) -> Result<[u8; 16], String> {
    Ok(decode_hex(src)
        .expect(format!("Could not parse the given seed {} to hexidecimals", src).as_str())
        .try_into()
        .unwrap_or_else(|v: Vec<u8>| panic!("Seed need exactly 16 hexidecimals; It got {:?}", v)))
}

fn main() {
    let opt = Settings::from_args();
    Listener::new(opt.port).listen(opt.seed)
}

