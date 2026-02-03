use types::command::*;

pub fn pwd(command: &Command){
 println!("{}", command.state.cwd.borrow().to_string_lossy());
}