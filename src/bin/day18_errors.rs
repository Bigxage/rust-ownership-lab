fn buy_land(balance: u64, price: u64) -> Result<u64, String> {
    if balance >= price {
        return Ok(balance - price);
    } else {
        return Err("insufficient funds".to_string());
    }
}

fn batch_purchase(start_balance: u64) -> Result<u64, String> {
    println!("attempting plot 1...");

    let b1 = buy_land(start_balance, 500)?;

    println!("attempting plot 2...");

    let final_b = buy_land(b1, 800)?;

    return Ok(final_b);
}

fn main() {
    let my_cash = 1000;

    match batch_purchase(my_cash) {
        Ok(cash) => println!("success! remaining: {}", cash),
        Err(e) => println!("transaction failed: {}", e),
    }
}