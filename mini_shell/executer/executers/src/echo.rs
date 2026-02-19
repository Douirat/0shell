/*
The echo command is used to show a line of text or a variable's value in the terminal.
The echo command has several options to customize its output:

-n - Don't add a new line at the end
-e - Allow special characters like \n for new lines
-E - Don't allow special characters (default)
*/
// shell/executers/echo.rs

use types::command::*;

pub fn echo(command: &Command) {
    let output = command.args.join(" ");
    let output = interpret_escapes(&output);
    println!("{}", output);
}

fn interpret_escapes(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('n')  => result.push('\n'),
                Some('t')  => result.push('\t'),
                Some('r')  => result.push('\r'),
                Some('\\') => result.push('\\'),
                Some('"')  => result.push('"'),
                Some('\'') => result.push('\''),
                Some('0')  => result.push('\0'),
                Some(c)    => { result.push('\\'); result.push(c); }
                None       => result.push('\\'),
            }
        } else {
            result.push(ch);
        }
    }

    result
}