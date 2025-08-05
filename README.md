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