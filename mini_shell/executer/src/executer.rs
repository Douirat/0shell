use types::command::*;
use executers::*;

pub fn execute(commands: &Commands) {
    for command in &commands.command {
        println!("--> {:?} --> {:?} --> {:?}", command.name, command.flags, command.args);
        match command.name {
            CommandType::Cd => cd(command),
            CommandType::Ls => ls(command), //(supporting -l, -a, -F)
            CommandType::Pwd =>pwd(command),
            CommandType::Cat => cat(command),
            CommandType::Cp => cp(command),
            CommandType::Rm => rm(command), // (supporting -r)
            CommandType::Mv =>mv(command),
            CommandType::Mkdir =>mkdir(command),
            CommandType::Exit => cd(command),
            _ => panic!("Error executing a command"),
        }
    }
}

//TODO executers = {path="executers"} Add this the the dependency when time for the executers to be done come.