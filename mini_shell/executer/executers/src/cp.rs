
    /*
    The cp command is used to copy files and directories from one location to another
    It's like making a duplicate of your file or folder.
     */


use types::command::*;

pub fn cp(command: &Command){
 println!("--> {:?} --> {:?} --> {:?}", command.name, command.flags, command.args);
}