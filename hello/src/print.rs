pub fn run() {
    // Print to console
    println!("Hello, World!");

    // Basic Formatting
    println!("Hello {}!", "there");
    
    // Positional Arguments
    println!("Bhanu likes to {0} and he can {0} in {1} languages.", "code", 3);
    
    // Named Arguments
    println!("{name} is from {city}.", name="Bhanu", city="Indore");

    // Placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Oct: {:o}", 10, 10, 10);
    
    // Placeholder for debug traits
    println!("{:?}", (12, true, "hello"));

    // Basic Operations
    println!("10 + 10 = {}", 10+10)
    
}