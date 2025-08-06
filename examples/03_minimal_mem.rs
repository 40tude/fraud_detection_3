// examples/03_repo.rs

use fraud_detection_3::domain::repository::TransRepository;
use fraud_detection_3::domain::transaction::Transaction;
use fraud_detection_3::persistence::in_memory::InMemoryTransactionRepo;
use std::sync::Arc;

fn main() {
    let repo: Arc<dyn TransRepository> = Arc::new(InMemoryTransactionRepo::new());

    let tx = Transaction {
        id: "tx-001".to_string(),
        amount: 123.45,
        currency: "EUR".to_string(),
    };

    repo.save(tx.clone());

    if let Some(found) = repo.get("tx-001") {
        println!("Transaction found: {:?}", found);
    } else {
        println!("Transaction not found");
    }
}
