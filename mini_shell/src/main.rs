use std::io::*;
use parser::parse;
use executer::execute;
use types::state::*;
use std::rc::Rc;

enum OpenQuote {
    Single,
    Double,
}

fn unclosed_quote(input: &str) -> Option<OpenQuote> {
    let mut in_single = false;
    let mut in_double = false;
    let mut escaped = false;

    for ch in input.chars() {
        if escaped {
            escaped = false;
            continue;
        }
        match ch {
            '\\' => {
                escaped = true;
            }
            '\'' if !in_double => {
                in_single = !in_single;
            }
            '"' if !in_single => {
                in_double = !in_double;
            }
            _ => {}
        }
    }

    if in_single {
        Some(OpenQuote::Single)
    } else if in_double {
        Some(OpenQuote::Double)
    } else {
        None
    }
}

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
            Ok(0) => {
                break;
            }
            Ok(_) => {
                // Continuer à lire tant que des quotes sont ouvertes
                loop {
                    match unclosed_quote(&input) {
                        Some(OpenQuote::Single) => {
                            print!("quote> ");
                        }
                        Some(OpenQuote::Double) => {
                            print!("dquote> ");
                        }
                        None => {
                            break;
                        }
                    }

                    let _ = stdout().flush();

                    let mut extra = String::new();
                    match stdin().read_line(&mut extra) {
                        Ok(0) | Err(_) => {
                            break;
                        }
                        Ok(_) => input.push_str(&extra),
                    }
                }

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
