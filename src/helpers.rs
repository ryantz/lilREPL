// establish a module
pub mod helper_fn {
    // bring the std io library into the module scope
    use std::io::{self, Write};

    pub fn display_then_read(string_to_display: &str) -> String {
        print!("{}", string_to_display);
        io::stdout().flush().expect("No input");

        print!("lilREPL>> ");
        io::stdout().flush().expect("No input");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("No input");

        input.trim().to_string()
    }

    pub fn string_to_int(input_string: String) -> u8 {
        let convert: u8 = input_string.parse().expect("No input");

        convert
    }
}

pub mod ui_cmpts {
    pub fn insert_line() {
        println!("==============================================================");
    }

    pub fn profile_builder_greeting() {
        println!("Welcome user");
        println!("==============================================================");
        println!("P R O F I L E - B U I L D E R");
        println!("==============================================================");
    }

    pub fn not_done_notice() {
        println!("Welcome user");
        println!("==============================================================");
        println!("XXXXXX------N O T . C R E A T E D------XXXXXX");
        println!("==============================================================");
    }
}
