use std::io::{ self, Write };
use std::{env};
use std::path::Path;
use std::fs;
use std::os::unix::fs::{ MetadataExt, PermissionsExt };
use chrono::{ DateTime, Local };
use users::{ get_user_by_uid, get_group_by_gid };

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
                            if let Err(e) = env::set_current_dir(path) {
                                eprintln!("cd: {}: {}", e, path.display());
                            }
                        }
                    }
                    "ls" => {
                        let mut show_hidden = false;
                        let mut classify = false;
                        let mut path_arg = None;
                        let mut long_listing = false;

                        for arg in &args {
                            if arg.starts_with('-') {
                                for ch in arg.chars().skip(1) {
                                    match ch {
                                        'a' => {
                                            show_hidden = true;
                                        }
                                        'F' => {
                                            classify = true;
                                        }
                                        'l' => {
                                            long_listing = true;
                                        }
                                        _ => {}
                                    }
                                }
                            } else {
                                path_arg = Some(arg);
                            }
                        }

                        let path = Path::new(*path_arg.unwrap_or(&"."));

                        let entries = match fs::read_dir(path) {
                            Ok(entries) => entries,
                            Err(e) => {
                                eprint!("ls: connot access '{}': {}", path.display(), e);
                                continue;
                            }
                        };

                        for entry in entries {
                            if let Ok(entry) = entry {
                                let file_name = entry.file_name();
                                let mut file_name_str = file_name.to_string_lossy().into_owned();

                                if show_hidden || !file_name_str.starts_with('.') {
                                    let metadata = match entry.metadata() {
                                        Ok(meta) => meta,
                                        Err(e) => {
                                            eprintln!("ls: cannot get metadata for '{}': {}",file_name_str,e);
                                            continue;
                                        }
                                    };

                                    if long_listing {
                                        let perms = metadata.permissions();
                                        let mode = perms.mode();
                                        let is_dir = metadata.is_dir();
                                        let perms_str = format!(
                                            "{}{}{}{}{}{}{}{}{}{}", 
                                            if is_dir { 'd' } else { '-' },
                                            if mode & 0o400 != 0 { 'r' } else { '-' },
                                            if mode & 0o200 != 0 { 'w' } else { '-' },
                                            if mode & 0o100 != 0 { 'x' } else { '-' },
                                            if mode & 0o040 != 0 { 'r' } else { '-' },
                                            if mode & 0o020 != 0 { 'w' } else { '-' },
                                            if mode & 0o010 != 0 { 'x' } else { '-' },
                                            if mode & 0o004 != 0 { 'r' } else { '-' },
                                            if mode & 0o002 != 0 { 'w' } else { '-' },
                                            if mode & 0o001 != 0 { 'x' } else { '-' }
                                        );

                                        let link_count = metadata.nlink();

                                        let uid = metadata.uid();
                                        let gid = metadata.gid();
                                        let owner = get_user_by_uid(uid).map(|u| u.name().to_string_lossy().into_owned()).unwrap_or_else(|| uid.to_string());
                                        let group = get_group_by_gid(gid).map(|g| g.name().to_string_lossy().into_owned()).unwrap_or_else(|| gid.to_string());

                                        let size = metadata.len();

                                        let modified_time: DateTime<Local> = metadata.modified().unwrap().into();
                                        let time_str = modified_time.format("%b %d %H:%M").to_string();

                                        if is_dir && classify { file_name_str.push('/'); }

                                        println!(
                                            "{} {:>3} {:<8} {:<8} {:>8} {} {}",
                                            perms_str, link_count, owner, group, size, time_str, file_name_str
                                        );
                                    } else {
                                        if metadata.is_dir() && classify { file_name_str.push('/'); }
                                        print!("{}  ", file_name_str);
                                    }
                                }
                            }
                        }
                        if !long_listing {
                            println!();
                        }
                    }
                    "cat" => {
                        if args.is_empty() {
                            eprintln!("cat: missing operand");
                            continue;
                        }

                        for path in args {
                            match fs::read_to_string(path) {
                                Ok(contents) => {
                                    print!("{}", contents);
                                }
                                Err(e) => {
                                    eprintln!("cat: {}: {}", path, e);
                                }
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
