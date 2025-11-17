mod shell;

 use crate::shell::parser::Commands;
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let commands = Commands::parse_commands(&input);
        for cmd in commands.commands {
            // Debug print
            println!("Command name: {}", cmd.name);
            println!("Args: {:?}", cmd.args);

            // Example: simple built-ins
            match cmd.name.as_str() {
                "exit" => {
                    println!("Bye!");
                    break;
                }
                "echo" => {
                    println!("{}", cmd.args.join(" "));
                }
                "" => continue, // empty input
                _ => println!("Unknown command: {}", cmd.name),
            }
        }
    }
}

