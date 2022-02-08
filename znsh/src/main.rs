#![allow(unused)]

use std::io;
use std::io::Write;
use std::env;
use std::path::Path;
use std::process::Command;
use colored::*;

fn main() {
    let home = dirs::home_dir().unwrap();

    println!("{}", "************* Welcome to znsh **************".bold());
    // Infinite loop until exit is called
    loop {
        // Get cwd and convert to string (for header)
        let mut cwd = std::env::current_dir().unwrap().as_path().display().to_string();
        // Convert $HOME prefix to nothing or to ~ if cwd is $HOME
        if cwd.starts_with(home.as_path().display().to_string().as_str()) {
            if (cwd == home.as_path().display().to_string()) {
                cwd = "~".to_string();
            } else {
                cwd = cwd.trim_start_matches(&home.as_path().display().to_string()).to_string();
                cwd = cwd.trim_start_matches("/").to_string();
            }
        }
        // Get username
        let key = "USER";
        let user = match env::var(key) {
            Ok(val) => val,
            Err(e) => String::from("root"),
        };
        print!("{}{}: {} {} ", user.cyan(), "@znsh".cyan(), cwd.blue(), "➜".green());
        // Take user input and write to a string line
        io::stdout().flush().unwrap();
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        // Truncate newline from line
        line.truncate(line.len() - 1);
        // println!("line = {}", line);

        // Tokenize line into substrings, collect iterator into a vector names
        let mut names: Vec<&str> = line.split(" ").collect();
        // println!("{}", names[0]);

        // Implement cd wrapper command
        if names[0] == "cd" {
            if names.len() > 1 {
                // names[1] exists
                let path = Path::new(names[1]);
                // Check if path exists
                if path.exists() && path.is_dir() {
                    env::set_current_dir(path)
                        .expect(format!("cd: could not chdir() to path {}", names[1]).as_str());
                } else if path.exists() && !path.is_dir() {
                    println!("cd: not a directory: {}", names[1])
                } else if (names[1] == "~") {
                    env::set_current_dir(home.as_path())
                        .expect("Error: could not chdir() to HOME");
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