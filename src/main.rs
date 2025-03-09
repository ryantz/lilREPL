use std::io::{self, Write};

#[derive(Debug)]
enum UserType {
    ADMIN,
    USER,
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

    //loop {

    //        print!("lilREPL >> ");
    //        io::stdout().flush().expect("Failed to flush output that is tagged to print! statement");

    //        let mut input = String::from("");
    //        repl_reply(&mut input);

    //        println!("{}", input.trim());

    //        println!("==============================================================");
    //    }
    print!("Please enter your name: ");
    io::stdout().flush().expect("No input");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("no input");

    print!("Please enter your age: ");
    io::stdout().flush().expect("No input");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("no input");
    let age: u8 = age.trim().parse().expect("no input");

    print!("Please enter your user type selection: ");
    io::stdout().flush().expect("No input");
    let mut selection = String::new();
    io::stdin().read_line(&mut selection).expect("no input");
    let selection: u8 = selection.trim().parse().expect("no input");

    let user_profile = Profile::build(name, age, selection);
    user_profile.show();
}

// pass in thru ref
fn repl_reply(input_ref: &mut String) -> &str {
    io::stdin().read_line(input_ref).expect("failed to read input");

    input_ref
}

