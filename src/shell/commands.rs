use std::vec;

use crate::shell;

pub fn echo_func(_shell: &mut shell::Shell, args: & vec::Vec<String>) {
    // Implementation for echo command
    println!("{}", args.join(" "));
    return;
}

pub fn type_func(shell: &mut shell::Shell, tokens: & vec::Vec<String>) {
    // Implementation for type command
    let arg = tokens[1];
    let command = shell.commands.get(&arg);
    if let Some(cmd) = command {
        println!("{}", cmd.description);
    } else {
        let found = shell.find_in_path(tokens);
        if found {
            return
        }
    }
    println!("{}: not found", arg);
    return;
}
