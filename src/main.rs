// src/main.rs

// They are moved to src/lib.rs
// mod command_bus;
// mod commands;
// mod domain;
// mod state_machine;

// use command_bus::dispatch;
// use commands::process_transaction::{ProcessTransaction, ProcessTransactionHandler};
// use state_machine::event::Event;
// use domain::fraud_scorer::{FraudScorer, RandomScorer};
// use domain::transaction::Transaction;
// use state_machine::state::{Enriched, /*Persisted,*/ State, Validated};

// Once the modules have been moved to src/lib.rs, add fraud_detection_3:: in order to access to the modules
use fraud_detection_3::command_bus::dispatch;
use fraud_detection_3::commands::process_transaction::{ProcessTransaction, ProcessTransactionHandler};
use fraud_detection_3::domain::fraud_scorer::{FraudScorer, RandomScorer};
use fraud_detection_3::domain::transaction::Transaction;
use fraud_detection_3::state_machine::event::Event;
use fraud_detection_3::state_machine::state::{Enriched, /*Persisted,*/ State, Validated};

// fn run_state_machine() {
//     let mut state: Box<dyn State> = Box::new(Validated);

//     loop {
//         let current_name = state.name();
//         let next = state.handle(Event::Process);

//         if current_name == next.name() {
//             println!("Final state: {}", next.name());
//             break;
//         }

//         state = next;
//     }
// }

fn run_state_machine(tx: &Transaction, scorer: &dyn FraudScorer) {
    let mut state: Box<dyn State> = Box::new(Validated);

    loop {
        // Store current state's name before moving it
        let current_name = state.name();

        state = match state.name() {
            "Validated" => state.handle(Event::Process),
            "Enriched" => {
                // let enriched = state.downcast::<Enriched>().expect("Expected Enriched state");
                let enriched = state.as_any().downcast::<Enriched>().expect("Expected Enriched state");
                enriched.handle_with_scorer(tx, scorer)
            }
            _ => state.handle(Event::Process),
        };

        if current_name == state.name() {
            println!("Final state: {}", state.name());
            break;
        }
    }
}

// fn main() {
//     println!("--- Command Dispatch Demo ---");
//     let tx = Transaction {
//         id: "tx-001".into(),
//         amount: 500.0,
//         currency: "USD".into(),
//     };

//     let cmd = ProcessTransaction { transaction: tx };
//     let handler = ProcessTransactionHandler;

//     let output = dispatch(cmd, handler);
//     println!("{output}");

//     println!("\n--- State Machine Demo ---");
//     run_state_machine();
// }

fn main() {
    println!("--- Command Dispatch Demo ---");
    let tx = Transaction {
        id: "tx-001".into(),
        amount: 500.0,
        currency: "USD".into(),
    };

    let cmd = ProcessTransaction { transaction: tx.clone() };
    let handler = ProcessTransactionHandler;

    let output = dispatch(cmd, handler);
    println!("{output}");

    println!("\n--- State Machine Demo ---");
    let scorer = RandomScorer { fraud_rate: 0.3 }; // 30% chance
    run_state_machine(&tx, &scorer);
}
