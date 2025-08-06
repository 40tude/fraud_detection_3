```
            +---------------------+
            |   Command Receiver  |
            +----------+----------+
                       |
          [ Command Bus (Dispatcher) ]
                       |
      +----------------+---------------+
      |                |               |
+-----------+   +--------------+   +--------------+
| Validation|   | State Machine|   | Persistence  |
+-----------+   +--------------+   +--------------+
                       |
                  [ Channel Bus ]
                       |
               +-----------------+
               | Worker Executors|
               +-----------------+

```





```
fraud_detection_3/
├── Cargo.toml
├── benches/
│   └── transaction_benchmark.rs       # Criterion-based benchmark
├── src/
│   ├── main.rs                        # App entry point
│   ├── command_bus.rs                 # Command, Handler traits + dispatcher
│   ├── commands/
│   │   ├── mod.rs
│   │   └── process_transaction.rs     # ProcessTransaction command + handler
│   ├── domain/
│   │   ├── mod.rs
│   │   ├── transaction.rs             # Core transaction struct
│   │   └── fraud_scorer.rs           # FraudScorer trait + mock implementations
│   ├── state_machine/
│   │   ├── mod.rs
│   │   ├── state.rs                   # State trait and concrete states
│   │   └── event.rs                   # Event enum
│   ├── persistence/
│   │   ├── mod.rs
│   │   └── in_memory.rs              # In-memory implementation of transaction repo
│   └── workers/
│       ├── mod.rs
│       └── dispatcher.rs             # Async worker dispatcher with round-robin logic
└── tests/
    └── command_validation.rs         # Unit tests for command input validation


```


```
cargo add rand

```


## Pour créer un worker asynchrone


cargo add tokio --features full

cargo run --example 00_sync
cargo run --example 01_async

cargo test --test command_validation



## Ajouter le log

```
cargo add tracing tracing-subscriber
# cargo add tracing-subscriber --features fmt,json
cargo add tracing-appender # tracing-appender handles rotation, not size limits

```

info!("App started");
debug!("Connecting to DB");
warn!("Retrying after failure");
error!("Could not reach service");


## Ajouter persistence
### In memory
Define the Trait (Interface) in the domain layer
See domain/repository.rs then persistence/in_memory.rs
Inject the Trait Object into Your App Logic. In your async worker or service:

1- Injecter InMemoryTransactionRepo dans le worker
Tu peux modifier dispatcher.rs pour qu’il accepte une implémentation du trait TransactionRepository 
Voir dispatcher.rs

2- Modifier la démo dans examples/02_fraud_detection_3al.rs
Injecte un Arc<InMemoryTransactionRepo> :
Probleme dans le code de la demo avec la main qui se termine avant que les logs aient pu être écrits



### SQLite
Créer une implémentation de TransactionRepository avec SQLite en backend, en gardant l'interface inchangée côté worker.
rusqlite est synchrone mais c'est OK ici
cargo add rusqlite --features bundled

Add src/persistence/sqlite.rs

Modifier src/persistence/mod.rs ajouter pub mod sqlite;

Utiliser https://sqliteviewer.app pour voir le contenu
Ou alors SQLite explorer/transaction/show table




## Mock ML model scorer (MlModelScorer via fake HTTP)

Créer un trait `MlModelScorer` avec une méthode `score(&self, tx: &Transaction) -> f32`, puis une implémentation simulée qui renvoie un score fixe ou pseudo-aléatoire.
Une nouvelle table dans la base

Modèle de données dans src/domain/scoring.rs
Trait de repository dans src/domain/repository.rs
Implémentation SQLite : src/persistence/sqlite/scoring_repo.rs

cargo add rand



On va maintenant modifier le worker pour qu’il :

Étapes prévues :
 Ajouter un "scoring dummy" dans dispatcher.rs
 Persister un ScoringResult
 Ajouter le repo dans la démo (05_sqlite.rs)



dispatcher.rs
Modifie start_worker pour :
simuler un score (aléatoire ici),
construire un ScoringResult,
le persister via un second repository.



## Real benchmarking with Criterion

Add Criterion to Cargo.toml
cargo add criterion --features html_reports
Z! section dev-dependencies

[dev-dependencies]
criterion = { version = "0.7.0", features = ["html_reports"] }

[[bench]]
name = "sqlite_trans_save"
harness = false

[[bench]]
name = "sqlite_score_save"
harness = false

[[bench]]
name = "end_to_end"
harness = false



Add benches/sqlite_trans_repo.rs 
Note: This will create and write to `./bench_trans_save.db`. If needed, add logic to remove it between runs.


Add benches/sqlite_scoring_repo.rs 
Note: This will create and write to `./bench_score_save.db`. If needed, add logic to remove it between runs.


cargo bench
cargo bench --bench sqlite_trans_save
cargo bench --bench sqlite_score_save
Le rapport est dans target\criterion\report\index.html


### Bench end to end
Dans dispatcher.rs , ajouter 
#[cfg(bench)]
process_transaction_bench()...

Cette version évite tokio et les mpsc pour isoler la logique du benchmark.

Si besoin un jour ajouter cfg-if
    [dependencies]
    cfg-if = "1.0"

    Puis 
    cfg_if::cfg_if! {
        if #[cfg(not(feature = "bench"))]{
            use crate::domain::repository::{ScoreRepository, TransRepository};
            use tokio::sync::mpsc::Receiver;
            use tracing::{info /* , debug*/}; // For score generation
        }
    }

Bien voir #[cfg(feature = "bench")] et #[cfg(not(feature = "bench"))] un peu partout

Créer benches/end_to_end.rs

Ajouter une feature "bench" a cargo.toml
[features]
bench = []

Ajouter 
[[bench]]
name = "end_to_end"
harness = false

cargo bench --features bench --bench end_to_end

Rapport dans /target/criterion
Par exemple
Mean: 19.289 ms per transaction
    This value includes:
    Saving the transaction to SQLite
    Generating a score
    Saving the score result
    Since it's not using any async, channels, or concurrency in the benchmark (process_transaction_bench), this is a good baseline for pure synchronous throughput.
    The std. dev. seems large (24.855 ms), so the performance varies a lot between runs. This could be due to disk I/O (especially on Windows) or internal SQLite locking.



### max throughput
To estimate theoretical max throughput, you want to strip down everything to the bare essentials and eliminate I/O bottlenecks.
Here’s a breakdown of what you suggested, plus a few more ideas:

Keep
    Use multiple async workers (to simulate concurrency realistically)
    Simulate message dispatching (via tokio::mpsc)
    A minimal scoring logic (e.g. is_fraud = false)
    In-memory storage only (no SQLite I/O)
    Time just the worker processing loop

Remove or minimize
    Score generation randomness → use a constant
    SQLite file/database → use an in-memory implementation of TransRepository and ScoreRepository
    Logging (tracing) → disable it or feature-gate it out
    Transaction construction randomness → create a single Transaction instance and clone it

This would give you the upper bound of your architecture's throughput assuming:
    No disk I/O
    No ML model
    No batch overhead
    It's an excellent benchmark to evaluate:
    The performance limit of your threading and async architecture
    How much headroom you have for additional logic (ML, logging, etc.)

Add benches/max_throughput.rs
    This uses 4 async workers. You can tweak NUM_WORKERS as needed.
    All data is stored in memory — no disk writes or random scoring.
    It measures total wall-clock time and computes the throughput.
    The println! output will appear in the terminal when you run: 

[[bench]]
name = "max_throughput"
harness = false

cargo bench --bench max_throughput