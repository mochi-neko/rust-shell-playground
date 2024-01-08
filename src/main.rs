mod builtin_command;
mod command;
mod external_command;
mod output;

use command::Command;
use output::Output;
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
    // Ignore comment lines
    if line.starts_with('#') {
        return Ok(());
    }

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

    // Parse command and args
    let command = Command::find(command)?;
    let (output, args) = Output::parse_args(args)?;

    // Execute command and write output
    match command.execute(args) {
        | Ok(content) => {
            output.write(content)?;
        },
        | Err(error) => {
            output.write_error(error)?;
        },
    };

    Ok(())
}
