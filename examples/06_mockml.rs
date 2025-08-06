// examples/03_mem.rs | async + log + SQLite persistence
use fraud_detection_3::persistence::sqlite::SQLiteScoringResultRepo;
use fraud_detection_3::{
    domain::transaction::Transaction,
    workers::dispatcher::{self, WorkerMessage},
};

use tokio::sync::mpsc;

// For tracing
use tracing::{info, warn};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{Layer, Registry, fmt, layer::SubscriberExt, util::SubscriberInitExt};

// For persistence
// use fraud_detection_3::persistence::in_memory::InMemoryTransactionRepo;
use fraud_detection_3::persistence::sqlite::SQLiteTransactionRepo;

fn init_logging() -> tracing_appender::non_blocking::WorkerGuard {
    // Create a daily rotating file appender in ./logs/
    let file_appender = RollingFileAppender::new(Rotation::DAILY, "logs", "app.log");

    // Optional: make it non-blocking
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let file_layer = fmt::layer()
        .with_ansi(false) // no ANSI color codes in log file
        .with_writer(non_blocking)
        .with_filter(tracing::level_filters::LevelFilter::DEBUG); // File: keep all logs (incl. debug)

    let stdout_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .with_ansi(true) // keep ANSI for console
        .with_filter(tracing::level_filters::LevelFilter::INFO); // Console: info and above

    // Combine layers with explicit subscriber registry
    Registry::default().with(stdout_layer).with(file_layer).init();

    // Build subscriber with formatting to both stdout and file
    // tracing_subscriber::registry()
    //     .with(fmt::layer().with_writer(std::io::stdout)) // log to console
    //     .with(fmt::layer().with_writer(non_blocking)) // log to file
    //     .init();

    // Important: keep _guard alive to flush logs properly
    // (store it globally or in main)
    guard // Return the guard
}

// #[tokio::main]
// async fn main() {
//     let _guard = init_logging(); // Initializes logging to stdout and log file
//     // println!("Launching async worker demo...");
//     info!("Launching async worker demo...");
//     warn!("This is a warning");

//     let (tx, rx) = mpsc::channel(10);

//     // Launch worker
//     tokio::spawn(dispatcher::start_worker(rx));

//     // Simulate sending transactions
//     for i in 1..=5 {
//         let tx_data = Transaction {
//             id: format!("tx-{i:03}"),
//             amount: 100.0 * i as f64,
//             currency: "USD".to_string(),
//         };
//         info!(tx_id = %tx_data.id, amount = tx_data.amount, "Processing transaction");
//         tx.send(WorkerMessage::Transaction(tx_data)).await.unwrap();
//     }

//     // Send shutdown
//     tx.send(WorkerMessage::Shutdown).await.unwrap();
//     info!("Shuting down...");
// }

use std::sync::Arc;
#[tokio::main]
async fn main() {
    // The worker is asynchronous and spawned in a separate task. However, as soon as main() ends, the guard variable is dropped, and therefore the non-blocking writer is released.
    // Consequence: all worker logs are lost if they arrive after the guard is dropped.

    let guard = init_logging(); // Initializes logging to stdout and log file

    info!("Launching async worker demo...");
    warn!("This is a warning");

    let tx_repo = Arc::new(SQLiteTransactionRepo::new("data.db"));
    let score_repo = Arc::new(SQLiteScoringResultRepo::new("data.db"));

    let (tx, rx) = mpsc::channel(10);

    // In order to make sure logs are also written to file
    // Keeps logs alive as long as the worker is running.
    // Ensures that all logs are flushed properly to the file.
    let worker_handle = tokio::spawn(dispatcher::start_worker(rx, tx_repo.clone(), score_repo.clone()));

    for i in 1..=5 {
        let tx_data = Transaction {
            id: format!("tx-{i:03}"),
            amount: 100.0 * i as f64,
            currency: "USD".to_string(),
        };
        tx.send(WorkerMessage::Transaction(tx_data)).await.unwrap();
    }

    tx.send(WorkerMessage::Shutdown).await.unwrap();

    // Wait for worker to finish before dropping guard
    worker_handle.await.unwrap();

    info!("Shutting down...");

    // `guard` will be dropped here, after the logs have been flushed
    drop(guard);
}
