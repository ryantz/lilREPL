use std::io::{self, Write};

#[derive(Debug)]
enum UserType {
    ADMIN,
    USER,
    NOTSELECTED,
}

#[derive(PartialEq)]
enum NavChoice {
    BUILDER,
    VIEW,
    NOTSELECTED,
}

#[derive(Debug)]
struct Profile {
    name: String,
    age: u8,
    user_type: UserType,
}

impl Profile {
    fn show(&self) {
        println!("{self:#?}");
    }

    fn build(name: String, age: u8, user_type_selection: u8) -> Self {
        let user_type_selected = match user_type_selection {
            1 => UserType::ADMIN,
            2 => UserType::USER,
            _ => UserType::NOTSELECTED,
        };

        Self {
            name,
            age,
            user_type: user_type_selected,
        }
    }
}

fn main() {
    println!("Welcome user");
    println!("==============================================================");
    println!("P R O F I L E - B U I L D E R");
    println!("==============================================================");

    let greeting_choice: NavChoice = navigate(string_to_int(display_and_read("Navigation:\n1. Build profile,\n2. View profile\n")));

    if greeting_choice == NavChoice::BUILDER {
        let name = display_and_read("Please enter your name: ");
        let age = string_to_int(display_and_read("Please enter your age: "));
        let selection = string_to_int(display_and_read("Please enter your user type: "));

        let user_profile = Profile::build(name, age, selection);


        user_profile.show();
    } else {
        println!("Not developed yet");
    }
    println!("==============================================================");
}

// NAV: helper functions
// read input thats entered into function
fn display_and_read(display_message: &str) -> String {
    print!("{}", display_message);
    io::stdout().flush().expect("no input");
    
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("no input");

    user_input.trim().to_string()
}

fn string_to_int(input_string: String) -> u8 {
    let convert: u8 = input_string.parse().expect("nothing");

    convert
}

fn navigate(choice: u8) -> NavChoice {
    let chosen_route = match choice {
        1 => NavChoice::BUILDER,
        2 => NavChoice::VIEW,
        _ => NavChoice::NOTSELECTED,
    };

    chosen_route
}


// TODO:: need to use a vec. 
fn save_profile() {
    todo!()
}
