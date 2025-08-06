// cargo run --example 01_tokio_event_dispatcher

// Why this architecture scales well:
// - Async backpressure through bounded channels
// - Natural separation of concerns (producers and consumers are decoupled)
// - Easy fault isolation — if one worker crashes, others can continue
// - No shared mutable state (safe concurrency)

// This example uses Tokio's mpsc (multi-producer, single-consumer) asynchronous channel.
// It allows multiple asynchronous tasks to send messages to the same receiver task.

use tokio::sync::mpsc;

// Define the type of messages that will flow through the channel.
//
// - Event(String): carries a payload (here a String)
// - Shutdown: signals the worker to stop and exit gracefully
enum Message {
    Event(String),
    Shutdown,
}

// This asynchronous function acts as the message consumer.
//
// It listens for incoming messages using `rx.recv().await` inside a loop.
// The loop stops either when a Shutdown message is received,
// or when the channel is closed (no more messages).
async fn start_worker(mut rx: mpsc::Receiver<Message>) {
    while let Some(msg) = rx.recv().await {
        match msg {
            Message::Event(data) => {
                println!("Worker received: {}", data);
            }
            Message::Shutdown => {
                println!("Worker shutting down.");
                break;
            }
        }
    }
}

// The #[tokio::main] macro initializes the async runtime and allows `main` to be async.
#[tokio::main]
async fn main() {
    // Create a channel with a buffer size of 100 messages.
    //
    // - tx: the sender side, used by producers to send messages
    // - rx: the receiver side, used by the consumer (worker)
    let (tx, rx) = mpsc::channel(100);

    // Spawn a new asynchronous task to run the worker.
    //
    // The worker will process messages received through the channel.
    tokio::spawn(start_worker(rx));

    // Send a message to the worker
    tx.send(Message::Event("hello".into())).await.unwrap();

    // Send the shutdown signal to stop the worker
    tx.send(Message::Shutdown).await.unwrap();
}
