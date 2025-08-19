use std::io::{ self, Write };
use std::env;
use std::path::Path;
use std::fs;

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
                    "ls" => {
                        let mut show_hidden = false;
                        let mut classify = false;
                        let mut path_arg = None;

                        for arg in &args{
                            if arg.starts_with('-'){
                                for ch in arg.chars().skip(1) {
                                    match ch {
                                        'a' => show_hidden = true,
                                        'F' => classify = true,
                                        _ => {},
                                    }
                                }
                            } else {
                                path_arg = Some(arg);
                            }
                        }

                        let path = Path::new(*path_arg.unwrap_or(&"."));

                        match fs::read_dir(path){
                            Ok(entries) => {
                                for entry in entries{
                                    if let Ok(entry) = entry {
                                        let file_name = entry.file_name();
                                        let mut file_name_str = file_name.to_string_lossy().into_owned();

                                        if show_hidden || !file_name_str.starts_with('.'){
                                            if classify {
                                                if let Ok(metadata) = entry.metadata() {
                                                    if metadata.is_dir(){
                                                        file_name_str.push('/');
                                                    }
                                                }
                                            }
                                            print!("{} ", file_name_str);
                                        }
                                    }
                                }
                                println!();
                            }
                            Err(e) => {
                                eprintln!("ls: cannot access '{}': {}", path.display(), e);
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
