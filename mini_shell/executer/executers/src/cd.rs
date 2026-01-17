/*
It allows users to move from the current working directory to another specified directory by providing either an absolute path or a relative path.
This command is essential for exploring different locations within the Linux environment and managing files efficiently.
*/
use types::command::*;
use std::env;

pub fn cd(command: &Command){
    let path = if command.args.is_empty() {
        match  env::var("HOME").or_else(|_| env::var("USERPROFILE")) {
            Ok(home) => home,
            Err(_) => {
                eprintln!("cd: HOME directory not found");
                return;
            }
            
        }
    } else {
        command.args[0].clone()
    };

    match env::set_current_dir(&path) {
        Ok(_) => {

        }
        Err(e) => {
            eprintln!("cd: {}: {}", path, e)
        }
    }
}