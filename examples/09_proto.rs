// This file is a prototype of the fraud detection pipeline using:
// - Command Bus pattern
// - Channel-driven Actor Model (Tokio + async fn)
// - Mocked logic using println! for didactic purposes

use std::time::Duration;
use tokio::sync::mpsc;
use tokio::task;

#[derive(Debug, Clone)]
struct Transaction {
    id: usize,
}

#[derive(Debug)]
struct ScoringResult {
    id: usize,
    score: f64,
    is_fraud: bool,
}

#[derive(Debug)]
enum Command {
    Validate(Vec<Transaction>),
    Persist(Vec<Transaction>),
    Score(Vec<Transaction>),
    PersistScores(Vec<ScoringResult>),
}

// Type aliases for the Command Bus
type CommandSender = mpsc::Sender<Command>;
type CommandReceiver = mpsc::Receiver<Command>;

// Simulated transaction generator
async fn generator(bus: CommandSender) {
    let mut id = 0;
    loop {
        let batch: Vec<Transaction> = (0..5)
            .map(|_| {
                let tx = Transaction { id };
                id += 1;
                tx
            })
            .collect();

        println!("[Generator] Sending {} transactions for validation", batch.len());
        bus.send(Command::Validate(batch)).await.unwrap();
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}

// Agent 1: Validates a batch of transactions
async fn validation_agent(mut rx: CommandReceiver, bus: CommandSender) {
    while let Some(command) = rx.recv().await {
        if let Command::Validate(transactions) = command {
            println!("[Validation] Validating {} transactions", transactions.len());
            bus.send(Command::Persist(transactions)).await.unwrap();
        }
    }
}

// Agent 2: Persists validated transactions
async fn persistence_agent(mut rx: CommandReceiver, bus: CommandSender) {
    while let Some(command) = rx.recv().await {
        if let Command::Persist(transactions) = command {
            println!("[Persistence] Persisting {} transactions", transactions.len());
            bus.send(Command::Score(transactions)).await.unwrap();
        }
    }
}

// Agent 3: Scores the transactions
async fn scoring_agent(mut rx: CommandReceiver, bus: CommandSender) {
    while let Some(command) = rx.recv().await {
        if let Command::Score(transactions) = command {
            println!("[Scoring] Scoring {} transactions", transactions.len());
            let scores: Vec<ScoringResult> = transactions
                .into_iter()
                .map(|tx| {
                    let score = 0.9;
                    ScoringResult {
                        id: tx.id,
                        score,
                        is_fraud: score > 0.8,
                    }
                })
                .collect();

            bus.send(Command::PersistScores(scores)).await.unwrap();
        }
    }
}

// Agent 4: Persists scores
async fn scoring_persistence_agent(mut rx: CommandReceiver) {
    while let Some(command) = rx.recv().await {
        if let Command::PersistScores(scores) = command {
            println!("[Score Persistence] Saving {} scores", scores.len());
        }
    }
}

#[tokio::main]
async fn main() {
    // Central command bus
    let (bus_tx, mut bus_rx) = mpsc::channel::<Command>(100);

    // Channels for each agent
    let (tx_val, rx_val) = mpsc::channel(10);
    let (tx_persist, rx_persist) = mpsc::channel(10);
    let (tx_score, rx_score) = mpsc::channel(10);
    let (tx_score_persist, rx_score_persist) = mpsc::channel(10);

    // Launch agents
    task::spawn(validation_agent(rx_val, bus_tx.clone()));
    task::spawn(persistence_agent(rx_persist, bus_tx.clone()));
    task::spawn(scoring_agent(rx_score, bus_tx.clone()));
    task::spawn(scoring_persistence_agent(rx_score_persist));
    task::spawn(generator(bus_tx.clone()));

    // Command bus dispatcher loop
    while let Some(cmd) = bus_rx.recv().await {
        match &cmd {
            Command::Validate(_) => tx_val.send(cmd).await.unwrap(),
            Command::Persist(_) => tx_persist.send(cmd).await.unwrap(),
            Command::Score(_) => tx_score.send(cmd).await.unwrap(),
            Command::PersistScores(_) => tx_score_persist.send(cmd).await.unwrap(),
        }
    }
}
