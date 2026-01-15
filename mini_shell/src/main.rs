use std::io::*;
use parser::parse;

fn main() {
    loop{
        print!("$");
        let _=stdout().flush();

        let mut input = String::new();
        match stdin().read_line(&mut input){
            Ok(_) =>  {
                match parse(&input) {
                    Ok(command) => {
                        println!("Command: {}", command.name);
                        println!("Args: {:?}", command.args);
                    }
                    Err(e) => {
                        println!("Parse error: {}", e);
                    }
                }
            }
            Err(_) => println!("Error inserting the input"),
        };
    }
}
