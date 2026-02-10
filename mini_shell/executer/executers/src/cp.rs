
    /*
    The cp command is used to copy files and directories from one location to another
    It's like making a duplicate of your file or folder.
    */


use types::command::*;
use std::fs;
use std::path::Path;

pub fn cp(command: &Command){
    if command.args.len() < 2 {
        eprintln!("cp: missing file operand");
        eprintln!("Usage: cp <source> <destination>");
        return;
    }

    let source = &command.args[0];
    let destination = &command.args[1];

    // Résoudre les chemins relatifs
    let cwd = command.state.cwd.borrow();
    let source_path = if Path::new(source).is_absolute() {
        Path::new(source).to_path_buf()
    } else {
        cwd.join(source)
    };

    let dest_path = if Path::new(destination).is_absolute() {
        Path::new(destination).to_path_buf()
    } else {
        cwd.join(destination)
    };

    // Si la destination est un dossier existant
    let final_dest = if dest_path.is_dir() {
        let filename = source_path.file_name().unwrap_or_default();
        dest_path.join(filename)
    } else {
        dest_path
    };

    // Copier le fichier source vers la destination
    match fs::copy(&source_path, &final_dest) {
        Ok(_bytes) => {

        }
        Err(e) => {
            eprintln!("cp: cannot copy '{}' to '{}': {}", source, destination, e);
        }
    }
}