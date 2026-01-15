use std::io::*;

fn main() {
    loop{
        print!("$");
        let _=stdout().flush();
        let mut input = String::new();
        match stdin().read_line(&mut input){
            Ok(_) =>  println!("${}", input.trim()),
            Err(v) => println!("Error inserting the input"),
        };
    }
}
