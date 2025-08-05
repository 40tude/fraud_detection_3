// src/workers/dispatcher.rs
// Define the async worker logic:

use crate::domain::transaction::Transaction;

// use tokio::sync::mpsc;

#[derive(Debug)]
pub enum WorkerMessage {
    Transaction(Transaction),
    Shutdown,
}

// pub async fn start_worker(mut rx: mpsc::Receiver<WorkerMessage>) {
//     while let Some(msg) = rx.recv().await {
//         match msg {
//             WorkerMessage::Transaction(tx) => {
//                 println!("[Worker] Processing transaction: {:?}", tx);
//                 // Future steps: run command bus, state machine, etc.
//             }
//             WorkerMessage::Shutdown => {
//                 println!("[Worker] Shutting down.");
//                 break;
//             }
//         }
//     }
// }

use crate::domain::repository::TransactionRepository;

use std::sync::Arc;
use tokio::sync::mpsc::Receiver;
use tracing::info;

pub async fn start_worker<R: TransactionRepository + Send + Sync + 'static>(mut rx: Receiver<WorkerMessage>, repo: Arc<R>) {
    while let Some(msg) = rx.recv().await {
        match msg {
            WorkerMessage::Transaction(tx) => {
                // Log the transaction
                info!(tx_id = %tx.id, "Processing transaction");

                // Persist transaction
                repo.save(tx.clone());
                info!(tx_id = %tx.id, "Transaction saved");

                // Optionally: retrieve it back
                // if let Some(saved_tx) = repo.get_by_id(&tx.id).await {
                // if let Some(saved_tx) = repo.as_ref().get_by_id(&tx.id) {
                if let Some(saved_tx) = repo.get(&tx.id) {
                    info!(?saved_tx, "Transaction persisted");
                }
            }
            WorkerMessage::Shutdown => {
                info!("Worker shutting down.");
                break;
            }
        }
    }
}
