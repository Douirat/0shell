 use std::collections::HashMap;
 use nix::unistd::{fork, ForkResult, getpid, execvp};

pub fn execute_command(commands: &crate::shell::parser::Commands) {
// since i'm validating the comand i will validate the args as well
    let valid_commands = vec!["echo", "cd", "ls", "pwd", "cat", "cp", "rm", "mv", "mkdir", "exit"];
    let valid_args: HashMap<&str, Vec<&str>> = [
        // ("echo", vec!["-n"]),
        ("ls", vec!["-l", "-a", "-F"]),
        ("rm", vec!["-r"]),
    ].iter().cloned().collect();

    

    for cmd in &commands.commands {
        if !valid_commands.contains(&cmd.name.as_str()) {
            eprintln!("Command '{}' not found", cmd.name);
            continue;
        }

        if let Some(valid) = valid_args.get(cmd.name.as_str()) {
            for arg in &cmd.args {
                if !valid.contains(&arg.as_str()) {
                    eprintln!("Invalid argument '{}' for command '{}'", arg, cmd.name);
                }
            }
        }

        match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            println!("Child process: PID = {}", getpid());
            // Child can do something different here
        }
        Ok(ForkResult::Parent { child }) => {
            println!("Parent process: PID = {}, child PID = {}", getpid(), child);
            // Parent can do something here
        }
        Err(err) => {
            eprintln!("Fork failed: {}", err);
        }
    }
}

}


    

