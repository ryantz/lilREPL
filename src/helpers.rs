// establish a module
pub mod helper_fn {
    use super::colors;

    // bring the std io library into the module scope
    use std::io::{self, Write};

    // TODO:syntax highlighting for some words in the enum Keywords
    // how to get the first word in the user input? slices?
    // how to change the colours in real time?
    //pub fn read_input(string_to_display: &str) {
    //    println!("lilREPL>>");
    //    io::stdout().flush().expect("no input");
    //    todo!();
    //}

    pub fn display_then_read(string_to_display: &str) -> String {
        print!("{}", string_to_display);
        io::stdout().flush().expect("No input");

        print!("{}lilREPL>> {}", colors::BLUE, colors::RESET);
        io::stdout().flush().expect("No input");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("No input");

        input.trim().to_string()
    }

    pub fn string_to_int(input_string: String) -> u8 {
        let convert: u8 = input_string.parse().expect("No input");

        convert
    }

    pub fn string_to_usize(input_string: String) -> usize {
        let convert: usize = input_string.parse().expect("No input");

        convert
    }

    // lol found in the rust book woohoo just what i needed
    // TODO: get rest of string
    pub fn read_first_word(input_string: &String) -> &str {
        let input_as_bytes = input_string.as_bytes();

        for (i, &item) in input_as_bytes.iter().enumerate() {
            if item == b' ' {
                return &input_string[0..i];
            }
        }
        &input_string[..]
    }

    pub fn disp_after_keyword(input_string: &String) -> &str {
        let input_as_bytes = input_string.as_bytes();

        for (i, &item) in input_as_bytes.iter().enumerate() {
            if item == b' ' {
                return &input_string[i..];
            }
        }
        &input_string[..]
    }
}

// TODO: How do you read the inputs and read symbols?
// input => solve 1 + 1
// output => 2
// pub mod simple_maths {
//     pub fn add(){
//         todo!();
//     }
// }

pub mod structs_enums {
    use std::time::SystemTime;

    #[derive(Debug)]
    pub enum Feelings {
        Happy,
        Sad,
        Hopeful,
        Despair,
        Excited,
        Motivated,
        NoFeelings,
    }

    #[derive(Debug)]
    pub struct FeelingBoard {
        feeling: Feelings,
        timestamp: SystemTime,
    }

    impl FeelingBoard {
        pub fn show(&self) {
            println!("Feelings saved: {:?}\nDate:{:?}", self.feeling, self.timestamp);
        }

        pub fn build(selection: u8, timestamp: SystemTime) -> Self {
            let user_selected_feeling: Feelings = match selection {
                1 => Feelings::Happy,
                2 => Feelings::Sad,
                3 => Feelings::Hopeful,
                4 => Feelings::Despair,
                5 => Feelings::Excited,
                6 => Feelings::Motivated,
                _ => Feelings::NoFeelings ,
            };

            Self {
                feeling: user_selected_feeling,
                timestamp,
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum UserType {
        Admin,
        User,
        NotSelected,
    }

    #[derive(Debug, PartialEq)]
    pub struct Profile {
        pub name: String,
        pub age: u8,
        pub user_type: UserType,
    }

    impl Profile {
        pub fn show(&self) {
            println!(
                "Preview:\n\nname: {},\nage: {},\nuser type: {:?}\n",
                self.name, self.age, self.user_type
            );
        }

        pub fn build(name: String, age: u8, user_type_selection: u8) -> Self {
            let user_type_selected: UserType = match user_type_selection {
                1 => UserType::Admin,
                2 => UserType::User,
                _ => UserType::NotSelected,
            };

            Self {
                name,
                age,
                user_type: user_type_selected,
            }
        }
    }
}

pub mod ui_cmpts {
    use super::colors;

    pub fn insert_line() {
        println!("==============================================================");
    }

    pub fn profile_builder_greeting() {
        println!("==============================================================");
        println!(
            "{}P R O F I L E - B U I L D E R{}",
            colors::MAGENTA,
            colors::RESET
        );
        println!("==============================================================");
    }

    pub fn profile_viewer_greeting() {
        println!("==============================================================");
        println!(
            "{}P R O F I L E - V I E W E R{}",
            colors::YELLOW,
            colors::RESET
        );
        println!("==============================================================");
    }

    pub fn profile_finder_greeting() {
        println!("==============================================================");
        println!(
            "{}P R O F I L E - V I E W E R{}",
            colors::CYAN,
            colors::RESET
        );
        println!("==============================================================");
    }

    pub fn not_done_notice() -> bool {
        println!(
            "{}==============================================================",
            colors::RED
        );
        println!("XXXXXX------N O T . C R E A T E D------XXXXXX");
        println!(
            "=============================================================={}",
            colors::RESET
        );
        true
    }
}

pub mod colors {
    // colours in ANSI
    pub const RESET: &str = "\x1b[0m";
    pub const RED: &str = "\x1b[91m";
    pub const GREEN: &str = "\x1b[92m";
    pub const BLUE: &str = "\x1b[94m";
    pub const YELLOW: &str = "\x1b[93m";
    pub const MAGENTA: &str = "\x1b[95m";
    pub const CYAN: &str = "\x1b[96m";
}

pub mod keywords {
    pub const SHOUT: &str = "shout";
    pub const WHERE: &str = "where";
}
