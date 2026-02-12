use std::io::*;
use parser::parse;
use executer::execute;
use types::state::*;
use std::rc::Rc;

fn main() {
    let state = Rc::new(State::init_state());
    loop {
            let current_path = state.cwd.borrow().display().to_string();
        println!(
                // "\x1b[32m{}@{} ~{}\x1b[0m",
               "\x1b[31m{}@{}\x1b[0m \x1b[32m~{}\x1b[0m",
                state.user,
                state.host,
                current_path
            );

        print!("$ ");
        let _ = stdout().flush();

        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => {
                match parse(&state, &input) {
                    Ok(commands) => {
                        execute(&commands);
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
            }
            Err(_) => eprintln!("Error reading input"),
        }
    }
}