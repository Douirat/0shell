// use types::command::*;

// use executers::{
//     cd::cd,
//     ls::ls,
//     echo::echo,
//     pwd::pwd,
//     cat::cat,
//     cp::cp,
//     rm::rm,
//     mv::mv,
//     mkdir::mkdir,
//     exit::exit,
// };

// pub fn execute<'a>(commands: &Commands) {

//     for command in &commands.command {
//         match &command.name {
//             CommandType::Cd => cd(command),
//             CommandType::Ls => ls(command), //(supporting -l, -a, -F)
//             CommandType::Echo => echo(command),
//             CommandType::Pwd =>pwd(command),
//             CommandType::Cat => cat(command),
//             CommandType::Cp => cp(command),
//             CommandType::Rm => rm(command), // (supporting -r)
//             CommandType::Mv =>mv(command),
//             CommandType::Mkdir =>mkdir(command),
//             CommandType::Exit => exit(command),
//         }
//     }
// }


use types::command::*;
use executers::*;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};

pub fn execute(commands: &Commands) {
    for command in &commands.command {
        // Sauvegarder stdout original
        let original_stdout = io::stdout();
        
        // Appliquer les redirections
        let mut output_file: Option<File> = None;
        
        for redir in &command.redirections {
            match redir {
                Redirection::Output(filename) => {
                    // Résoudre le chemin
                    let path = command.state.cwd.borrow().join(filename);
                    
                    match File::create(&path) {
                        Ok(file) => {
                            output_file = Some(file);
                        }
                        Err(e) => {
                            eprintln!("Cannot create '{}': {}", filename, e);
                            return;
                        }
                    }
                }
                Redirection::Append(filename) => {
                    let path = command.state.cwd.borrow().join(filename);
                    
                    match OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(&path)
                    {
                        Ok(file) => {
                            output_file = Some(file);
                        }
                        Err(e) => {
                            eprintln!("Cannot open '{}': {}", filename, e);
                            return;
                        }
                    }
                }
                Redirection::Input(_filename) => {
                    // Pour l'instant, on ne gère pas < (bonus du bonus)
                    eprintln!("Input redirection not yet implemented");
                }
            }
        }
        
        // Capturer la sortie si redirection
        if let Some(mut file) = output_file {
            // Capturer stdout dans un buffer
            use std::io::Cursor;
            
            // On va devoir modifier les commandes pour retourner String au lieu de println!
            // Pour l'instant, rediriger via une approche simplifiée
            
            match &command.name {
                CommandType::Echo => {
                    let output = command.args.join(" ");
                    writeln!(file, "{}", output).ok();
                }
                CommandType::Pwd => {
                    let cwd = command.state.cwd.borrow();
                    writeln!(file, "{}", cwd.display()).ok();
                }
                CommandType::Cat => {
                    // Gérer cat avec redirection
                    cat_with_output(command, &mut file);
                }
                CommandType::Ls => {
                    ls(command); // Complexe à rediriger pour l'instant
                }
                _ => {
                    // Pour les autres commandes, exécuter normalement
                    execute_command(command);
                }
            }
        } else {
            // Pas de redirection, exécution normale
            execute_command(command);
        }
    }
}

fn execute_command(command: &Command) {
    match &command.name {
        CommandType::Echo => echo(command),
        CommandType::Cd => cd(command),
        CommandType::Ls => ls(command),
        CommandType::Pwd => pwd(command),
        CommandType::Cat => cat(command),
        CommandType::Cp => cp(command),
        CommandType::Rm => rm(command),
        CommandType::Mv => mv(command),
        CommandType::Mkdir => mkdir(command),
        CommandType::Exit => exit(command),
    }
}

// Helper pour cat avec redirection
fn cat_with_output(command: &Command, file: &mut File) {
    use std::fs;
    use std::path::Path;
    
    if command.args.is_empty() {
        return;
    }

    for file_arg in &command.args {
        let file_path = if Path::new(file_arg).is_absolute() {
            file_arg.clone()
        } else {
            let cwd = command.state.cwd.borrow();
            cwd.join(file_arg).to_string_lossy().to_string()
        };

        match fs::read_to_string(&file_path) {
            Ok(content) => {
                write!(file, "{}", content).ok();
            }
            Err(_) => {
                // Ignorer les erreurs lors de la redirection
            }
        }
    }
}