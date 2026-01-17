/*
It allows users to move from the current working directory to another specified directory by providing either an absolute path or a relative path.
This command is essential for exploring different locations within the Linux environment and managing files efficiently.
*/
use types::command::*;
pub fn cd(command: &Command){
 println!("--> {:?} --> {:?} --> {:?}", command.name, command.flags, command.args);
}