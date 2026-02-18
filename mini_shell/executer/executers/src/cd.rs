use std::path::{Path, PathBuf};
use types::command::Command;

pub fn cd(command: &Command) {
    let state = command.state.clone();

    if !command.args.is_empty(){
        let arg = command.args[0].clone();

        if arg == "." {
            return
        }
        
        if   let Some(current_path) = state.cwd.borrow().to_str(){
           if current_path == "/home"{ 
            if &arg == ".."{
               return
           }

           println!("... {:?}", &command.args);
           
           if arg != state.user && arg != "./".to_owned() + &state.user{
               println!("cd: permission denied: {}", arg);
               return
            } 
        }}
    }



    // Determine the target path
    let target = if command.args.is_empty() || command.args.get(0).map(|s| s == "~").unwrap_or(false) {
        state.home.clone()  // clone the PathBuf from RefCell
    } else {
        let path_str = command.args.last().unwrap().clone();
        if Path::new(&path_str).is_absolute() {
            PathBuf::from(path_str)
        } else {
            state.cwd.borrow().join(path_str)
        }
    };

    // Canonicalize
    let normalized: PathBuf = match target.canonicalize() {
        Ok(path) => path,
        Err(_) => {
            eprintln!("cd: No such directory: {}", target.display());
            return;
        }
    };

    // Update cwd if it's a directory
    if normalized.is_dir() {
        *state.cwd.borrow_mut() = normalized; // interior mutability
    } else {
        eprintln!("cd: Not a directory: {}", normalized.display());
    }
}
