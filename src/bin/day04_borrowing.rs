fn main() {
    let original_owner = String::from("Money");

    // The Fix: Use & to Borrow (Reference) instead of copying
    let new_owner = &original_owner; 

    println!("Original owner says: {}", original_owner);
    println!("New owner sees: {}", new_owner);
}