extern crate core;

use structopt::StructOpt;

use the_world::game::Game;
use crate::parser::hex_encoder::decode_hex;
use crate::parser::tcp_listener::Listener;

mod the_world;
mod command;
mod parser;
mod generator;

#[derive(Debug, StructOpt)]
pub struct Settings {
    #[structopt(short, long, default_value = "1337", help = "A u16 representing the port used to listen for incoming commands.")]
    pub(crate) port: u16,

    #[structopt(short, long, parse(try_from_str = parse_seed), help = "32 hexidecimal representation of the seed. Example: e66832fd2e73fec455149e08b9c08bc1")]
    pub(crate) seed: Option<[u8; 16]>,
}

fn parse_seed(src: &str) -> Result<[u8; 16], String> {
    if src.len() != 32 {
        return Err("32 Hexidecimals as a string requires 32 chars!".to_string());
    }
    match decode_hex(src) {
        Err(e) => Err(e),
        Ok(r) => Ok(r.try_into().unwrap())
    }
}

fn main() {
    let opt = Settings::from_args();
    Listener::new(opt.port).listen(opt.seed);
}

