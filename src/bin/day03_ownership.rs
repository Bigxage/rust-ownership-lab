fn main() {
    let original_owner = String::from("Money");

    // The Fix: Use .clone() to create a deep copy
    // Without clone, original_owner would be moved (destroyed)
    let new_owner = original_owner.clone(); 

    println!("Original owner says: {}", original_owner);
    println!("New owner says: {}", new_owner);
}