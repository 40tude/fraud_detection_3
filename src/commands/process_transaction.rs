// src/commands/process_transaction.rs

use crate::command_bus::{Command, Handler};
use crate::domain::transaction::Transaction;

pub struct ProcessTransaction {
    pub transaction: Transaction,
}

impl Command for ProcessTransaction {
    type Output = String; // For now, just a status message
}

pub struct ProcessTransactionHandler;

impl Handler<ProcessTransaction> for ProcessTransactionHandler {
    fn handle(&self, cmd: ProcessTransaction) -> String {
        let tx = cmd.transaction;
        format!("Transaction processed: amount = {}, currency = {}", tx.amount, tx.currency)
    }
}
