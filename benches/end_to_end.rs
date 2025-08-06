// benches/end_to_end.rs

use criterion::{Criterion, criterion_group, criterion_main};
use fraud_detection_3::domain::transaction::Transaction;
use fraud_detection_3::workers::dispatcher::process_transaction_bench;
use std::fs;

fn bench_end_to_end(c: &mut Criterion) {
    // Clean up files before each bench
    let _ = fs::remove_file("bench_trans.db");
    let _ = fs::remove_file("bench_score.db");

    c.bench_function("end_to_end_transaction", |b| {
        b.iter(|| {
            let tx = Transaction {
                id: format!("tx-{}", rand::random::<u64>()),
                amount: 100.0,
                currency: "USD".to_string(),
            };
            process_transaction_bench(tx);
        });
    });

    // Clean up files after bench
    let _ = fs::remove_file("bench_trans.db");
    let _ = fs::remove_file("bench_score.db");
}

criterion_group!(benches, bench_end_to_end);
criterion_main!(benches);
