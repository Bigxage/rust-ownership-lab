fn main() {
    let solana_price = 150;     // Integer (i32)
    let gas_fee = 0.5;          // Float (f64)

    // The Fix: Explicitly cast the integer to a float using 'as f64'
    let total_cost = (solana_price as f64) + gas_fee; 
    
    println!("Total cost: {}", total_cost);
}