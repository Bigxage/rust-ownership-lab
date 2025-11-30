fn main() {
    let spaces = "   "; // Start as text
    println!("Original: '{}'", spaces);

    // Shadowing: We reuse the name 'spaces' to store a number
    let spaces = spaces.len(); 
    println!("Shadowed: {}", spaces);
}