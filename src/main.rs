mod builtin_command;
mod external_command;

use builtin_command::BuiltinCommand;
use external_command::ExternalCommand;

use std::sync::{atomic::AtomicBool, Arc};

fn main() -> anyhow::Result<()> {
    // Deactivate Ctrl-C by signal handling
    let int = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(
        signal_hook::consts::SIGINT,
        Arc::clone(&int),
    )?;

    // Judge mode by args
    match std::env::args().nth(1) {
        // $ cargo run -- script.sh
        | Some(filename) => script_mode(filename)?,
        // $ cargo run
        | None => interactive_mode()?,
    };

    Ok(())
}

fn script_mode(filename: String) -> anyhow::Result<()> {
    // Read file
    let file = std::fs::read_to_string(filename)?;

    // Interpret for each line
    for line in file.lines() {
        interpret_line(line.to_string())?;
    }

    Ok(())
}

fn interactive_mode() -> anyhow::Result<()> {
    loop {
        // Create input buffer
        let mut input = String::new();

        // Read input from stdin
        std::io::stdin().read_line(&mut input)?;

        // Interpret line of input.
        interpret_line(input)?;
    }
}

/// Interprets a line as command.
fn interpret_line(line: String) -> anyhow::Result<()> {
    // Split input into elements
    let elements: Vec<&str> = line
        .split_whitespace()
        .collect();

    // If no elements, continue
    if elements.is_empty() {
        return Ok(());
    }

    // Get command and args
    let command = elements[0];
    let args = &elements[1..];

    // Parse and execute command
    let command = find_command(command)?;

    match command {
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
    }

    Ok(())
}

/// Commands in shell.
enum Command {
    Builtin(BuiltinCommand),
    External(ExternalCommand),
    NotFound(String),
}

/// Finds a command from string.
fn find_command(command: &str) -> anyhow::Result<Command> {
    // Try parse builtin command
    match BuiltinCommand::parse(command) {
        | Some(builtin_command) => Ok(Command::Builtin(builtin_command)),
        // Find external command
        | None => match ExternalCommand::find_command(command)? {
            // Found external command
            | Some(external_command) => Ok(Command::External(external_command)),
            // Not found in external command
            | None => Ok(Command::NotFound(command.to_string())),
        },
    }
}
