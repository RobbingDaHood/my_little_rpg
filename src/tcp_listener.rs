use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use serde_json::json;
use crate::{Game};
use std::io::Read;
use std::io::Write;
use crate::command_craft_reroll_modifier::execute_craft_reroll_modifier;
use crate::command_equip_swap::{execute_equip_item, execute_swap_equipped_item};
use crate::command_expand_elements::execute_expand_elements;
use crate::command_expand_equipment_slots::execute_expand_equipment_slots;
use crate::command_expand_max_element::execute_expand_max_element;
use crate::command_expand_max_simultaneous_element::execute_expand_max_simultaneous_element;
use crate::command_expand_min_element::execute_expand_min_element;
use crate::command_expand_min_simultanius_element::execute_expand_min_simultaneous_element;
use crate::command_expand_modifier::execute_expand_modifiers;
use crate::command_expand_places::execute_expand_places;
use crate::command_help::execute_help;
use crate::command_move::execute_move_command;
use crate::command_state::execute_state;
use crate::commands::Command;
use crate::game_generator::{generate_new_game};

pub struct Listener {
    tcp_listener: TcpListener,
}

impl Listener {
    pub fn new(port: u16) -> Self {
        Self { tcp_listener: TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap() }
    }

    pub fn listen(&self, seed: Option<[u8; 16]>) {
        let mut game = generate_new_game(seed);
        println!("Game is ready and listening on: 0.0.0.0:{}", self.tcp_listener.local_addr().unwrap().port());

        for stream in self.tcp_listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    self.handle_request(&mut stream, &mut game);
                }
                Err(e) => {
                    println!("Error: {}", e);
                    /* connection failed */
                }
            }
        }
    }

    fn handle_request(&self, stream: &mut TcpStream, game: &mut Game) {
        let command_as_string = Self::read_command(stream);

        match Command::try_from(&command_as_string) {
            Err(e) => if let Err(e) = stream.write(format!("{:?}", e).as_bytes()) {
                panic!("{}", e);
            },
            Ok(Command::State) => if let Err(e) = stream.write(format!("{} \n", json!(execute_state(game))).as_bytes()) {
                panic!("{}", e);
            },
            Ok(Command::Move(place_index)) => if let Err(e) = stream.write(format!("{} \n", json!(execute_move_command(game, place_index))).as_bytes()) {
                panic!("{}", e);
            },
            Ok(Command::Equip(inventory_position, equipped_item_position)) => if let Err(e) = stream.write(format!("{} \n", json!(execute_equip_item(game, inventory_position, equipped_item_position))).as_bytes()) {
                panic!("{}", e);
            },
            Ok(Command::SwapEquipment(equipped_item_position_1, equipped_item_position_2)) => if let Err(e) = stream.write(format!("{} \n", json!(execute_swap_equipped_item(game, equipped_item_position_1, equipped_item_position_2))).as_bytes()) {
                panic!("{}", e);
            },
            Ok(Command::RerollModifier(inventory_index, modifier_index)) => if let Err(e) = stream.write(format!("{} \n", json!(execute_craft_reroll_modifier(game, inventory_index, modifier_index))).as_bytes()) {
                panic!("{}", e);
            },
            Ok(Command::ExpandPlaces) => if let Err(e) = stream.write(format!("{} \n", json!(execute_expand_places(game))).as_bytes()) {
                panic!("{}", e);
            },
            Ok(Command::ExpandElements) => if let Err(e) = stream.write(format!("{} \n", json!(execute_expand_elements(game))).as_bytes()) {
                panic!("{}", e);
            },
            Ok(Command::ExpandMaxElement) => if let Err(e) = stream.write(format!("{} \n", json!(execute_expand_max_element(game))).as_bytes()) {
                panic!("{}", e);
            },
            Ok(Command::ExpandMinElement) => if let Err(e) = stream.write(format!("{} \n", json!(execute_expand_min_element(game))).as_bytes()) {
                panic!("{}", e);
            },
            Ok(Command::ExpandMaxSimultaneousElement) => if let Err(e) = stream.write(format!("{} \n", json!(execute_expand_max_simultaneous_element(game))).as_bytes()) {
                panic!("{}", e);
            },
            Ok(Command::ExpandMinSimultaneousElement) => if let Err(e) = stream.write(format!("{} \n", json!(execute_expand_min_simultaneous_element(game))).as_bytes()) {
                panic!("{}", e);
            },
            Ok(Command::ExpandEquipmentSlots) => if let Err(e) = stream.write(format!("{} \n", json!(execute_expand_equipment_slots(game))).as_bytes()) {
                panic!("{}", e);
            },
            Ok(Command::AddModifier(place_index)) => if let Err(e) = stream.write(format!("{} \n", json!(execute_expand_modifiers(game, place_index))).as_bytes()) {
                panic!("{}", e);
            },
            Ok(Command::Help) => if let Err(e) = stream.write(format!("{} \n", json!(execute_help())).as_bytes()) {
                panic!("{}", e);
            },
        }
    }

    fn read_command(stream: &mut TcpStream) -> String {
        let mut buffer = [0; 1024];
        if let Err(e) = stream.set_read_timeout(Some(Duration::from_secs(1))) {
            panic!("Got error from setting timeout on reading tcp input, aborting: {}", e);
        }
        let buffer_size = match stream.read(&mut buffer) {
            Ok(buffer_size_value) => { buffer_size_value }
            Err(e) => {
                panic!("Got error from reading command, aborting: {}", e);
            }
        };
        let command = &buffer[..buffer_size];
        let command_as_string = String::from_utf8(command.to_vec()).unwrap();
        println!("Received request with following command: {}", command_as_string);
        command_as_string
    }
}

