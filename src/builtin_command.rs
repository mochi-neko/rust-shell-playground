/// Builtin commands in shell.
pub(crate) enum BuiltinCommand {
    Echo,
    Exit,
}

impl BuiltinCommand {
    pub(crate) fn parse(s: &str) -> Option<Self> {
        match s {
            | "echo" => Some(BuiltinCommand::Echo),
            | "exit" => Some(BuiltinCommand::Exit),
            | _ => None,
        }
    }

    pub(crate) fn execute(
        self,
        args: &[&str],
    ) -> anyhow::Result<()> {
        match self {
            | BuiltinCommand::Echo => {
                println!("{}", args.join(" "));
            },
            | BuiltinCommand::Exit => {
                std::process::exit(0);
            },
        }

        Ok(())
    }
}
