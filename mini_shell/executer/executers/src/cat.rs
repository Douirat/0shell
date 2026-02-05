// use std::fs::File;
// use std::io::{self, Read, Write};
/*
Primarily used to read and display the contents of files on the terminal.
Can concatenate multiple files and display them as a single continuous output.
*/
use types::command::*;
use std::fs;
use std::path::Path;

pub fn cat(command: &Command) {
    if command.args.is_empty() {
        eprintln!("cat: missing file operand");
        return;
    }

    for file_arg in &command.args {
        let file_path = if Path::new(file_arg).is_absolute() {
            file_arg.clone()
        } else {
            // Résoudre le chemin relatif par rapport au cwd
            let cwd = command.state.cwd.borrow();
            cwd.join(file_arg).to_string_lossy().to_string()
        };

        match fs::read_to_string(&file_path) {
            Ok(content) => print!("{}", content),
            Err(e) => {
                eprintln!("cat: {}: {}", file_arg, e);
            }
        }
    }
}