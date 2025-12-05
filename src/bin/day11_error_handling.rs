fn get_balance(wallet_id: i32) -> Option<f64> {
    if wallet_id == 99 {
        return Some(150.50);
    } else {
        return None;
    }
}

fn main() {
    let id_to_check = 99;

    let result = get_balance(id_to_check);

    match result {
        Some(amount) => {
            println!("Wallet Found! Balance {} SOL", amount);
        },
        None => {
            println!("Error: Wallet ID {} not found.", id_to_check)
        }
    }
}