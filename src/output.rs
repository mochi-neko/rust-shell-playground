use std::path::PathBuf;

/// Output of command execution.
pub(crate) enum Output {
    // Standard output
    Stdout,
    // Redirect to file by ">"
    File(PathBuf),
}

impl Output {
    /// Parses arguments and returns output.
    pub(crate) fn parse_args<'a>(
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

    /// Writes content to output.
    pub(crate) fn write(
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

    /// Writes error to output.
    pub(crate) fn write_error(
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
