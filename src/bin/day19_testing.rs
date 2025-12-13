//production code to calculate 5% tax on a transaction
fn calculate_tax(amount: u64) -> u64 {
    amount * 5 / 100
}

fn send_sol(amount: u64) -> bool {
    if amount > 0 {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tax_calculation() {
        let income = 100;
        let expected_tax = 5;
        let actual_tax = calculate_tax(income);

        assert_eq!(actual_tax, expected_tax, "tax calculation failed!");
    }

    #[test]
    fn test_send_sol_success() {
        let result = send_sol(50);
        assert!(result, "sending 50 sol should return true");
    }

    #[test]
    fn test_send_sol_fail() {
        let result = send_sol(0);
        assert!(!result, "sending 0 sol should return false");
    }
}

fn main() {
    println!("to run tests, don't use 'cargo run'. use 'cargo test'.")
}