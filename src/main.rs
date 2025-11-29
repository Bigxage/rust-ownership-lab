fn main() {
    let original_owner = String::from("Money");
    let new_owner = &original_owner;
    println!("{}", original_owner);
    println!("{}", new_owner)
}