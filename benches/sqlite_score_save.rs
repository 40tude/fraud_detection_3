// benches/sqlite_scoring_repo.rs

use criterion::{Criterion, criterion_group, criterion_main};
use fraud_detection_3::domain::repository::ScoreRepository;
use fraud_detection_3::domain::scoring::Score;
use fraud_detection_3::persistence::sqlite::SQLiteScoreRepo;
use std::sync::Arc;

fn bench_sqlite_scoring_result_save(c: &mut Criterion) {
    // Cleanup before the benchmark
    // let _ = fs::remove_file("bench_score_save.db");

    // Useful for benchmarking only SQL logic without disk
    // let repo = Arc::new(SQLiteScoreRepo::new(":memory:"));
    let repo = Arc::new(SQLiteScoreRepo::new("bench_score_save.db"));

    c.bench_function("sqlite_scoring_result_save", |b| {
        b.iter(|| {
            let result = Score {
                id: format!("tx-{}", rand::random::<u64>()),
                score: 0.77,
                is_fraud: false,
            };
            repo.save(result);
        });
    });

    // Cleanup after the benchmark
    // let _ = fs::remove_file("bench_score_save.db");
}

criterion_group!(benches, bench_sqlite_scoring_result_save);
criterion_main!(benches);
