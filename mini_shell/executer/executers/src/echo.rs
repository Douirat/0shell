/*
The echo command is used to show a line of text or a variable's value in the terminal.
The echo command has several options to customize its output:

-n - Don't add a new line at the end
-e - Allow special characters like \n for new lines
-E - Don't allow special characters (default)
*/
// shell/executers/echo.rs

use types::command::*;

pub fn echo(command: &Command){
 println!("--> {:?} --> {:?} --> {:?}", command.name, command.flags, command.args);
}