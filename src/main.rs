use std::fs;
use std::io;

fn main() {

    loop {

        let mut line = String::new();
        
        // Read command in the format of "kong file_dir"
        io::stdin().read_line(&mut line).expect("Failed to read line");

        // Split the command into two parts
        let mut iter = line.split_whitespace();
        let command = iter.next();
        let file_dir = iter.next();

        match command {
            Some("kong") => {
                match file_dir {
                    Some(dir) => {
                        println!("Kong is running...");
                        match fs::read_to_string(dir) {
                            Ok(contents) => println!("{}", contents),
                            Err(e) => eprintln!("Failed to read file: {}", e),
                        }
                    },
                    None => {
                        println!("No file directory provided. Usage: kong [file_dir]");
                    }
                }
            },

            Some("ls") => {
                match file_dir {
                    Some(dir) => {
                        println!("Listing files in directory...");
                        match fs::read_dir(dir) {
                            Ok(entries) => {
                                for entry in entries {
                                    match entry {
                                        Ok(entry) => println!("{}", entry.file_name().to_string_lossy()),
                                        Err(e) => eprintln!("Failed to read directory entry: {}", e),
                                    }
                                }
                            },
                            Err(e) => eprintln!("Failed to read directory: {}", e),
                        }
                    },
                    None => {
                        println!("No directory provided. Usage: ls [dir]");
                    }
                }
            },

            Some(_) => {
                println!("Invalid command. Usage: kong [file_dir]");
            },
            None => {
                println!("No command provided. Usage: kong [file_dir]");
            },
        }
    }
}
