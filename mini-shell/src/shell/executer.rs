use libc::{fork, waitpid, _exit};
use super::executers; // sibling module

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
    "echo" => executers::echo::echo(&cmd.args),
    "cd" => {
        if !cmd.args.is_empty() {
            executers::cd::cd(&cmd.args[0]);
        } else {
            eprintln!("cd: missing argument");
        }
    },
    "pwd" => executers::pwd::pwd(),
    "cat" => {
        if !cmd.args.is_empty() {
            executers::cat::cat(&cmd.args[0]);
        } else {
            eprintln!("cat: missing argument");
        }
    },
    "cp" => {
        if cmd.args.len() >= 2 {
            executers::cp::cp(&cmd.args[0], &cmd.args[1]);
        } else {
            eprintln!("cp: missing source or destination");
        }
    },
    "rm" => executers::rm::rm(),
    "mv" => executers::mv::mv(),
    "mkdir" => executers::mkdir::mkdir(),
    "exit" => executers::exit::exit(),
    _ => eprintln!("Command '{}' not implemented yet", cmd.name),
}


                _exit(0); // exit child after execution
            } else if pid > 0 {
                // PARENT PROCESS: wait for child
                waitpid(pid, std::ptr::null_mut(), 0);
            } else {
                eprintln!("Fork failed");
            }
        }
    }
}
