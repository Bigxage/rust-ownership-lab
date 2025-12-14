use std::collections::HashMap;

trait Mintable {
    fn mint_to(&mut self, receiver: String, amount: u64) -> Result<u64, String>;
}

struct TokenMint {
    max_supply: u64,
    total_minted: u64,
    balances: HashMap<String, u64>,
}

impl TokenMint {
    fn new(max: u64) -> Self {
        TokenMint{
            max_supply: max,
            total_minted: 0,
            balances: HashMap::new(),
        }
    }
}

impl Mintable for TokenMint {
    fn mint_to(&mut self, receiver: String, amount: u64) -> Result<u64, String> {
        if self.total_minted + amount > self.max_supply {
            return Err("minting would exceed max supply".to_string());
        }
        self.total_minted += amount;

        let user_balance = self.balances.entry(receiver).or_insert(0);
        *user_balance += amount;

        return Ok(self.total_minted);
    }
}

fn main() {
    let mut bonk = TokenMint::new(1000);

    match bonk.mint_to("Alice".to_string(), 100) {
        Ok(total) => println!("Success! Total Minted: {}", total),
        Err(e) => println!("Error: {}", e),
    }

    match bonk.mint_to("Bob".to_string(), 2000) {
        Ok(total) => println!("Success! Total Minted: {}", total),
        Err(e) => println!("Error: {}", e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_successful_mint() {
        let mut mint = TokenMint::new(1000);
        let result = mint.mint_to("Alice".to_string(), 100);
        assert!(result.is_ok());
        assert_eq!(mint.total_minted, 100);
    }

    #[test]
    fn test_supply_overflow() {
        let mut mint = TokenMint::new(100);
        let _ = mint.mint_to("Alice".to_string(), 50);
        let result = mint.mint_to("Bob".to_string(), 51);

        assert!(result.is_err());
    }
}