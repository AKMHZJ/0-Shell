use std::io::{ self, Write };
use std::env;
use std::path::Path;

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

                let args: Vec<&str> = parts.collect();

                match command {
                    "exit" => {
                        break;
                    }
                    "pwd" => {
                        match env::current_dir() {
                            Ok(path) => {
                                println!("{}", path.display());
                            }
                            Err(e) => {
                                eprintln!("pwd: {}", e);
                            }
                        }
                    }
                    "echo" => {
                        let output = args.join(" ");
                        println!("{}", output);
                    }
                    "cd" => {
                        let path_arg = args.get(0);

                        if path_arg.is_none() {
                            if let Some(home_dir) = home::home_dir() {
                                if let Err(e) = env::set_current_dir(home_dir) {
                                    eprintln!("cd: {}", e);
                                }
                            } else {
                                eprintln!("cd: could not find home directory");
                            }
                        } else {
                            let path = Path::new(path_arg.unwrap());
                            if let Err(e) = env::set_current_dir(path){
                                eprintln!("cd: {}: {}", e, path.display());
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
