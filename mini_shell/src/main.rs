use std::io::*;
use parser::parse;
use types::command::CommandType;

fn main() {
    loop {
        print!("$ ");
        let _ = stdout().flush();

        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => {
                match parse(&input) {
                    Ok(command) => {
                        println!("Command: {:?}", command.name);
                        println!("Flags: {:?}", command.flags);
                        println!("Args: {:?}", command.args);
                        
                        // Vérifier si c'est la commande exit
                        if matches!(command.name, CommandType::Exit) {
                            println!("Goodbye!");
                            break;
                        }
                    }
                    Err(e) => {
                        println!("Parse error: {}", e);
                    }
                }
            }
            Err(_) => println!("Error inserting the input"),
        };
    }
}