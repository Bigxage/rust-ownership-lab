fn main() {
    let transactions = vec![100, 250, 50, 1000];

    println!("Original: {:?}", transactions);

    // iterator chain
    // 1. .iter() -> create an iterator
    // 2. .map() -> apply a function to every item (double it)
    // 3. .collect() -> turn it back into a vector
    let processed: Vec<i32> = transactions.iter()
                                          .map(|x| x * 2)
                                          .collect();

    println!("processed (doubled): {:?}", processed);

    // check if any transaction is over 1000
    let huge_tx = processed.iter().any(|x| *x > 1000);
    println!("has huge transaction? {}", huge_tx);
}