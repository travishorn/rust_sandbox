use std::env;

pub fn main() {
    // First item will be the running executable
    // For example, it might be "target\\debug\\rust_sandbox.exe"
    // Any remaining items will be arguments passed in
    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);

    // Make sure some command was given
    if args.len() > 1 {
        let command = args[1].clone();

        // Print the name of the given command
        println!("Command: {}", command);

        // Conditional based on command
        if command == "hello" {
            println!("Hello. How are you?");
        } else if command == "goodbye" {
            println!("Goodbye. Have a great day.");
        } else {
            // Anything other than "hello" or "goodbye"
            println!("That is not a valid command.");
        }
    } else {
        // No command was given
        println!("Please specify a command.");
    }
}
