use types::command::*;
use std::fs;
use std::path::Path;

pub fn mv(command: &Command){
    if command.args.len() < 2 {
        eprintln!("mv: missing file operand");
        eprintln!("Usage: mv <source> <destination>");
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

    // si la destination est un dossier existant,
    let final_dest = if dest_path.is_dir() {
        let filename = source_path.file_name().unwrap_or_default();
        dest_path.join(filename)
    } else {
        dest_path
    };

    // Déplacer/renommer le fichier source vers la destination
    match fs::rename(&source_path, &dest_path) {
        Ok(_) => {

        }
        Err(e) => {
            eprintln!("mv: cannot move '{}' to '{}': {}", source, destination, e);
        }
    }
}