use crate::commands::Command::Move;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Eq, Hash)]
pub enum Command {
    State,
    Move(usize)
}

impl TryFrom<&String> for Command {
    type Error = String;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let command_parts = value.split(" ").collect::<Vec<&str>>();

        if command_parts.len() == 0 {
            return Err("Command is empty".to_string());
        }

        return match command_parts[0] {
            "State" => Ok(Command::State),
            "Move" => {
                if command_parts.len() < 2 {
                    return Err(format!("Trouble parsing move command, it needs the index of the place. Got {:?}", command_parts));
                }

                return if let Ok(place_index) = command_parts[1].parse::<usize>() {
                    Ok(Move(place_index))
                } else {
                    Err(format!("Trouble parsing move command, it needs the index of the place. Got {:?}", command_parts))
                }
            }
            _ => Err(format!("Command not known. Got {:?}", command_parts))
        };
    }
}

#[cfg(test)]
mod tests_int {
    use crate::commands::Command;

    #[test]
    fn try_from() {
        assert_eq!(Command::State, Command::try_from(&"State".to_string()).unwrap());

        assert_eq!(Command::Move(22), Command::try_from(&"Move 22".to_string()).unwrap());
        assert_eq!(Err("Trouble parsing move command, it needs the index of the place. Got [\"Move\"]".to_string()), Command::try_from(&"Move".to_string()));
        assert_eq!(Err("Trouble parsing move command, it needs the index of the place. Got [\"Move\", \"-1\"]".to_string()), Command::try_from(&"Move -1".to_string()));
        assert_eq!(Err("Trouble parsing move command, it needs the index of the place. Got [\"Move\", \"B\"]".to_string()), Command::try_from(&"Move B".to_string()));

        assert_eq!(Err("Command not known. Got [\"InvalidCommand\"]".to_string()), Command::try_from(&"InvalidCommand".to_string()));
    }
}