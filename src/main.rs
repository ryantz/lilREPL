use std::io::{self, Write};

fn main() {
    println!("Welcome user");
    println!("==================");

    loop {
        let input = String::new();
        repl_prompt(input);
    }

}

fn repl_prompt(mut _input: String) {
    print!("lilREPL >> ");

    io::stdout()
        .flush()
        .expect("Failed to flush output that is tagged to print! statement");
 
    io::stdin()
        .read_line(&mut _input)
        .expect("failed to read input");

    println!("you entered: {}", _input.trim());
    println!("==================");
}
