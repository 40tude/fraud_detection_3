use criterion::{Criterion, criterion_group, criterion_main};
use fraud_detection_3::domain::repository::{ScoreRepository, TransRepository};
use fraud_detection_3::domain::scoring::Score;
use fraud_detection_3::domain::transaction::Transaction;
use std::sync::Arc;
use std::time::Instant;
use tokio::runtime::Runtime;
use tokio::sync::{Mutex, mpsc}; // tokio Mutex is Send

#[derive(Clone)]
struct InMemRepo;
impl TransRepository for InMemRepo {
    fn save(&self, _tx: Transaction) {}
    fn get(&self, _id: &str) -> Option<Transaction> {
        None
    }
}
impl ScoreRepository for InMemRepo {
    fn save(&self, _result: Score) {}
    fn get(&self, _id: &str) -> Option<Score> {
        None
    }
}

async fn start_worker(rx: Arc<Mutex<mpsc::Receiver<Transaction>>>, tx_repo: Arc<dyn TransRepository + Send + Sync>, score_repo: Arc<dyn ScoreRepository + Send + Sync>) {
    loop {
        let maybe_tx = {
            let mut locked = rx.lock().await;
            locked.recv().await
        };

        match maybe_tx {
            Some(tx) => {
                tx_repo.save(tx.clone());
                let result = Score {
                    id: tx.id.clone(),
                    score: 1.0,
                    is_fraud: false,
                };
                score_repo.save(result);
            }
            None => break,
        }
    }
}

fn bench_max_throughput(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    const NUM_TX: usize = 10_000;
    const NUM_WORKERS: usize = 4;

    c.bench_function("max_throughput", |b| {
        b.to_async(&rt).iter(|| async {
            let (tx, rx) = mpsc::channel::<Transaction>(NUM_TX);
            let rx = Arc::new(Mutex::new(rx));
            let repo = Arc::new(InMemRepo);

            for _ in 0..NUM_WORKERS {
                let rx_clone = rx.clone();
                let tx_repo = repo.clone();
                let score_repo = repo.clone();
                tokio::spawn(start_worker(rx_clone, tx_repo, score_repo));
            }

            let start = Instant::now();

            for i in 0..NUM_TX {
                let tx_data = Transaction {
                    id: format!("tx-{}", i),
                    amount: 100.0,
                    currency: "USD".to_string(),
                };
                tx.send(tx_data).await.unwrap();
            }

            drop(tx); // Important to close the channel

            tokio::time::sleep(std::time::Duration::from_millis(100)).await;

            let duration = start.elapsed();
            println!("Processed {} transactions in {:?}", NUM_TX, duration);
            println!("Throughput ≈ {:.2} tx/sec", NUM_TX as f64 / duration.as_secs_f64());
        });
    });
}

criterion_group!(benches, bench_max_throughput);
criterion_main!(benches);
