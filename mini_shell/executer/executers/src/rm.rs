use types::command::*;
use std::io::ErrorKind;
// if dir:
//     if -r → remove_dir_all
//     else → error
// else:
//     remove_file
pub fn rm(command: &Command) {
    if command.args.is_empty() {
        eprintln!("rm: missing operand");
        return;
    }

    let recursive = command.flags.contains(&Flag::R);

    for arg in &command.args {
        let path = command.state.cwd.borrow().join(arg);

        match path.symlink_metadata() {
            Ok(meta) => {
                if meta.is_dir() {
                    if recursive {
                        if let Err(e) = std::fs::remove_dir_all(&path) {
                            eprintln!("rm: {}: {}", arg, e);
                        }
                    } else {
                        eprintln!("rm: {}: Is a directory", arg);
                    }
                } else {
                    if let Err(e) = std::fs::remove_file(&path) {
                        eprintln!("rm: {}: {}", arg, e);
                    }
                }
            }
            Err(e) => {
                
            match e.kind() {
                ErrorKind::NotFound => {
                    eprintln!("rm: {}: No such file or directory", arg);
            }
                ErrorKind::PermissionDenied => {
                    eprintln!("rm: {}: Permission denied", arg);
            }
                _ => {
                    eprintln!("rm: {}: {}", arg, e);
            }
        }
    }
}
}
}
