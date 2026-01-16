use types::command::{Commands, Command, CommandType, Flag};
use CommandType::*;
use Flag::*; 
use executer::executer::*;

fn main() {
    let all_commands = Commands {
        command: vec![
            // Echo (no flags)
            Command { name: Echo, flags: vec![], args: vec!["Hello".into()] },

            // Cd (no flags)
            Command { name: Cd, flags: vec![], args: vec!["/home".into()] },

            // Ls (all flag combinations: -l, -a, -F, -l -a, -l -F, -a -F, -l -a -F)
            Command { name: Ls, flags: vec![], args: vec![".".into()] },
            Command { name: Ls, flags: vec![L], args: vec![".".into()] },
            Command { name: Ls, flags: vec![A], args: vec![".".into()] },
            Command { name: Ls, flags: vec![F], args: vec![".".into()] },
            Command { name: Ls, flags: vec![L, A], args: vec![".".into()] },
            Command { name: Ls, flags: vec![L, F], args: vec![".".into()] },
            Command { name: Ls, flags: vec![A, F], args: vec![".".into()] },
            Command { name: Ls, flags: vec![L, A, F], args: vec![".".into()] },

            // Pwd (no flags)
            Command { name: Pwd, flags: vec![], args: vec![] },

            // Cat (no flags)
            Command { name: Cat, flags: vec![], args: vec!["file.txt".into()] },

            // Cp (no flags)
            Command { name: Cp, flags: vec![], args: vec!["src.txt".into(), "dst.txt".into()] },

            // Rm (flag -r and no flag)
            Command { name: Rm, flags: vec![], args: vec!["file.txt".into()] },
            Command { name: Rm, flags: vec![R], args: vec!["folder".into()] },

            // Mv (no flags)
            Command { name: Mv, flags: vec![], args: vec!["old.txt".into(), "new.txt".into()] },

            // Mkdir (no flags)
            Command { name: Mkdir, flags: vec![], args: vec!["new_folder".into()] },

            // Exit (no flags)
            Command { name: Exit, flags: vec![], args: vec![] },
        ],
    };
execute(&all_commands);
    // loop{
    //     print!("$");
    //     let _=stdout().flush();

    //     let mut input = String::new();
    //     match stdin().read_line(&mut input){
    //         Ok(_) =>  {
    //             match parse(&input) {
    //                 Ok(command) => {
    //                     println!("Command: {}", command.name);
    //                     println!("Args: {:?}", command.args);
    //                 }
    //                 Err(e) => {
    //                     println!("Parse error: {}", e);
    //                 }
    //             }
    //         }
    //         Err(_) => println!("Error inserting the input"),
    //     };
    // }
}
