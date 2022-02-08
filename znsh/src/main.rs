use std::io;
use std::io::Write;

fn main() {
    println!("************* Welcome to znsh **************");
    // Infinite loop until exit is called
    loop {
        print!("user@znsh:/insert/cwd/here ➜ ");
        // Take user input and write to a string line
        io::stdout().flush().unwrap();
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        print!("{}", line);
    }
}
