// src/domain/repository.rs

use crate::domain::transaction::Transaction;

pub trait TransactionRepository: Send + Sync {
    fn save(&self, tx: Transaction);
    fn get(&self, id: &str) -> Option<Transaction>;
}
