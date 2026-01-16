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
                        // Afficher pour déboguer
                        println!("Command: {:?}", command.name);
                        println!("Flags: {:?}", command.flags);
                        println!("Args: {:?}", command.args);
                        
                        // Gestion de la commande exit
                        if matches!(command.name, CommandType::Exit) {
                            println!("Goodbye!");
                            break;
                        }
                        
                        // TODO: Appeler l'exécuteur ici
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
