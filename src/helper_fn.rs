pub mod helper_fn {
    use std::io::{self, Write};

    pub fn display_read(string_to_display: &str) -> String {
        print!("{}", string_to_display);
        io::stdout().flush().expect("No input");

        print!(">>");
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
