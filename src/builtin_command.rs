/// Builtin commands in shell.
pub(crate) enum BuiltinCommand {
    Echo,
    Exit,
}

impl BuiltinCommand {
    /// Parses a string to a command if it is a builtin command.
    pub(crate) fn parse(s: &str) -> Option<Self> {
        match s {
            | "echo" => Some(BuiltinCommand::Echo),
            | "exit" => Some(BuiltinCommand::Exit),
            | _ => None,
        }
    }

    /// Executes a builtin command with arguments.
    pub(crate) fn execute(
        self,
        args: &[&str],
    ) -> anyhow::Result<String> {
        match self {
            | BuiltinCommand::Echo => Ok(args.join(" ").to_string()),
            | BuiltinCommand::Exit => {
                std::process::exit(0);
            },
        }
    }
}
