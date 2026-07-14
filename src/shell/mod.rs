use std::os::unix::fs::PermissionsExt;
use std::{
    fs,
    io::{self, Write},
    path::Path,
    vec,
};
use std::env::consts::OS;
use std::process::Command;

pub mod commands;

struct ShellCommand {
    name: String,
    description: String,
    num_of_args: usize,
    func: fn(&mut Shell, & vec::Vec<&str>),
}
pub struct Shell {
    path: String,
    dirs: vec::Vec<String>,
    commands: std::collections::HashMap<&'static str, ShellCommand>,
}

impl Shell {
    pub fn new(path: &mut String) -> Shell {
        Shell {
            path: path.clone(),
            dirs: path.split(':').map(|s| s.to_string()).collect(),
            commands: std::collections::HashMap::new(),
        }
    }
    fn register_builtins(&mut self) {
        // echo command
        self.commands.insert(
            "echo",
            ShellCommand {
                name: String::from("echo"),
                description: String::from("echo is a shell builtin"),
                num_of_args: 1,
                func: commands::echo_func,
            },
        );

        // exit command
        self.commands.insert(
            "exit",
            ShellCommand {
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
            "type",
            ShellCommand {
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
            let tokens:Vec<&str>  = input.split_whitespace().collect();

            let found = self.find_in_builtins(&tokens);
            if found {
                continue;
            }

            let found = self.find_in_path(&tokens);
            if found {
                continue;
            }
            println!("{}: command not found", tokens[0]);
        }
    }
    pub fn find_in_builtins(&mut self, tokens: &Vec<&str>) -> bool {
        let command = tokens[0];
        if let Some(cmd) = self.commands.get(&command) {
            (cmd.func)(self, tokens);
            return true;
        }
        false
    }

    pub fn find_in_path(&self, tokens: &Vec<&str>) -> bool {
        let command = tokens[0];
        let mut  p = command;
        if command == "type" {
            p = tokens[1];
        }
        for dir in &self.dirs {
            let path = Path::new(dir).join(&p);

            if let Ok(metadata) = fs::metadata(&path) {
                let mode = metadata.permissions().mode();
                if metadata.is_file() && (mode & 0o111 != 0) {
                    if command == "type" {
                        println!("{} is {}", p, path.display());
                        return true;
                    }
                    let output = Command::new(command).args(&tokens[1..]).output();
                    println!("{}", String::from_utf8_lossy(&output.unwrap().stdout));
                    io::stdout().flush().unwrap();
                    return true;
                }
            }
        }
        false
    }
}
