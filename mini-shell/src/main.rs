mod shell;

 use crate::shell::parser::Commands;
use std::io::{self, Write};
use crate::shell::executer::execute_command;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let commands = Commands::parse_commands(&input);
        execute_command(&commands);
    }
}

