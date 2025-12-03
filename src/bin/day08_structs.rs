fn main() {
    struct SolanaUser {
        username: String,
        balance: u64,
        is_active: bool,
    }
    let user1 = SolanaUser {
        username: String::from("bigxage"),
        balance: 1000,
        is_active: true,
    };
    println!("user: {}", user1.username);
    println!("balance: {} SOL", user1.balance);
    println!("active: {}", user1.is_active);

}