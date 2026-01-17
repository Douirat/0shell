use types::command::*;

pub fn rm(command: &Command){
 println!("--> {:?} --> {:?} --> {:?}", command.name, command.flags, command.args);
}