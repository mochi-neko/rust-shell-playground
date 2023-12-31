mod builtin_command;
mod external_command;

use std::io;

use builtin_command::BuiltinCommand;
use external_command::ExternalCommand;

fn main() -> anyhow::Result<()> {
    // Create input buffer
    let mut input = String::new();

    loop {
        // Clear input buffer
        input.clear();

        // Read input from stdin
        io::stdin().read_line(&mut input)?;

        // Split input into elements
        let elements: Vec<&str> = input
            .split_whitespace()
            .collect();

        // If no elements, continue
        if elements.is_empty() {
            continue;
        }

        // Get command and args
        let command = elements[0];
        let args = &elements[1..];

        // Parse and execute command
        match find_command(command) {
            | Command::Builtin(builtin_command) => {
                builtin_command
                    .execute(args)
                    .map_err(|error| {
                        eprintln!(
                            "Execute builtin command error: {:?}",
                            error
                        );
                        error
                    })?;
            },
            | Command::External(external_command) => {
                external_command
                    .execute(args)
                    .map_err(|error| {
                        eprintln!(
                            "Execute external command error: {:?}",
                            error
                        );
                        error
                    })?;
            },
            | Command::NotFound(command) => {
                eprintln!("Command not found: {}", command);
            },
            | Command::Error(error) => {
                eprintln!("Error: {:?}", error);
            },
        }
    }
}

/// Commands in shell.
enum Command {
    Builtin(BuiltinCommand),
    External(ExternalCommand),
    NotFound(String),
    Error(anyhow::Error),
}

fn find_command(command: &str) -> Command {
    BuiltinCommand::parse(command)
        .map(Command::Builtin)
        .unwrap_or_else(|| {
            ExternalCommand::find_command(command)
                .map(|external_command| {
                    external_command.map_or_else(
                        || Command::NotFound(command.to_string()),
                        Command::External,
                    )
                })
                .unwrap_or_else(Command::Error)
        })
}
