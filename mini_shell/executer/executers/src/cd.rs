use std::path::{Path, PathBuf};
use types::command::Command;

pub fn cd(command: &Command) {
    let state = command.state;

    // Determine the target path
    let target = if command.args.is_empty() || command.args.get(0).map(|s| s == "~").unwrap_or(false) {
        state.home.borrow().clone()  // clone the PathBuf from RefCell
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
