#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        let mut words = input.split_whitespace();
        if let Some(command) = words.next() {
            match command {
                "echo" => {
                    for word in words {
                        print!("{} ", word);
                    }
                    println!();
                }
                "exit" => exit(0),
                _ => println!("{}: command not found", command),
            }
            io::stdout().flush().unwrap();
        }
    }
}
