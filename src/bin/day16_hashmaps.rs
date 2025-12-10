use std::collections::HashMap;

fn main() {
    let mut balances: HashMap<String, u64> = HashMap::new();

    balances.insert("wallet_a".to_string(), 500);
    balances.insert("wallet_b".to_string(), 1500);

    let search_key = "wallet_b";

    let result = balances.get(search_key);

    match result {
        Some(amount) => println!("balance: {} sol", amount),
        None => println!("user not found!")
    }
}