fn purchase_land(balance: u64, price: u64) -> Result<String, String> {
    if balance >= price {
        return Ok(String::from("Land Deed #1055 issued"))
    } else {
        return Err(String::from("Error: Insufficient funds. Transaction cancelled"))
    } 
}

fn main() {
    let my_balance = 100;
    let price = 150;
    let outcome = purchase_land(my_balance, price);

    match outcome {
        Ok(data) => println!("Success: {}", data),
        Err(reason) => println!("Failure: {}", reason),
    }
}