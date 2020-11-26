pub fn run() {
    // Print to console
    println!("Hello from the printrs file");

    // Basic Fromatting (To Replace Something)
    println!("Number: {}", 1);

    println! ("{} is from {}", "Brad", "Mass");

    // Positional Arguments
    println! ("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "Code");

    // Named Arguments

    println! ("{name} likes to play {activity}", name = "John", activity="baseball");

    // Plcaeholder Traits
    println! ("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder Fro Debug Traits / Touples

    println! ("{:?}", (12, true, "hello"));

    // Basic Maths
    println! ("10 + 10 = {}", 10+10);
}