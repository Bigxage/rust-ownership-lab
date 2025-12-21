# 🦀 The Rust Bible
> **Author:** bigxage  
> **Start Date:** Nov 25, 2025  
> **Mission:** 180 Days to Solana  

---

## 📚 Table of Contents
1. [Variables & Mutability](#1-variables--mutability-the-sealed-box)
2. [Strict Math (Type Casting)](#2-strict-math-type-casting)
3. [Ownership](#3-ownership-the-physical-object-rule)
4. [The Option Enum](#4-the-billion-dollar-fix-option-enum)
5. [Transaction Safety (Result)](#5-transaction-safety-result-enum)
6. [Structs & Impl](#6-the-real-estate-blueprint-structs--impl)
7. [Vectors](#7-vectors-the-dynamic-database)
8. [HashMaps](#8-hashmaps-the-fast-lookup)
9. [Traits](#9-traits-the-job-contract)
10. [Error Propagation (?)](#10-error-propagation-the--operator)
11. [Unit Testing](#11-unit-testing-the-guard-dog)
12. [Capstone Architecture](#12-the-capstone-architecture)
13. [Enums (Basic)](#13-enums-the-menu)
14. [Punctuation Logic](#14-punctuation-logic--vs--vs-nothing)
15. [Advanced Enums](#15-advanced-enums-enums-with-data)
16. [The Option Enum (Deep Dive)](#16-the-option-enum-the-null-killer)
17. [Generics](#17-generics-the-universal-placeholder)
18. [Generic Structs](#18-generic-structs-the-flexible-containers)

---

# PART ONE: BASICS

### 1. Variables & Mutability (The Sealed Box)
**Concept:** In Rust, variables are immutable (locked) by default. You must explicitly ask for permission to change them.

```rust
fn main() {
    let mut solana_balance = 100; // 1. Create a mutable variable
    solana_balance = 50;          // 2. Change the value
    println!("Balance: {}", solana_balance);
}
## 🔢 2. Strict Math (Type Casting)
**Concept:** Rust treats Integers (whole numbers) and Floats (decimals) as different species. You cannot add them together without manually converting one.

```rust
fn main() {
    let price = 150;      // Integer (i32)
    let fee = 0.5;        // Float (f64)
    
    // The Fix: Cast the Integer to a Float manually
    let total = (price as f64) + fee; 
}
##📦 3. Ownership (The Physical Object Rule)
Concept: Complex data (like Strings on the Heap) acts like a physical object. If you hand it to someone else, you don't have it anymore.
fn main() {
    let s1 = String::from("Money"); // 1. Create Heap String
    let s2 = s1;                    // 2. Move Ownership
    
    // println!("{}", s1); <--- CRASH! s1 is dead.
    println!("{}", s2); // s2 is the new owner.
}
The Consequence: Rust destroys s1 immediately to prevent two people from owning the same memory address. s1 is now invalid.

##❓ 4. The "Billion Dollar Fix" (Option Enum)
Concept: Rust handles missing data safely. A variable is never "null." It is a box that is either Some(data) or None.
fn get_id(user: i32) -> Option<String> {
    if user == 1 {
        return Some(String::from("Xage")); // Found it! Wrap it.
    } else {
        return None; // Found nothing.
    }
}

Some(...): "Here is the data, wrapped in the safety box."

None: "I am returning an empty box. This is not an error, just an absence of data."

##✅ 5. Transaction Safety (Result Enum)
Concept: For operations that can fail (like payments), we use Result. It returns Ok (Success) or Err (Failure Reason).
fn buy_land(balance: u64, price: u64) -> Result<String, String> {
    if balance >= price {
        return Ok(String::from("Success: Deed #101"));
    } else {
        return Err(String::from("Error: Insufficient Funds"));
    }
}

##🏗️ 6. The Real Estate Blueprint (Structs & Impl)
Concept: Structs let you define your own data types (Nouns). impl blocks let you give them logic (Verbs)
// 1. The Blueprint (Noun)
struct LandPlot {
    size: u64,
    price: u64,
    is_sold: bool,
}

// 2. The Logic (Verb)
impl LandPlot {
    // Factory: Creates a new plot
    fn new(size: u64, price: u64) -> LandPlot {
        return LandPlot {
            size: size,
            price: price,
            is_sold: false, // Default rule
        };
    }

    // Method: Calculates fee based on THIS plot's data
    fn calculate_fee(&self) -> u64 {
        self.price * 5 / 100
    }
}
&self: "I am referring to this specific plot of land."

##📊 7. Vectors (The Dynamic Database)
Concept: A Vector (Vec<T>) is a growable list that lives on the Heap. It allows you to store multiple items of the same type without knowing how many you will need in advance.
fn main() {
    let mut portfolio: Vec<LandPlot> = Vec::new();

    // Add Data (Push to Heap)
    portfolio.push(LandPlot { size: 1000, price: 50000 });
    portfolio.push(LandPlot { size: 500, price: 25000 });

    // The Loop (Analytics)
    let mut total_value = 0;
    
    // We iterate over '&portfolio' because we only want to READ it, not steal it.
    for plot in &portfolio {
        total_value += plot.price; 
    }

    println!("Total Value: {}", total_value);
}

##🗺️ 8. HashMaps (The Fast Lookup)
Concept: A Vector finds things by Index (0, 1, 2). A HashMap finds things by Key (Name, ID, Wallet Address). This is instant speed.
use std::collections::HashMap; // MUST IMPORT

fn main() { 
    // Key = String (Wallet Addy), Value = u64 (Balance) 
    let mut balances: HashMap<String, u64> = HashMap::new();

    balances.insert("Wallet_A".to_string(), 500); 
    
    // .get() returns an Option because the key might not exist
    match balances.get("Wallet_A") { 
        Some(amount) => println!("Balance: {} SOL", amount), 
        None => println!("User not found."), 
    }
}
##📜 9. Traits (The Job Contract)
Concept: A Struct is a Noun (Thing). A Trait is an Adjective (Behavior). If you want different structs to have the same function, you define a Trait.
// 1. The Contract
trait Summary {
    fn summarize(&self) -> String;
}

// 2. Signing the Contract
impl Summary for LandPlot {
    fn summarize(&self) -> String {
        return format!("{} sqm Plot selling for {} SOL", self.size, self.price);
    }
}
##🛑 10. Error Propagation (The ? Operator)
Concept: The ? operator is a shortcut. It says: "If this function returns Err, return that Error immediately. If Ok, give me the value inside."
fn batch_purchase(start_balance: u64) -> Result<u64, String> {
    // If buy_land fails, the function STOPS here and returns the error.
    let b1 = buy_land(start_balance, 500)?; 
    
    // We only reach here if step 1 succeeded!
    let final_b = buy_land(b1, 800)?;

    return Ok(final_b);
}
##🛡️ 11. Unit Testing (The Guard Dog)
Concept: Tests are small functions that check if your real functions are telling the truth.
#[cfg(test)]
mod tests {
    use super::*; // Bring real code into test scope

    #[test]
    fn test_addition() {
        let result = add(2, 2);
        assert_eq!(result, 4, "Math is broken!"); 
    }
}
cargo test: Runs all functions marked with #[test].

##🏛️ 12. The Capstone Architecture
Concept: A robust Rust program combines State, Logic, and Tests.

State: Structs + HashMaps (The Data)

Behavior: Traits (The Capabilities)

Safety: Result Types (Success/Failure handling)

Verification: #[test] modules.