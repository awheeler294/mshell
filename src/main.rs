
// mod parser;

use std::{
    io::stdin,
    process::Command
};

fn main() {
    let mut input = String::new();

    loop {
        input.truncate(0);

        eprint!("$ ");

        match stdin().read_line(&mut input) {
            Ok(_len) => {
                if let Err(e) = execute_command(&input) {
                    eprintln!("Error: {e}");
                }
            },
            Err(e) => {eprintln!("Error reading input: {e}")},
        }
    }
}

fn execute_command(command: &String) -> std::io::Result<()> {
    let command = command.trim();

    let _handle = Command::new(command).spawn()?.wait()?;

    Ok(())
}
