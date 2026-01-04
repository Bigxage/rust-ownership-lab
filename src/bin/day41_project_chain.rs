use sha2::{Sha256, Digest}; //hashing tools
use std::fmt::Write;        //formatting tools

#[derive(Debug, Clone)]
struct Block {
    id: u64,
    data: String,
    prev_hash: String,
    hash: String,
}

impl Block {
    //create a new block and calculate it's hash immediately
    fn new(id: u64, data: String, prev_hash: String) -> Self {
        let hash = calculate_hash(id, &data, &prev_hash);
        Block {
            id,
            data,
            prev_hash,
            hash,
        }
    }
}

//the "mining" function
fn calculate_hash(id: u64, data: &str, prev_hash: &str) -> String{
    let mut hasher = Sha256::new();
    //we combine all the data into one string
    let input = format!("{}{}{}", id, data, prev_hash);

    //we hash it
    hasher.update(input);
    let result = hasher.finalize();

    //convert generic bytes to a readable hex string (like "a1b2c3...")
    let mut hex_string = String::new();
    for byte in result {
        write!(&mut hex_string, "{:02x}", byte).unwrap();
    }
    hex_string
}

fn main() {
    //1. the genesis block (the first block has no previous hash)
    let genesis = Block::new(1, "Genesis Block".to_string(), "000000".to_string());

    let mut blockchain = vec![genesis];

    //2. Add new blocks
    let new_data = vec!["Rahim pays Inarah 5 sol", "Inarah pays client 2 sol"];

    for data in new_data {
        //grab the hash of the last block in the chain
        let last_block = blockchain.last().unwrap();
        let prev_hash = last_block.hash.clone();

        //create the new block linked to the old one
        let new_block = Block::new(last_block.id + 1, data.to_string(), prev_hash);

        blockchain.push(new_block);
    }

    //3. print the chain
    for block in blockchain {
        println!("\nBlock {}:", block.id);
        println!("data: {}", block.data);
        println!("prev: {}", block.prev_hash);
        println!("hash: {}", block.hash);
        println!("----------------------------");
    }
}