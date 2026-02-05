use std::io::*;
use parser::parse;
use executer::execute;
use types::state::*;
use std::rc::Rc;

fn main() {
    let state = Rc::new(State::init_state());
    loop {
        print!("$ ");
        let _ = stdout().flush();

        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => {
                match parse(&state, &input) {
                    Ok(commands) => {
                        execute(&commands);
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



//////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////          IMPLÉMENTATION DE LA REDIRECTION ">" POUR ECHO            ////////////////////
////////////////// Fonctionnalité ajoutée pour supporter: echo "text" > fichier.txt //////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////
// use std::fs;
// use std::io::*;
// use parser::parse;
// use executer::execute;
// use types::state::*;
// use std::rc::Rc;
// use std::path::Path;

// fn resolve_path(path_str: &str, state: &State) -> std::path::PathBuf {
//     let path = Path::new(path_str);
    
//     if path.is_absolute() {
//         path.to_path_buf()
//     } else {
//         let cwd = state.cwd.borrow();
//         cwd.join(path_str)
//     }
// }

// fn main() {
//     let state = Rc::new(State::init_state());
//     loop {
//         print!("$ ");
//         let _ = stdout().flush();

//         let mut input = String::new();
//         match stdin().read_line(&mut input) {
//             Ok(_) => {
//                 let line = input.trim();
                
//                 // Vérifier si la ligne contient une redirection ">"
//                 if let Some(index) = line.find('>') {
//                     // Sépare la commande et le fichier
//                     let command_part = line[..index].trim();
//                     let file_part = line[index + 1..].trim();
                    
//                     // Vérifier qu'il y a bien un nom de fichier après ">"
//                     if file_part.is_empty() {
//                         eprintln!("Syntax error: missing filename after '>'");
//                         continue;
//                     }
                    
//                     // Analyser la commande (sans la redirection)
//                     match parse(&state, command_part) {
//                         Ok(mut commands) => {
//                             // commands est un Commands<'a> qui contient un vecteur de Command
//                             if !commands.command.is_empty() {
//                                 // Prendre la première commande (pour l'instant, on suppose une seule commande)
//                                 let cmd = &commands.command[0];
                                
//                                 // Gérer la redirection seulement pour echo pour commencer
//                                 match cmd.name {
//                                     types::command::CommandType::Echo => {
//                                         let output = cmd.args.join(" ");
//                                         let file_path = resolve_path(file_part, &state);
                                        
//                                         // Créer le fichier avec le contenu
//                                         match fs::write(&file_path, output) {
//                                             Ok(_) => {
//                                                 // Le fichier a été créé avec succès
//                                             }
//                                             Err(e) => {
//                                                 eprintln!("Error writing to file '{}': {}", file_part, e);
//                                             }
//                                         }
//                                     }
//                                     _ => {
//                                         // Pour les autres commandes, exécuter normalement
//                                         // (La redirection ne fonctionnera pas encore)
//                                         eprintln!("Warning: Redirection '>' only works with 'echo' for now");
//                                         execute(&commands);
//                                     }
//                                 }
//                             }
//                         }
//                         Err(e) => {
//                             eprintln!("{}", e);
//                         }
//                     }
//                 } else {
//                     // Pas de redirection, exécuter normalement
//                     match parse(&state, &input) {
//                         Ok(commands) => {
//                             execute(&commands);
//                         }
//                         Err(e) => {
//                             eprintln!("{}", e);
//                         }
//                     }
//                 }
//             }
//             Err(_) => eprintln!("Error reading input"),
//         }
//     }
// }