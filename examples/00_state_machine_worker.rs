// 3 Independent State Machines acting as a single asynchronous one
//
// Each call to tokio::spawn(state_machine(...)) creates a separate instance of the state machine.
// Each FSM manages its own transaction in isolation.
// Non-Blocking Wait:
//      When an FSM reaches score_rx.await, it suspends its execution (but does not block the thread).
//      The other FSMs continue to run in parallel thanks to Tokio.
//
// The Message is “Misleading”:
//      println!(“[SM] Transaction {}: Waiting for score... (but I'm continuing to process other transactions!)”, tx_id);
//      This line is displayed before the blocking call (score_rx.await).
//      In reality, the SM does not process anything while waiting, but other SMs (on other transactions) continue to progress.

use std::time::Duration;
use tokio::sync::{mpsc, oneshot};

// ------ Type Definitions ------
#[derive(Debug)]
struct Transaction {
    id: u64,
    _amount: f64,
}

enum FraudTask {
    MlScoring(Transaction, oneshot::Sender<f64>), // Transaction + return channel
}

// ------ State Machine ------
// Simulates a state machine that evaluates transactions and delegates ML to Workers.
async fn state_machine(tx: Transaction, worker_tx: mpsc::Sender<FraudTask>) {
    let tx_id = tx.id; // Capture ID before move in worker_tx.send()

    println!("[SM] Transaction {}: Starting evaluation", tx_id);

    // Create a one-shot channel to receive Worker's result
    let (score_tx, score_rx) = oneshot::channel();

    // Send task to Worker (non-blocking)
    worker_tx.send(FraudTask::MlScoring(tx, score_tx)).await.unwrap();

    // While Worker processes, SM can do other things!
    println!("[SM] Transaction {}: Waiting for score... (but I can keep processing other transactions!)", tx_id);

    // Wait for score (async behavior, doesn't block thread)
    match score_rx.await {
        Ok(score) => {
            println!("[SM] Transaction {}: Received score = {:.2}", tx_id, score);
            if score > 0.8 {
                println!("🚨 Fraud Alert!");
            } else {
                println!("✅ Clean Transaction");
            }
        }
        Err(_) => eprintln!("[SM] Error: Worker died?"),
    }
}

// ------ Worker ------
// Simulates an async Worker calling an ML model.
async fn worker(mut rx: mpsc::Receiver<FraudTask>) {
    while let Some(task) = rx.recv().await {
        match task {
            FraudTask::MlScoring(tx, reply) => {
                println!("[Worker] Processing transaction {}...", tx.id);
                // Simulate slow network call (2 seconds)
                tokio::time::sleep(Duration::from_secs(2)).await;
                // Simulate random ML score
                let score = 0.9; // In reality: ml_model.predict(&tx).await;
                println!("[Worker] Calculated ML score: {:.2}", score);
                // Send score back to SM
                reply.send(score).unwrap();
            }
        }
    }
}

// ------ Main ------
#[tokio::main]
async fn main() {
    // Channel to send tasks to Workers
    let (worker_tx, worker_rx) = mpsc::channel(10);

    // Launch Worker in separate thread
    tokio::spawn(worker(worker_rx));

    // Simulate 3 transactions sent to SM
    for id in 1..=3 {
        let tx = Transaction { id, _amount: 100.0 * id as f64 };
        // Each state_machine call is non-blocking
        tokio::spawn(state_machine(tx, worker_tx.clone()));
    }

    // Wait for everything to process (for example purposes)
    tokio::time::sleep(Duration::from_secs(4)).await;
}
