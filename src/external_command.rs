use std::env;
use std::path::{Path, PathBuf};

/// External commands in shell.
pub(crate) struct ExternalCommand {
    pub(crate) path: PathBuf,
}

impl ExternalCommand {
    /// Finds a command if it exists as an external command.
    pub(crate) fn find_command(command: &str) -> anyhow::Result<Option<Self>> {
        // For relative and absolute paths, check if the path exists
        if command.contains('/') {
            let path = Path::new(command);
            if path.exists() {
                return Ok(Some(Self {
                    path: path.to_path_buf(),
                }));
            }
        }

        // For each directory in PATH, check if the command exists
        for path_directory in env::var("PATH")?.split(':') {
            let path = Path::new(path_directory).join(command);
            if path.exists() {
                return Ok(Some(Self {
                    path,
                }));
            }
        }

        Ok(None)
    }

    /// Executes an external command with arguments.
    pub(crate) fn execute(
        self,
        args: &[&str],
    ) -> anyhow::Result<String> {
        // Execute command by `std::process::Command`
        let output = std::process::Command::new(self.path)
            .args(args)
            .output()?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(anyhow::anyhow!(
                String::from_utf8_lossy(&output.stderr).into_owned()
            ))
        }
    }
}
