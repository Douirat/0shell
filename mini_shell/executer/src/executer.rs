use types::command::*;
use executers::{
    cd::cd,
    ls::ls,
    pwd::pwd,
    cat::cat,
    cp::cp,
    rm::rm,
    mv::mv,
    mkdir::mkdir,
    exit::exit,
};

pub fn execute(commands: &Commands) {

    for command in &commands.command {
      
        match &command.name {
            CommandType::Cd => cd(command),
            CommandType::Ls => ls(command), //(supporting -l, -a, -F)
            CommandType::Pwd =>pwd(command),
            CommandType::Cat => cat(command),
            CommandType::Cp => cp(command),
            CommandType::Rm => rm(command), // (supporting -r)
            CommandType::Mv =>mv(command),
            CommandType::Mkdir =>mkdir(command),
            CommandType::Exit => exit(command),
            _ => println!("error: --> {:?}", command),
        }
    }
}
