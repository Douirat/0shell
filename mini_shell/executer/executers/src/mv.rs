use types::command::*;

pub fn mv(command: &Command){
 println!("--> {:?} --> {:?} --> {:?}", command.name, command.flags, command.args);
}