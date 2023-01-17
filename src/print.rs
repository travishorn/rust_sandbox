pub fn run() {
    println!("Hello from the print.rs file.");

    // Basic formatting
    println!("{} is from {}.", "Mike", "New Jersey");

    // Positional arguments
    println!(
        "{0} is from {1} and {0} likes to {2}.",
        "Mike", "New Jersey", "play baseball"
    );

    // Named arguments
    println!(
        "{name} likes to {activity}.",
        name = "Mike",
        activity = "play baseball"
    );

    // Placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}
