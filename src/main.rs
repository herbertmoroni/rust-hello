fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    
    //1. Basic println! macro
    println!("Hello, world!");

    // 2. Variables are immutable by default in Rust
    let language = "Rust";
    println!("This is written in {}.", language);

    // 3. Calling a function that returns an owned String
    let message = greet("Moroni");
    println!("{}", message);

    // 4. A simple loop with a counter
    println!("\nCounting to 3 (because Rust owns its loops too):");
    for i in 1..=3 {
        println!("  Step {}: still safe, still fast.", i);
    } 

    // 5. Pattern matching - one of Rust's most powerful features
    let experience_years: u32 = 17;
    let level = match experience_years {
        0..=2   => "Beginner",
        3..=9   => "Intermediate",
        10..=19 => "Senior",
        _       => "Veteran",
    };
    println!("\nWith {} years of experience, level: {}.", experience_years, level);

    println!("\nOwnership, safety, and speed — that's Rust.");

}
