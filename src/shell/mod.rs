use std::{
    io::{self, Write},
    vec,
};

pub mod commands;

struct Command {
    name: String,
    description: String,
    num_of_args: usize,
    func: fn(&mut Shell, &mut vec::Vec<String>),
}
pub struct Shell {
    commands: std::collections::HashMap<String, Command>,
}

impl Shell {
    pub fn new() -> Shell {
        Shell {
            commands: std::collections::HashMap::new(),
        }
    }
    fn register_builtins(&mut self) {
        // echo command
        self.commands.insert(
            "echo".to_string(),
            Command {
                name: String::from("echo"),
                description: String::from("echo is a shell builtin"),
                num_of_args: 1,
                func: commands::echo_func,
            },
        );

        // exit command
        self.commands.insert(
            "exit".to_string(),
            Command {
                name: String::from("exit"),
                description: String::from("exit is a shell builtin"),
                num_of_args: 0,
                func: |_shell, _args| {
                    std::process::exit(0);
                },
            },
        );

        // type command
        self.commands.insert(
            "type".to_string(),
            Command {
                name: String::from("type"),
                description: String::from("type is a shell builtin"),
                num_of_args: 1,
                func: commands::type_func,
            },
        );
    }

    pub fn run(&mut self) {
        self.register_builtins();
        loop {
            print!("$ ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            let mut words = input.split_whitespace();
            if let Some(command) = words.next() {
                if let Some(cmd) = self.commands.get(command) {
                    let mut args = vec::Vec::new();
                    for word in words {
                        args.push(word.to_string());
                    }
                    (cmd.func)(self, &mut args);
                } else {
                    println!("{}: command not found", command);
                }
            }
        }
    }
}
