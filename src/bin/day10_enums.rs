fn main() {
   let status = TransactonStatus::Pending;

   match status {
    TransactonStatus::Pending => println!("Transaction pending"),
    TransactonStatus::Success => println!("Transaction successful"),
    TransactonStatus::Failed => println!("Transaction failed"),
   }
}
enum TransactonStatus {
    Pending,
    Success,
    Failed,
}