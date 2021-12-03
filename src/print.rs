pub fn run() {
    // Print to console
    println!("Hello World from the print.rs file!");

    // Basin formatting
    println!("{} from {}", "Algen", "CDO");

    // Positional arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Algen", "CDO", "code"
    );

    // Named arguments
    println!(
        "{name} likes to play {activity}",
        name = "Algen",
        activity = "Dota"
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic Math
    println!("10 + 10 = {}", 10 + 10);
}
