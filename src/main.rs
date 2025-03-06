use std::io::{self, Write};

fn main() {
    println!("Welcome user");
    println!("==================");

    loop {
        let input = String::from("input recieved -> ");
        repl_prompt(input);
    }

}

fn repl_prompt(mut input: String) {
    print!("lilREPL >> ");

    io::stdout()
        .flush()
        .expect("Failed to flush output that is tagged to print! statement");
 
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");

    let input = input.trim();

    println!("{}", input);
    println!("==================");
}
