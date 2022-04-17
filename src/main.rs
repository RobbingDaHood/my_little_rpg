use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use crate::game::Game;
use std::io::Read;

mod game;
mod attack_types;
mod crafting_materials;
mod place;
mod command_state;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:1337").unwrap();


    println!("Game is ready and listening on: 0.0.0.0:1337");

    let mut game = Game::new(Vec::new());

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                game = handle_request(&mut stream, game)
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
}

fn handle_request(stream: &mut TcpStream, game: Game) -> Game {
    let mut buffer = [0; 1024];
    if let Err(e) = stream.set_read_timeout(Some(Duration::from_secs(1))) {
        println!("Got error from setting timeout on reading tcp input, aborting: {}", e);
        return game;
    }
    let buffer_size = match stream.read(&mut buffer) {
        Ok(buffer_size_value) => { buffer_size_value }
        Err(e) => {
            println!("Got error from reading command, aborting: {}", e);
            return game;
        }
    };
    let command = &buffer[..buffer_size];
    let command_as_string = String::from_utf8(command.to_vec()).unwrap();
    println!("Received request with following command: {}", command_as_string);



    game
}
