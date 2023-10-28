use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    time::Duration,
};

use serde_json::json;

//TODO align all the execute method names
use crate::{
    command::{
        craft_expand_modifier::execute_craft_expand_modifiers_json,
        craft_reroll_modifier::execute_json as execute_craft_reroll_modifier,
        equip_swap::{execute_equip_item_json, execute_swap_equipped_item_json},
        expand_elements::execute_json as execute_expand_elements,
        expand_equipment_slots::execute_json as execute_expand_equipment_slots,
        expand_max_element::execute_json as execute_expand_max_element,
        expand_max_simultaneous_element::execute_json as execute_expand_max_simultaneous_element,
        expand_min_element::execute_json as execute_expand_min_element,
        expand_min_simultanius_element::execute_expand_min_simultaneous_element_json,
        expand_places::execute_json as execute_expand_places,
        help::execute_json as execute_help,
        presentation_game_state::execute_json as execute_presentation_game_state,
        r#move::execute_json as execute_move_command,
        reduce_difficulty::execute_json as execute_reduce_difficulty,
        reorder_inventory::execute_json as execute_reorder_inventory,
        save_load::{execute_load_command_json, execute_save_command_json},
    },
    generator::game::new,
    my_little_rpg_errors::MyError,
    parser::commands::Command,
    Game,
};

pub struct Listener {
    tcp_listener: TcpListener,
}

impl Listener {
    pub fn new(port: u16) -> Self {
        Self {
            tcp_listener: TcpListener::bind(format!("0.0.0.0:{port}")).unwrap(),
        }
    }

    pub fn listen(
        &self,
        seed: Option<[u8; 16]>,
    ) {
        let mut game = new(seed);
        println!(
            "Game is ready and listening on: 0.0.0.0:{}",
            self.tcp_listener.local_addr().unwrap().port()
        );

        for stream in self.tcp_listener.incoming() {
            match stream {
                Ok(mut stream) => Listener::handle_request(&mut stream, &mut game),
                Err(e) => {
                    println!("Failed handling the request, got the following error: {e}");
                }
            }
        }
    }

    fn handle_request(
        stream: &mut TcpStream,
        game: &mut Game,
    ) {
        let result = Self::read_command(stream)
            .map(Command::try_from)
            .and_then(|r| r)
            .map(|command| {
                match command {
                    Command::State => execute_presentation_game_state(game),
                    Command::ReduceDifficulty => execute_reduce_difficulty(game),
                    Command::Move(place_index) => execute_move_command(game, place_index),
                    Command::Equip(inventory_position, equipped_item_position) => {
                        execute_equip_item_json(game, inventory_position, equipped_item_position)
                    }
                    Command::SwapEquipment(equipped_item_position_1, equipped_item_position_2) => {
                        execute_swap_equipped_item_json(
                            game,
                            equipped_item_position_1,
                            equipped_item_position_2,
                        )
                    }
                    Command::RerollModifier(
                        inventory_index,
                        modifier_index,
                        sacrifice_item_indexes,
                    ) => {
                        execute_craft_reroll_modifier(
                            game,
                            inventory_index,
                            modifier_index,
                            sacrifice_item_indexes,
                        )
                    }
                    Command::ExpandPlaces => execute_expand_places(game),
                    Command::ExpandElements => execute_expand_elements(game),
                    Command::ExpandMaxElement => execute_expand_max_element(game),
                    Command::ExpandMinElement => execute_expand_min_element(game),
                    Command::ExpandMaxSimultaneousElement => {
                        execute_expand_max_simultaneous_element(game)
                    }
                    Command::ExpandMinSimultaneousElement => {
                        execute_expand_min_simultaneous_element_json(game)
                    }
                    Command::ExpandEquipmentSlots => execute_expand_equipment_slots(game),
                    Command::AddModifier(place_index, sacrifice_item_indexes) => {
                        execute_craft_expand_modifiers_json(
                            game,
                            place_index,
                            sacrifice_item_indexes,
                        )
                    }
                    Command::Help => execute_help(),
                    Command::ReorderInventory => execute_reorder_inventory(game),
                    Command::SaveTheWorld(save_game_name, save_game_path) => {
                        execute_save_command_json(game, &save_game_name, save_game_path)
                    }
                    Command::LoadTheWorld(save_game_name, save_game_path) => {
                        execute_load_command_json(game, &save_game_name, save_game_path)
                    }
                }
            });

        let result_message = match result {
            Ok(result) => format!("{} \n", json!(result)),
            Err(result) => format!("{} \n", json!(result)),
        };

        match stream.write(result_message.as_bytes()) {
            Ok(_) => {
                println!("Responded to request.");
            } //TODO give more details
            Err(error) => {
                panic!("Got the following error when writing the response to the user: {error}")
            }
        }
    }

    fn read_command(stream: &mut TcpStream) -> Result<Box<str>, MyError> {
        let mut buffer = [0; 1024];
        stream
            .set_read_timeout(Some(Duration::from_secs(1)))
            .map_err(|e| {
                MyError::create_network_error(format!(
                    "Got error from setting timeout on reading tcp input, aborting: {}",
                    e
                ))
            })?;
        let buffer_size = stream.read(&mut buffer).map_err(|e| {
            MyError::create_network_error(format!(
                "Got error from reading command, aborting: {}",
                e
            ))
        })?;
        let command = &buffer[..buffer_size];
        let command_as_string = String::from_utf8(command.to_vec()).map_err(|e| {
            MyError::create_parse_command_error(format!(
                "Failed parsing the command, got error: {}",
                e
            ))
        })?;
        println!("Received request with following command: {command_as_string}");
        Ok(command_as_string.into())
    }
}
