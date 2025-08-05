// examples/01_async.rs | async
use fraud_detection_3::{
    domain::transaction::Transaction,
    workers::dispatcher::{self, WorkerMessage},
};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    println!("Launching async worker demo...");

    let (tx, rx) = mpsc::channel(10);

    // Launch worker
    tokio::spawn(dispatcher::start_worker(rx));

    // Simulate sending transactions
    for i in 1..=5 {
        let tx_data = Transaction {
            id: format!("tx-{i:03}"),
            amount: 100.0 * i as f64,
            currency: "USD".to_string(),
        };
        tx.send(WorkerMessage::Transaction(tx_data)).await.unwrap();
    }

    // Send shutdown
    tx.send(WorkerMessage::Shutdown).await.unwrap();
}
