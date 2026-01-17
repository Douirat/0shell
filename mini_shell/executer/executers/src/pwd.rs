use types::command::*;

pub fn pwd(command: &Command){
 println!("--> {:?} --> {:?} --> {:?}", command.name, command.flags, command.args);
}