// benches/sqlite_trans_repo.rs

use criterion::{Criterion, criterion_group, criterion_main};
use fraud_detection_3::domain::repository::TransRepository;
use fraud_detection_3::domain::transaction::Transaction;
use fraud_detection_3::persistence::sqlite::SQLiteTransRepo;
use std::sync::Arc;

fn bench_sqlite_transaction_save(c: &mut Criterion) {
    // Cleanup before the benchmark
    // let _ = fs::remove_file("bench_trans_save.db");

    // Useful for benchmarking only SQL logic without disk
    // let repo = Arc::new(SQLiteTransRepo::new(":memory:"));
    let repo = Arc::new(SQLiteTransRepo::new("bench_trans_save.db"));

    c.bench_function("sqlite_transaction_save", |b| {
        b.iter(|| {
            let tx = Transaction {
                id: format!("tx-{}", rand::random::<u64>()),
                amount: 42.0,
                currency: "USD".to_string(),
            };
            repo.save(tx);
        });
    });

    // Cleanup after the benchmark
    // let _ = fs::remove_file("bench_trans_save.db");
}

criterion_group!(benches, bench_sqlite_transaction_save);
criterion_main!(benches);
