 use std::collections::HashMap;

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
    }
    }
