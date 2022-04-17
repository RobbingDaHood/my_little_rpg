use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use serde_json::json;
use crate::Game;
use std::io::Read;
use std::io::Write;
use crate::commands::Command;
use crate::game_generator::generate_new_game;

pub struct Listener {
    tcp_listener: TcpListener,
}

impl Listener {
    pub fn new() -> Self {
        Self { tcp_listener: TcpListener::bind("0.0.0.0:1337").unwrap() }
    }

    pub fn listen(&self) {
        let mut game = generate_new_game();
        println!("Game is ready and listening on: 0.0.0.0:1337");

        for stream in self.tcp_listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    game = self.handle_request(&mut stream, game);
                }
                Err(e) => {
                    println!("Error: {}", e);
                    /* connection failed */
                }
            }
        }
    }

    fn handle_request(&self, stream: &mut TcpStream, game: Game) -> Game {
        let command_as_string = Self::read_command(stream);

        match Command::try_from(&command_as_string) {
            Err(e) => if let Err(e) = stream.write(format!("{:?}", e).as_bytes()) {
                panic!("{}", e);
            },
            Ok(Command::State) => if let Err(e) = stream.write(format!("{} \n", json!(game)).as_bytes()) {
                panic!("{}", e);
            }
        }

        game
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

