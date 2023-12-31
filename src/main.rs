mod builtin_command;

use std::io;

use builtin_command::BuiltinCommand;

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

        // Parse command
        match BuiltinCommand::from_str(command) {
            // If command is builtin, execute it
            | Some(builtin_command) => {
                builtin_command.execute(args)?;
            },
            // If command is not builtin, execute it
            | None => {
                // TODO: Execute external command
            },
        }
    }

    Ok(())
}
