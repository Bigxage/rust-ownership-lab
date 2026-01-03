use std::collections::HashMap;
use std::io;

//the blueprint for a single account
#[derive(Debug)]
struct Account {
    holder: String,
    balance: u64, //using u64 because money can't be negative!
}

//the 'database' that holds all accounts
struct Bank {
    //key: string (username), value: Account
    accounts: HashMap<String, Account>,
}

impl Bank {
    //constructor: opens a new empty bank
    fn new() -> Self {
        Bank {
            accounts: HashMap::new(),
        }
    }
            //1. create account
            fn add_account(&mut self, holder: String) {
                let account = Account {
                    holder: holder.clone(),
                    balance: 0,
                };
                self.accounts.insert(holder, account);
            }

            //2. deposit money
            //returns a result: ok(new_balance) or Err("account not found")
            fn deposit(&mut self, holder: &str, amount: u64) -> Result<u64, String> {
                match self.accounts.get_mut(holder) {
                    Some(account) => {
                        account.balance += amount;
                        Ok(account.balance)
                    }
                    None => Err("account not found!".to_string()),
                }
            }

            //3. withdraw money
            //critical rule: cannot withdraw more than balance
            fn withdraw(&mut self, holder: &str, amount: u64) -> Result<u64, String> {
                match self.accounts.get_mut(holder) {
                    Some(account) => {
                        if amount > account.balance {
                            return Err("insufficient funds!".to_string());
                        }
                        account.balance -= amount;
                        Ok(account.balance)
                    }
                    None => Err("account not found!".to_string()),
                }
            }

            //4. check balance
            fn balance(&self, holder: &str) -> Option<u64> {
                //we use .map to extract just the balance field if the account exists
                self.accounts.get(holder).map(|acc| acc.balance)
            }
}

        fn main() {
            let mut bank = Bank::new();

            //the 'game loop'
            loop{
                println!("\n--- RUST BANK SYSTEM ---");
                println!("1. Create Account");
                println!("2. Deposit");
                println!("3. Withdraw");
                println!("4. Check Balance");
                println!("5. Exit");
                println!("Enter Choice:");

                let mut choice = String::new();
                io::stdin().read_line(&mut choice).expect("failed to read");

                let choice = choice.trim();

                match choice {
                    "1" => {
                        println!("enter username:");
                        let mut name = String::new();
                        io::stdin().read_line(&mut name).unwrap();
                        bank.add_account(name.trim().to_string());
                        println!("Account created");
                    }
                    "2" => {
                        //hardcoded example for simplicity.
                        //in a real app, you'd ask for name and amount input like above.
                        println!("(demo) depositing 500 to 'rahim'...");
                        match bank.deposit("rahim", 500) {
                            Ok(bal) => println!("success! new balance: {}", bal),
                            Err(e) => println!("Error: {}", e),
                        }
                    }
                    "3" => {
                        println!("(demo) withdrawing 200 from 'rahim'...");
                        match bank.withdraw("rahim", 200) {
                            Ok(bal) => println!("success! new balance: {}", bal),
                            Err(e) => println!("Error: {}", e),
                        }
                    }
                    "4" => {
                        println!("enter username to check:");
                        let mut name = String::new();
                        io::stdin().read_line(&mut name).unwrap();
                        match bank.balance(name.trim()) {
                            Some(bal) => println!("balance: ${}", bal),
                            None => println!("user not found."),
                        }
                    }
                    "5" => break,
                    _ => println!("invalid choice!"),
                }
            }
        }