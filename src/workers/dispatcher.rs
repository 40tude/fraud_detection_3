// src/workers/dispatcher.rs

use crate::domain::repository::{ScoringResultRepository, TransactionRepository};
use crate::domain::scoring::ScoringResult;
use crate::domain::transaction::Transaction;

use std::sync::Arc;
use tokio::sync::mpsc::Receiver;
use tracing::{info /* , debug*/};

// use rand::Rng; // For score generation
use rand::random;

#[derive(Debug)]
pub enum WorkerMessage {
    Transaction(Transaction),
    Shutdown,
}

// Updated start_worker
pub async fn start_worker<TR: TransactionRepository + Send + Sync + 'static, SR: ScoringResultRepository + Send + Sync + 'static>(
    mut rx: Receiver<WorkerMessage>,
    tx_repo: Arc<TR>,
    score_repo: Arc<SR>,
) {
    while let Some(msg) = rx.recv().await {
        match msg {
            WorkerMessage::Transaction(tx) => {
                info!(tx_id = %tx.id, "Processing transaction");

                // Save transaction to DB
                tx_repo.save(tx.clone());
                info!(tx_id = %tx.id, "Transaction saved");

                // Retrieve it back
                if let Some(saved_tx) = tx_repo.get(&tx.id) {
                    info!(?saved_tx, "Transaction persisted");
                }

                // Generate a dummy score
                // let mut rng = rand::thread_rng();
                let score: f64 = random(); // value in [0.0, 1.0)
                let is_fraud = score > 0.8;

                // Build and persist scoring result
                let result = ScoringResult { id: tx.id.clone(), score, is_fraud };

                score_repo.save(result.clone());
                info!(?result, "Scoring result saved");
            }

            WorkerMessage::Shutdown => {
                info!("Worker shutting down.");
                break;
            }
        }
    }
}
