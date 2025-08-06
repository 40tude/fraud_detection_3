// src/workers/dispatcher.rs

// This file is used in two compilation modes:
// - Default: the async worker is launched using Tokio and mpsc.
// - Bench: a stripped-down version of the processing logic is exposed for Criterion benchmarks,
//          activated with `--features bench` (for example : cargo bench --features bench --bench end_to_end)
//
// Use `#[cfg(not(feature = "bench"))]` for code only needed in normal runtime mode.
// Use `#[cfg(feature = "bench")]` for benchmark-specific code.

// Used in both runtime and bench mode → no cfg required
use crate::domain::repository::{ScoreRepository, TransRepository};
use crate::domain::scoring::Score;
use crate::domain::transaction::Transaction;
use rand::random;
use std::sync::Arc;

// Should I use a cfg_if::cfg_if! {...} block ?
#[cfg(not(feature = "bench"))]
use {
    tokio::sync::mpsc::Receiver,
    tracing::{info /* , debug*/},
};

#[cfg(feature = "bench")]
use crate::persistence::sqlite::{SQLiteScoreRepo, SQLiteTransRepo};

#[derive(Debug)]
pub enum WorkerMessage {
    Transaction(Transaction),
    Shutdown,
}

// Updated start_worker
#[cfg(not(feature = "bench"))]
pub async fn start_worker<TR: TransRepository + Send + Sync + 'static, SR: ScoreRepository + Send + Sync + 'static>(mut rx: Receiver<WorkerMessage>, tx_repo: Arc<TR>, score_repo: Arc<SR>) {
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
                let result = Score { id: tx.id.clone(), score, is_fraud };

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

// This version avoids tokio and mpsc to isolate the benchmark logic.
#[cfg(feature = "bench")]
pub fn process_transaction_bench(tx: Transaction) {
    let trans_repo = Arc::new(SQLiteTransRepo::new("bench_trans.db"));
    let score_repo = Arc::new(SQLiteScoreRepo::new("bench_score.db"));

    // Here we simulate the main processing logic from the worker
    // let score = rand::random::<f64>();
    let score: f64 = random(); // value in [0.0, 1.0)
    let is_fraud = score > 0.8;
    let result = Score { id: tx.id.clone(), score, is_fraud };

    trans_repo.save(tx);
    score_repo.save(result);
}
