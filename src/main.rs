#[allow(unused_imports)]
mod shell;

use shell::Shell;

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    let mut shell_path = std::env::var("PATH").unwrap_or_else(|_| String::from("shell"));
    let mut shell = Shell::new(&mut shell_path);
    shell.run();
}
