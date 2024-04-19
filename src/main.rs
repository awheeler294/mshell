use std::{
    env::set_current_dir,
    io::{stdin, ErrorKind},
    process::{Command, ExitStatus},
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
            Ok(len) => {
                // handle EOF signal
                if len == 0 {
                    break;
                }

                match parse_command(&input) {
                    Ok(command) => {
                        if let Some(command_name) = command.get(0) {
                            match command_name.as_str() {
                                // shell builtins
                                "cd" => {
                                    let new_working_dir = match command.get(1) {
                                        Some(path) => path,
                                        None => ".",
                                    };
                                    if let Err(e) = set_current_dir(new_working_dir) {
                                        eprintln!("Error changing directories: {e}");
                                    }
                                }

                                "exit" => {
                                    break;
                                }

                                // execute external command
                                _ => match execute_command(&command) {
                                    Ok(exit_status) => {
                                        if exit_status.success() == false {
                                            eprintln!("Error: command exited with {exit_status}");
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!("Error: {e}");
                                    }
                                },
                            };
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

fn execute_command(command_parts: &[String]) -> std::io::Result<ExitStatus> {
    let mut iter = command_parts.iter();

    let mut command = Command::new(iter.next().ok_or_else(|| {
        eprintln!("Error: empty command");
        ErrorKind::Other
    })?);

    for arg in iter {
        command.arg(arg);
    }

    let exit_status = command.spawn()?.wait()?;

    Ok(exit_status)
}


