// use std::fs::File;
// use std::io::{self, Read, Write};
/*
Primarily used to read and display the contents of files on the terminal.
Can concatenate multiple files and display them as a single continuous output.
*/
use types::command::*;

pub fn cat(command: &Command){
 println!("--> {:?} --> {:?} --> {:?}", command.name, command.flags, command.args);
}