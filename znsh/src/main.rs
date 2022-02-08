#![allow(unused)]

use std::io;
use std::io::Write;
use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let home = dirs::home_dir().unwrap();

    println!("************* Welcome to znsh **************");
    // Infinite loop until exit is called
    loop {
        print!("user@znsh:/insert/cwd/here ➜ ");
        // Take user input and write to a string line
        io::stdout().flush().unwrap();
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        // Convert line to lowercase for processing
        line = line.to_lowercase();
        // Truncate newline from line
        line.truncate(line.len() - 1);
        println!("line = {}", line);

        // Tokenize line into substrings, collect iterator into a vector names
        let mut names: Vec<&str> = line.split(" ").collect();
        println!("{}", names[0]);

        // Implement cd wrapper command
        if names[0] == "cd" {
            if names.len() > 1 {
                // names[1] exists
                let path = Path::new(names[1]);
                // Check if path exists
                if path.exists() {
                    env::set_current_dir(path)
                        .expect(format!("cd: could not chdir() to path {}", names[1]).as_str());
                } else {
                    println!("cd: no such file or directory: {}", names[1])
                }
            } else {
            // otherwise, chdir to HOME
            env::set_current_dir(home.as_path())
                .expect("Error: could not chdir() to HOME");
            }
            continue;
        }

        // Implement exit wrapper command
        if names[0] == "exit" {
            std::process::exit(0);
        }
        
        // Execute command
        let cmd_name = names[0];
        names.remove(0);
        let mut child = Command::new(cmd_name)
                                .args(names)
                                .spawn()
                                .expect(format!("znsh: {} failed", cmd_name).as_str());
        
        let parent = child.wait()
                        .expect("failed to wait on child");
    }
}