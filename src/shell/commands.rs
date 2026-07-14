use std::vec;

use crate::shell;

pub fn echo_func(_shell: &mut shell::Shell, args: &mut vec::Vec<String>) {
    // Implementation for echo command
    println!("{}", args.join(" "));
    return;
}

pub fn type_func(shell: &mut shell::Shell, args: &mut vec::Vec<String>) {
    // Implementation for type command
    let arg = args[0].clone();
    let command = shell.commands.get(&arg);
    if let Some(cmd) = command {
        println!("{}", cmd.description);
    } else {
        println!("{}: not found", arg);
    }
    return;
}
