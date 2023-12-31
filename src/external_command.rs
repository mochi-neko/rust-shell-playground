use std::env;
use std::path::{Path, PathBuf};

pub(crate) struct ExternalCommand {
    pub(crate) path: PathBuf,
}

impl ExternalCommand {
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

    pub(crate) fn execute(
        self,
        args: &[&str],
    ) -> anyhow::Result<()> {
        println!(
            "Executing external command: {:?} {:?}",
            self.path, args,
        );

        Ok(())
    }
}
