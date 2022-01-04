pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    // Basic formatting
    println!("{} is from: {}", "Chen", "Mass");

    // Positional args
    println!("{0} is from {1} and {0} likes to {2}", "Chen", "Mass", "code");

    // named args
    println!("{name} likes to play {activity}", name = "Chen", activity = "basketball");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}