use std::{env::set_current_dir, io::stdin, process::ExitStatus};

use crate::parse::ParsedCommand;

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

                match ParsedCommand::parse_command(&input) {
                    Ok(parsed) => {
                        match parsed.command.as_str() {
                            // shell builtins
                            "cd" => {
                                let new_working_dir = match parsed.args.get(0) {
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
                            _ => match execute_external_command(parsed) {
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

/// Helper function to execute a command and wait on the results. Returns Err if
/// the command failed to execute. Return Ok(ExitStatus) after command successfully completes
fn execute_external_command(parsed: ParsedCommand) -> std::io::Result<ExitStatus> {
    let mut command = parsed.to_command();

    let exit_status = command.spawn()?.wait()?;

    Ok(exit_status)
}
