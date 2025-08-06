use std::io::{self, Write};

fn main(){
    loop{
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(bytes_read) => {
                if bytes_read == 0{
                    println!();
                    break;
                }
                println!("you entered: {}", input.trim());
            }
            Err(error) => {
                eprintln!("Error reading line: {}", error);
                break;
            }
        }
    }
}