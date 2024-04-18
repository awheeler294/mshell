use std::{
    io::{stdin, ErrorKind},
    process::Command,
};

use crate::parse::parse_command;

mod parse;

const PROMPT: &str = "$ ";

fn main() {
    let mut input = String::new();

    loop {
        input.truncate(0);

        eprint!("{PROMPT}");

        match stdin().read_line(&mut input) {
            Ok(_len) => {
                dbg!(&input);
                match parse_command(&input) {
                    Ok(command) => {
                        dbg!(&command);
                        if let Err(e) = execute_command(&command) {
                            eprintln!("Error: {e}");
                        }
                    }
                    Err(e) => {
                        eprintln!("Error parsing input: {e}")
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading input: {e}")
            }
        }
    }
}

fn execute_command(command_parts: &[String]) -> std::io::Result<()> {
    let mut iter = command_parts.iter();

    let mut command = Command::new(iter.next().ok_or_else(|| {
        eprintln!("Error: empty command");
        ErrorKind::Other
    })?);

    for arg in iter {
        command.arg(arg);
    }

    let _handle = command.spawn()?.wait()?;

    Ok(())
}
