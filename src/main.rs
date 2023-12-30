use std::io;

fn main() {
    // Create input buffer
    let mut input = String::new();

    loop {
        // Clear input buffer
        input.clear();

        // Read input from stdin
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

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

        // Print command and args
        println!("Command: {:?}", command);
        println!("Args: {:?}", args);
    }
}
