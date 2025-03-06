use std::io::{self, Write};

fn main() {
    println!("Welcome user");
    println!("==================");

    loop {
        let mut input = String::from("input recieved -> ");
        repl_prompt(&mut input);
        println!("{}", input.trim());
    }

}

// pass in thru ref
fn repl_prompt(input_ref: &mut String) -> &str {
    print!("lilREPL >> ");

    io::stdout()
        .flush()
        .expect("Failed to flush output that is tagged to print! statement");
 
    io::stdin()
        .read_line(input_ref)
        .expect("failed to read input");

    input_ref
}
