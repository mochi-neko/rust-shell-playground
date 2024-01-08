use crate::builtin_command::BuiltinCommand;
use crate::external_command::ExternalCommand;
use anyhow::anyhow;

/// Commands in this shell.
pub(crate) enum Command {
    /// Builtin commands in this shell.
    Builtin(BuiltinCommand),
    /// External commands in this shell.
    External(ExternalCommand),
    /// Command not found.
    NotFound(String),
}

impl Command {
    /// Finds a command from string.
    pub(crate) fn find(command: &str) -> anyhow::Result<Command> {
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

    /// Executes a command with arguments.
    pub(crate) fn execute(
        self,
        args: &[&str],
    ) -> anyhow::Result<String> {
        match self {
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
        }
    }
}
