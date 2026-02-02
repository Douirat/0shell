use types::command::*;

pub fn pwd(command: &Command){
    let current_dir = command.state.cwd.borrow();
    println!("{}", current_dir.display());
}