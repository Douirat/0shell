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
                    Ok(commands) => {
                        println!("Parsed {} command(s):", commands.command.len());
                        
                        for (i, command) in commands.command.iter().enumerate() {
                            println!("  Command #{}: {:?}", i + 1, command.name);
                            println!("    Flags: {:?}", command.flags);
                            println!("    Args: {:?}", command.args);
                            
                            // Vérifier si c'est exit
                            if matches!(command.name, CommandType::Exit) {
                                println!("Goodbye!");
                                return;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
            }
            Err(_) => eprintln!("Error reading input"),
        }
    }
}