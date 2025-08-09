use std::io::{ self, Write };
use std::env;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    println!();
                    break;
                }

                let trimmed_input = input.trim();

                if trimmed_input.is_empty() {
                    continue;
                }

                let mut parts = trimmed_input.split_whitespace();

                let command = parts.next().unwrap();

                let _args: Vec<&str> = parts.collect();

                match command {
                    "exit" => {
                        break;
                    }
                    "pwd" => {
                        match env::current_dir(){
                            Ok(path) => {
                                println!("{}",path.display());
                            }
                            Err(e) => {
                                eprintln!("pwd: {}", e);
                            }
                        }
                    }
                    _ => {
                        println!("Command '{}' not found", command);
                    }
                }
            }
            Err(error) => {
                eprintln!("Error reading line: {}", error);
                break;
            }
        }
    }
}
