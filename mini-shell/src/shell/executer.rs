use std::env::current_dir;
use libc::{fork, waitpid, _exit, c_int};


pub fn execute_command(commands: &crate::shell::parser::Commands) {
    let valid_commands = vec!["echo", "cd", "ls", "pwd", "cat", "cp", "rm", "mv", "mkdir", "exit"];

    for cmd in &commands.commands {
        if !valid_commands.contains(&cmd.name.as_str()) {
            eprintln!("Command '{}' not found", cmd.name);
            continue;
        }

        unsafe {
            let pid = fork();
            if pid == 0 {
                // CHILD PROCESS: Execute the appropriate command
                match cmd.name.as_str() {
                    "echo" => executers::echo(&cmd.args),
                    "cd" => {
                        // call the cd function
                    },
                    "pwd" => {
                   // call the pwd function
                    },
                    "cat" => {
                        
                    // call the cat function
                    },
                    "cp" => {
                        // call the cp function
                    },
                    "exit" => {
                        _exit(0);
                    },
                    _ => {
                        eprintln!("Command '{}' not implemented yet", cmd.name);
                    }
                }

                _exit(0); // Exit child after execution
            } else if pid > 0 {
                // PARENT PROCESS: Wait for child to finish
                waitpid(pid, std::ptr::null_mut(), 0);
            } else {
                eprintln!("Fork failed");
            }
        }
    }

}


    

