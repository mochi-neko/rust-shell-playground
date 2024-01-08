mod builtin_command;
mod external_command;

use anyhow::anyhow;
use builtin_command::BuiltinCommand;
use external_command::ExternalCommand;

use std::{
    path::PathBuf,
    sync::{atomic::AtomicBool, Arc},
};

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

    // Parse and execute command
    let command = Command::find(command)?;
    let (output, args) = Output::from_args(args)?;
    command.execute(args, output)?;

    Ok(())
}

pub enum Output {
    Stdout,
    File(PathBuf),
}

impl Output {
    fn from_args<'a>(
        args: &'a [&'a str]
    ) -> anyhow::Result<(Self, &'a [&'a str])> {
        match args
            .iter()
            .filter(|&&arg| arg == ">")
            .count()
        {
            | 0 => Ok((Output::Stdout, args)),
            | 1 => {
                let index = args
                    .iter()
                    .position(|&arg| arg == ">")
                    .ok_or(anyhow::anyhow!(
                        "Redirection symbol not found."
                    ))?;
                let path = args
                    .get(index + 1)
                    .ok_or(anyhow::anyhow!("File path not found."))?;
                Ok((
                    Output::File(PathBuf::from(path)),
                    &args[..index],
                ))
            },
            | _ => {
                anyhow::bail!("More than two redirections are not supported.")
            },
        }
    }

    pub fn write(
        &self,
        content: String,
    ) -> anyhow::Result<()> {
        match self {
            | Self::Stdout => {
                println!("{}", content);
            },
            | Self::File(path) => {
                std::fs::write(path, content)?;
            },
        };

        Ok(())
    }

    pub fn write_error(
        &self,
        error: anyhow::Error,
    ) -> anyhow::Result<()> {
        match self {
            | Self::Stdout => {
                eprintln!("{}", error);
            },
            | Self::File(path) => {
                std::fs::write(path, error.to_string())?;
            },
        };

        Ok(())
    }
}

/// Commands in shell.
enum Command {
    Builtin(BuiltinCommand),
    External(ExternalCommand),
    NotFound(String),
}

impl Command {
    /// Finds a command from string.
    fn find(command: &str) -> anyhow::Result<Command> {
        // Try parse builtin command
        match BuiltinCommand::parse(command) {
            | Some(builtin_command) => Ok(Command::Builtin(builtin_command)),
            // Find external command
            | None => match ExternalCommand::find_command(command)? {
                // Found external command
                | Some(external_command) => {
                    Ok(Command::External(external_command))
                },
                // Not found in external command
                | None => Ok(Command::NotFound(command.to_string())),
            },
        }
    }

    fn execute(
        self,
        args: &[&str],
        output: Output,
    ) -> anyhow::Result<()> {
        let result = match self {
            | Command::Builtin(builtin_command) => builtin_command
                .execute(args)
                .map_err(|error| {
                    anyhow!(
                        "Execute builtin command error: {:?}",
                        error
                    )
                }),
            | Command::External(external_command) => external_command
                .execute(args)
                .map_err(|error| {
                    anyhow!(
                        "Execute external command error: {:?}",
                        error
                    )
                }),
            | Command::NotFound(command) => Err(anyhow!(
                "Command not found: {:?}",
                command
            )),
        };

        match result {
            | Ok(content) => {
                output.write(content)?;
            },
            | Err(error) => {
                output.write_error(error)?;
            },
        };

        Ok(())
    }
}
