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


## Real benchmarking with Criterion