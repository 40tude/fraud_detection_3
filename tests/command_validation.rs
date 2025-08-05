// tests/command_validation.rs

use fraud_detection_3::command_bus::dispatch;
use fraud_detection_3::commands::process_transaction::{ProcessTransaction, ProcessTransactionHandler};
use fraud_detection_3::domain::transaction::Transaction;

#[test]
fn test_process_transaction_command() {
    let tx = Transaction {
        id: "tx-001".to_string(),
        amount: 123.45,
        currency: "USD".to_string(),
    };

    let cmd = ProcessTransaction { transaction: tx };
    let handler = ProcessTransactionHandler;
    let result = dispatch(cmd, handler);

    assert!(result.contains("Transaction processed"));
}
