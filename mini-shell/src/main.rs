mod shell;
use std::io::{self, Write};
use shell::parser::parse_command;

fn main() {
    loop {
        print!("mini-shell> ");
        io::stdout().flush().unwrap(); // <-- flush the output

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let command = input.trim();

        let parsed_command = parse_command(command);
        println!("Parsed command: {:?}", parsed_command);
    }
}
