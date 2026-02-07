use std::io::*;
use parser::parse;
use executer::execute;
use types::state::*;
use std::rc::Rc;

fn main() {
    let initial_state = match State::init_state(){
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };
    let state = Rc::new(initial_state);
    loop {
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