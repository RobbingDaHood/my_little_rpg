#[cfg(test)]
mod tests_int {
    use crate::my_little_rpg_errors::MyError;
    use crate::parser::commands::Command;
    use crate::the_world::index_specifier::IndexSpecifier;

    #[test]
    fn try_from() {
        //TODO can parameter be simplified?
        assert_eq!(
            Command::State,
            Command::try_from(Into::<Box<str>>::into("State")).unwrap()
        );
        assert_eq!(
            Command::ExpandPlaces,
            Command::try_from(Into::<Box<str>>::into("ExpandPlaces")).unwrap()
        );
        assert_eq!(
            Command::ExpandElements,
            Command::try_from(Into::<Box<str>>::into("ExpandElements")).unwrap()
        );
        assert_eq!(
            Command::ExpandMaxElement,
            Command::try_from(Into::<Box<str>>::into("ExpandMaxElement")).unwrap()
        );
        assert_eq!(
            Command::ExpandMinElement,
            Command::try_from(Into::<Box<str>>::into("ExpandMinElement")).unwrap()
        );
        assert_eq!(
            Command::ExpandEquipmentSlots,
            Command::try_from(Into::<Box<str>>::into("ExpandEquipmentSlots")).unwrap()
        );
        assert_eq!(
            Command::ReduceDifficulty,
            Command::try_from(Into::<Box<str>>::into("ReduceDifficulty")).unwrap()
        );
        assert_eq!(
            Command::ExpandMaxSimultaneousElement,
            Command::try_from(Into::<Box<str>>::into("ExpandMaxSimultaneousElement")).unwrap()
        );
        assert_eq!(
            Command::ExpandMinSimultaneousElement,
            Command::try_from(Into::<Box<str>>::into("ExpandMinSimultaneousElement")).unwrap()
        );
        assert_eq!(
            Command::Help,
            Command::try_from(Into::<Box<str>>::into("Help")).unwrap()
        );
        assert_eq!(
            Command::ReorderInventory,
            Command::try_from(Into::<Box<str>>::into("ReorderInventory")).unwrap()
        );

        assert_eq!(
            Command::Move(22),
            Command::try_from(Into::<Box<str>>::into("Move 22")).unwrap()
        );
        assert_eq!(
            Err(MyError::create_parse_command_error(
                "Trouble parsing move command, it needs the index of the place. Got [\"Move\"]"
                    .to_string()
            )),
            Command::try_from(Into::<Box<str>>::into("Move"))
        );
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter -1, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("Move -1")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter B, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("Move B")));

        assert_eq!(
            Command::AddModifier(
                22,
                vec![
                    IndexSpecifier::Absolute(1),
                    IndexSpecifier::Absolute(2),
                    IndexSpecifier::Absolute(3)
                ]
            ),
            Command::try_from(Into::<Box<str>>::into("AddModifier 22 1,2,3")).unwrap()
        );
        //TODO remove all the boxes and replace with pure into
        assert_eq!(
            Command::AddModifier(
                22,
                vec![
                    IndexSpecifier::RelativePositive(1),
                    IndexSpecifier::RelativeNegative(2),
                    IndexSpecifier::Absolute(3)
                ]
            ),
            Command::try_from(Into::<Box<str>>::into("AddModifier 22 +1,-2,3")).unwrap()
        );
        assert_eq!(Err(MyError::create_parse_command_error("Trouble parsing AddModifier command, it needs the index of the item and a list comma seperated list of items to sacrifice. Got [\"AddModifier\"]".to_string())), Command::try_from(Into::<Box<str>>::into("AddModifier")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter -1, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("AddModifier -1")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter -1, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("AddModifier -1  1,2,3")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter B, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("AddModifier B 1,2,3")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter b, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("AddModifier 1 b")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter b, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("AddModifier 1 +b")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter b, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("AddModifier 1 -b")));
        assert_eq!(
            Err(MyError::create_parse_command_error(
                "1-22 created an underflow!".to_string()
            )),
            Command::try_from(Into::<Box<str>>::into("AddModifier 1 -22"))
        );

        assert_eq!(
            Command::Equip(21, 22),
            Command::try_from(Into::<Box<str>>::into("Equip 21 22")).unwrap()
        );
        assert_eq!(Err(MyError::create_parse_command_error("Trouble parsing Equip command, it needs index of inventory and index of equipment slot. Got [\"Equip\"]".to_string())), Command::try_from(Into::<Box<str>>::into("Equip")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter -1, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("Equip -1 22")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter -1, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("Equip 21 -1")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter B, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("Equip B 22")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter B, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("Equip 21 B")));

        assert_eq!(
            Command::SwapEquipment(21, 22),
            Command::try_from(Into::<Box<str>>::into("SwapEquipment 21 22")).unwrap()
        );
        assert_eq!(Err(MyError::create_parse_command_error("Trouble parsing SwapEquipment command, it needs index of inventory and index of equipment slot. Got [\"SwapEquipment\"]".to_string())), Command::try_from(Into::<Box<str>>::into("SwapEquipment")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter -1, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("SwapEquipment -1 22")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter -1, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("SwapEquipment 21 -1")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter B, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("SwapEquipment B 22")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter B, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("SwapEquipment 21 B")));

        assert_eq!(
            Command::RerollModifier(
                21,
                22,
                vec![
                    IndexSpecifier::Absolute(1),
                    IndexSpecifier::Absolute(2),
                    IndexSpecifier::Absolute(3)
                ]
            ),
            Command::try_from(Into::<Box<str>>::into("RerollModifier 21 22 1,2,3")).unwrap()
        );
        assert_eq!(
            Command::RerollModifier(
                21,
                22,
                vec![
                    IndexSpecifier::RelativePositive(1),
                    IndexSpecifier::RelativeNegative(2),
                    IndexSpecifier::Absolute(3)
                ]
            ),
            Command::try_from(Into::<Box<str>>::into("RerollModifier 21 22 +1,-2,3")).unwrap()
        );
        assert_eq!(Err(MyError::create_parse_command_error("Trouble parsing RerollModifier command, it needs index of inventory, index of modifier and a list comma seperated list of items to sacrifice. Got [\"RerollModifier\"]".to_string())), Command::try_from(Into::<Box<str>>::into("RerollModifier")));
        assert_eq!(Err(MyError::create_parse_command_error("Trouble parsing RerollModifier command, it needs index of inventory, index of modifier and a list comma seperated list of items to sacrifice. Got [\"RerollModifier\", \"1\"]".to_string())), Command::try_from(Into::<Box<str>>::into("RerollModifier 1")));
        assert_eq!(Err(MyError::create_parse_command_error("Trouble parsing RerollModifier command, it needs index of inventory, index of modifier and a list comma seperated list of items to sacrifice. Got [\"RerollModifier\", \"1\", \"1\"]".to_string())), Command::try_from(Into::<Box<str>>::into("RerollModifier 1 1")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter -1, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("RerollModifier -1 22 1,2,3")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter -1, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("RerollModifier 21 -1 1,2,3")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter B, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("RerollModifier B 22 1,2,3")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter B, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("RerollModifier 21 B 1,2,3")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter a, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("RerollModifier 21 22 a")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter a, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("RerollModifier 21 22 -a")));
        assert_eq!(Err(MyError::create_parse_command_error("The following parameter a, got the following error while parsing: ParseIntError { kind: InvalidDigit }".to_string())), Command::try_from(Into::<Box<str>>::into("RerollModifier 21 22 +a")));
        assert_eq!(
            Err(MyError::create_parse_command_error(
                "21-23 created an underflow!".to_string()
            )),
            Command::try_from(Into::<Box<str>>::into("RerollModifier 21 22 -23"))
        );

        assert_eq!(
            Command::SaveTheWorld("a".into(), Some("b".into())),
            Command::try_from(Into::<Box<str>>::into("SaveTheWorld a b")).unwrap()
        );
        assert_eq!(
            Command::SaveTheWorld("a".into(), None),
            Command::try_from(Into::<Box<str>>::into("SaveTheWorld a")).unwrap()
        );
        assert_eq!(Err(MyError::create_parse_command_error("Trouble parsing SaveTheWorld command, it needs a save game name and optionally a path to the savegame (remember to end the path with /). Default location is ./save_games/. Got [\"SaveTheWorld\"]".to_string())), Command::try_from(Into::<Box<str>>::into("SaveTheWorld")));

        assert_eq!(
            Command::LoadTheWorld("a".into(), Some("b".into())),
            Command::try_from(Into::<Box<str>>::into("LoadTheWorld a b")).unwrap()
        );
        assert_eq!(
            Command::LoadTheWorld("a".into(), None),
            Command::try_from(Into::<Box<str>>::into("LoadTheWorld a")).unwrap()
        );
        assert_eq!(Err(MyError::create_parse_command_error("Trouble parsing LoadTheWorld command, it needs a save game name and optionally a path to the savegame (remember to end the path with /). Default location is ./save_games/. Got [\"LoadTheWorld\"]".to_string())), Command::try_from(Into::<Box<str>>::into("LoadTheWorld")));

        assert_eq!(
            Err(MyError::create_parse_command_error(
                "Command not known. Got [\"InvalidCommand\"]".to_string()
            )),
            Command::try_from(Into::<Box<str>>::into("InvalidCommand"))
        );
    }
}
